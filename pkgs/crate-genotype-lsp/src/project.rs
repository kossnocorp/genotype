use crate::document::{Document, Name, error_diagnostic, offset};
use async_lsp::lsp_types::{Diagnostic, Location, Position, Range, TextEdit, Url, WorkspaceEdit};
use std::collections::{HashMap, HashSet};
use std::path::{self as path, Path, PathBuf};

pub mod generated;
mod intelligence;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Symbol(Url, usize);

#[derive(Clone)]
pub struct Project {
    pub documents: HashMap<Url, Document>,
}

impl Project {
    pub fn diagnostics(&self, uri: &Url) -> Vec<Diagnostic> {
        let Some(doc) = self.documents.get(uri) else {
            return vec![];
        };
        if !doc.valid {
            return doc.diagnostics.clone();
        }
        let mut diagnostics = vec![];
        for import in &doc.imports {
            let Some(target) = module_uri(uri, &import.path) else {
                continue;
            };
            if !self.documents.contains_key(&target) {
                let diagnostic = error_diagnostic(
                    doc.range(&import.path_span),
                    "module-not-found",
                    format!("Module not found: {}", import.path.trim_end_matches('/')),
                );
                if !diagnostics.contains(&diagnostic) {
                    diagnostics.push(diagnostic);
                }
            } else if let Some(name) = &import.name
                && self.exported(&target, &name.text).is_none()
                && !self.export_unavailable(&target)
            {
                diagnostics.push(error_diagnostic(
                    doc.range(&name.span),
                    "reference-not-found",
                    format!(
                        "Reference '{}' not found in {}",
                        name.text,
                        import.path.trim_end_matches('/')
                    ),
                ));
            }
        }
        for reference in &doc.references {
            if reference.generic.is_some() {
                continue;
            }
            let target = match &reference.path {
                Some(path) => match module_uri(uri, path) {
                    Some(target) => target,
                    None => continue,
                },
                None => uri.clone(),
            };
            let resolved = if reference.path.is_some() {
                self.exported(&target, &reference.name.text)
            } else {
                self.resolve(&target, &reference.name.text, &mut HashSet::new())
            };
            if resolved.is_none()
                && !(if reference.path.is_some() {
                    self.export_unavailable(&target)
                } else {
                    self.unavailable(&target, &reference.name.text)
                })
            {
                diagnostics.push(error_diagnostic(
                    doc.range(&reference.name.span),
                    "reference-not-found",
                    format!("Reference '{}' not found", reference.name.text),
                ));
            }
        }
        diagnostics.sort_by_key(|d| (d.range.start.line, d.range.start.character));
        diagnostics
    }

    // Do not invent missing-reference errors when a dependency cannot be
    // analyzed (syntax errors or external packages without source).
    fn export_unavailable(&self, uri: &Url) -> bool {
        self.documents.get(uri).is_some_and(|doc| !doc.valid)
    }

    fn unavailable(&self, uri: &Url, name: &str) -> bool {
        let Some(doc) = self.documents.get(uri) else {
            return false;
        };
        if !doc.valid {
            return true;
        }
        doc.imports
            .iter()
            .filter(|import| {
                import
                    .alias
                    .as_ref()
                    .or(import.name.as_ref())
                    .is_none_or(|binding| binding.text == name)
            })
            .any(|import| match module_uri(uri, &import.path) {
                None => true,
                Some(target) => self.export_unavailable(&target),
            })
    }

    pub fn load(roots: &[PathBuf], open: &HashMap<Url, String>) -> Self {
        let mut documents = HashMap::new();
        for root in roots {
            let entries = Entries::new(root);
            for path in entries.files() {
                if path.is_file()
                    && let (Ok(uri), Ok(text)) =
                        (Url::from_file_path(&path), std::fs::read_to_string(&path))
                {
                    documents.insert(uri, Document::new(text));
                }
            }
        }
        for (uri, text) in open {
            if uri
                .to_file_path()
                .is_ok_and(|path| roots.iter().any(|root| path.starts_with(root)))
            {
                documents.insert(uri.clone(), Document::new(text.clone()));
            }
        }
        let mut project = Self { documents };
        project.follow_imports(open);
        project
    }

    pub fn update(&mut self, uri: &Url, open: &HashMap<Url, String>) {
        let text = open
            .get(uri)
            .cloned()
            .or_else(|| std::fs::read_to_string(uri.to_file_path().ok()?).ok());
        if let Some(text) = text {
            if self.documents.get(uri).is_none_or(|doc| doc.text != text) {
                self.documents.insert(uri.clone(), Document::new(text));
            }
        } else {
            self.documents.remove(uri);
        }
        self.follow_imports(open);
    }

    pub fn references_module(&self, target: &Url) -> bool {
        self.documents.iter().any(|(uri, doc)| {
            module_paths(doc).any(|path| module_uri(uri, path).as_ref() == Some(target))
        })
    }

    // Workspace string-glob watchers cannot report changes outside their roots.
    // Revalidate only those dependencies, including missing import targets.
    pub fn refresh_external(&mut self, workspace: &[PathBuf], open: &HashMap<Url, String>) -> bool {
        let candidates: HashSet<_> =
            self.documents
                .keys()
                .cloned()
                .chain(self.documents.iter().flat_map(|(uri, doc)| {
                    module_paths(doc).filter_map(|path| module_uri(uri, path))
                }))
                .collect();
        let mut changed = false;
        for uri in candidates {
            let Ok(path) = uri.to_file_path() else {
                continue;
            };
            if open.contains_key(&uri) || workspace.iter().any(|root| path.starts_with(root)) {
                continue;
            }
            let text = std::fs::read_to_string(path).ok();
            if self.documents.get(&uri).map(|doc| &doc.text) != text.as_ref() {
                if let Some(text) = text {
                    self.documents.insert(uri, Document::new(text));
                } else {
                    self.documents.remove(&uri);
                }
                changed = true;
            }
        }
        if changed {
            self.follow_imports(open);
        }
        changed
    }

    pub fn prune(&mut self, roots: &[PathBuf], open: &HashMap<Url, String>) {
        let entries: Vec<_> = roots.iter().map(|root| Entries::new(root)).collect();
        let mut reachable: HashSet<_> = self
            .documents
            .keys()
            .filter(|uri| {
                uri.to_file_path().is_ok_and(|path| {
                    entries.iter().any(|entry| entry.contains(&path))
                        || (open.contains_key(*uri)
                            && roots.iter().any(|root| path.starts_with(root)))
                })
            })
            .cloned()
            .collect();
        let mut pending: Vec<_> = reachable.iter().cloned().collect();
        while let Some(uri) = pending.pop() {
            if let Some(doc) = self.documents.get(&uri) {
                for path in module_paths(doc) {
                    if let Some(target) = module_uri(&uri, path)
                        && reachable.insert(target.clone())
                    {
                        pending.push(target);
                    }
                }
            }
        }
        self.documents.retain(|uri, _| reachable.contains(uri));
    }

    fn follow_imports(&mut self, open: &HashMap<Url, String>) {
        let documents = &mut self.documents;
        // Follow imports outside workspace roots as well (including unopened files).
        loop {
            let mut pending = HashSet::new();
            for (uri, doc) in documents.iter() {
                for path in module_paths(doc) {
                    if let Some(target) = module_uri(uri, path)
                        && !documents.contains_key(&target)
                    {
                        pending.insert(target);
                    }
                }
            }
            let mut added = false;
            for uri in pending {
                if let Some(text) = open
                    .get(&uri)
                    .cloned()
                    .or_else(|| std::fs::read_to_string(uri.to_file_path().ok()?).ok())
                {
                    documents.insert(uri, Document::new(text));
                    added = true;
                }
            }
            if !added {
                break;
            }
        }
    }

    fn exported(&self, uri: &Url, name: &str) -> Option<Symbol> {
        let definition = self
            .documents
            .get(uri)?
            .definitions
            .iter()
            .find(|def| def.text == name)?;
        Some(Symbol(uri.clone(), definition.span.start))
    }

    fn resolve(
        &self,
        uri: &Url,
        name: &str,
        visited: &mut HashSet<(Url, String)>,
    ) -> Option<Symbol> {
        if !visited.insert((uri.clone(), name.into())) {
            return None;
        }
        let doc = self.documents.get(uri)?;
        if let Some(def) = doc.definitions.iter().find(|def| def.text == name) {
            return Some(Symbol(uri.clone(), def.span.start));
        }
        for import in &doc.imports {
            if let Some(alias) = &import.alias {
                if alias.text == name {
                    return Some(Symbol(uri.clone(), alias.span.start));
                }
            } else if let Some(original) = &import.name
                && original.text == name
            {
                return self.exported(&module_uri(uri, &import.path)?, name);
            }
        }
        let mut found = None;
        for import in doc.imports.iter().filter(|i| i.name.is_none()) {
            if let Some(target) = module_uri(uri, &import.path)
                && let Some(symbol) = self.exported(&target, name)
            {
                if found.as_ref().is_some_and(|other| other != &symbol) {
                    return None;
                }
                found = Some(symbol);
            }
        }
        found
    }

    fn occurrences(&self, uri: &Url) -> Vec<(&Name, Symbol)> {
        let Some(doc) = self.documents.get(uri) else {
            return vec![];
        };
        let mut result: Vec<_> = doc
            .definitions
            .iter()
            .chain(&doc.generics)
            .map(|name| (name, Symbol(uri.clone(), name.span.start)))
            .collect();
        for import in &doc.imports {
            if let Some(name) = &import.name
                && let Some(target) = module_uri(uri, &import.path)
                && let Some(symbol) = self.exported(&target, &name.text)
            {
                result.push((name, symbol));
            }
            if let Some(alias) = &import.alias {
                result.push((alias, Symbol(uri.clone(), alias.span.start)));
            }
        }
        for reference in &doc.references {
            let symbol = if let Some(generic) = reference.generic {
                Some(Symbol(uri.clone(), doc.generics[generic].span.start))
            } else {
                let target = reference
                    .path
                    .as_ref()
                    .and_then(|path| module_uri(uri, path));
                if reference.path.is_some() && target.is_none() {
                    continue;
                }
                if let Some(target) = target {
                    self.exported(&target, &reference.name.text)
                } else {
                    self.resolve(uri, &reference.name.text, &mut HashSet::new())
                }
            };
            if let Some(symbol) = symbol {
                result.push((&reference.name, symbol));
            }
        }
        result
    }

    fn selected(&self, uri: &Url, position: Position) -> Option<(&Name, Symbol)> {
        let byte = offset(&self.documents.get(uri)?.text, position)?;
        self.occurrences(uri)
            .into_iter()
            .find(|(name, _)| name.span.contains(&byte))
    }

    pub fn prepare(&self, uri: &Url, position: Position) -> Option<(Range, String)> {
        let (name, _) = self.selected(uri, position)?;
        Some((self.documents[uri].range(&name.span), name.text.clone()))
    }

    pub fn definition(&self, uri: &Url, position: Position) -> Option<Location> {
        let (_, mut symbol) = self.selected(uri, position)?;
        let mut seen = HashSet::new();
        while seen.insert(symbol.clone()) {
            let doc = self.documents.get(&symbol.0)?;
            if let Some(import) = doc
                .imports
                .iter()
                .find(|i| i.alias.as_ref().is_some_and(|a| a.span.start == symbol.1))
            {
                symbol = self.exported(
                    &module_uri(&symbol.0, &import.path)?,
                    &import.name.as_ref()?.text,
                )?;
            } else {
                let name = doc
                    .definitions
                    .iter()
                    .chain(&doc.generics)
                    .find(|n| n.span.start == symbol.1)?;
                return Some(Location::new(symbol.0.clone(), doc.range(&name.span)));
            }
        }
        None
    }

    pub fn rename(
        &self,
        uri: &Url,
        position: Position,
        new_name: &str,
    ) -> Result<Option<WorkspaceEdit>, String> {
        if !valid_name(new_name) {
            return Err("Enter a valid, non-reserved Genotype identifier".into());
        }
        let Some((selected, symbol)) = self.selected(uri, position) else {
            return Ok(None);
        };
        if self.documents.values().any(|doc| !doc.valid) {
            return Err("Fix syntax errors in the project before renaming symbols".into());
        }
        let mut changes = HashMap::new();
        for (uri, doc) in &self.documents {
            let occurrences = self.occurrences(uri);
            let edits: Vec<_> = occurrences
                .iter()
                .filter(|(_, target)| target == &symbol)
                .map(|(name, _)| TextEdit::new(doc.range(&name.span), new_name.into()))
                .collect();
            let glob_dependent = doc.imports.iter().any(|import| {
                import.name.is_none() && module_uri(uri, &import.path).as_ref() == Some(&symbol.0)
            }) && self.exported(&symbol.0, &selected.text).as_ref()
                == Some(&symbol);
            if edits.is_empty() && !glob_dependent {
                continue;
            }
            // Reject capture in any affected module, including generic scopes and
            // imported bindings. This deliberately favors a safe rename.
            if new_name != selected.text
                && (doc
                    .definitions
                    .iter()
                    .chain(&doc.generics)
                    .chain(
                        doc.imports
                            .iter()
                            .filter_map(|i| i.alias.as_ref().or(i.name.as_ref())),
                    )
                    .any(|n| n.text == new_name)
                    || self.resolve(uri, new_name, &mut HashSet::new()).is_some()
                    || doc.references.iter().any(|r| r.name.text == new_name))
            {
                return Err(format!(
                    "Renaming to {new_name} would conflict with another symbol in {uri}"
                ));
            }
            if !edits.is_empty() {
                changes.insert(uri.clone(), edits);
            }
        }
        Ok(Some(WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        }))
    }
}

fn module_paths(doc: &Document) -> impl Iterator<Item = &str> {
    doc.imports
        .iter()
        .map(|i| i.path.as_str())
        .chain(doc.references.iter().filter_map(|r| r.path.as_deref()))
}

fn valid_name(name: &str) -> bool {
    name.starts_with(|ch: char| ch.is_ascii_alphabetic())
        && name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
        && ![
            "use", "as", "any", "string", "boolean", "number", "int", "float", "true", "false",
            "null", "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128",
            "usize", "f32", "f64",
        ]
        .contains(&name)
}

fn normalize(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            path::Component::ParentDir => {
                normalized.pop();
            }
            path::Component::CurDir => {}
            other => normalized.push(other),
        }
    }
    normalized
}

fn module_uri(uri: &Url, import: &str) -> Option<Url> {
    let file = uri.to_file_path().ok()?;
    let parent = file.parent()?;
    let path = import.trim_end_matches('/');
    let target = if let Some(path) = path.strip_prefix("~/") {
        source_root(parent).join(path)
    } else if path.starts_with('.') {
        parent.join(path)
    } else {
        return None;
    };
    Url::from_file_path(normalize(&target.with_extension("type"))).ok()
}

pub fn project_root(file: &Path) -> PathBuf {
    let parent = file.parent().unwrap_or(file);
    parent
        .ancestors()
        .find(|dir| dir.join("genotype.toml").is_file())
        .unwrap_or(parent)
        .to_path_buf()
}

fn source_root(parent: &Path) -> PathBuf {
    for dir in parent.ancestors() {
        if let Ok(text) = std::fs::read_to_string(dir.join("genotype.toml"))
            && let Ok(config) = text.parse::<toml::Value>()
        {
            let root = config.get("root").and_then(|v| v.as_str()).unwrap_or(".");
            let src = config.get("src").and_then(|v| v.as_str()).unwrap_or("src");
            return dir.join(root).join(src);
        }
    }
    parent.to_path_buf()
}

pub fn in_source(root: &Path, path: &Path) -> bool {
    Entries::new(root).contains(path)
}

struct Entries {
    pattern: String,
}

impl Entries {
    fn new(root: &Path) -> Self {
        let config = std::fs::read_to_string(root.join("genotype.toml"))
            .ok()
            .and_then(|text| genotype_project::GtpConfig::parse(text).ok());
        let (source, entry) = if let Some(config) = config {
            (
                normalize(
                    &root
                        .join(config.root.to_string())
                        .join(config.src.to_string()),
                ),
                config.entry.to_string(),
            )
        } else {
            (root.to_path_buf(), "**/*.type".into())
        };
        Self {
            pattern: format!(
                "{}/{}",
                glob::Pattern::escape(&source.to_string_lossy()),
                entry
            ),
        }
    }

    fn files(&self) -> Vec<PathBuf> {
        glob::glob(&self.pattern)
            .map(|paths| paths.flatten().collect())
            .unwrap_or_default()
    }

    fn contains(&self, path: &Path) -> bool {
        glob::Pattern::new(&self.pattern).is_ok_and(|pattern| {
            pattern.matches_path_with(
                path,
                glob::MatchOptions {
                    require_literal_separator: true,
                    ..Default::default()
                },
            )
        })
    }
}
