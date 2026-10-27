use crate::document::{Document, Name, error_diagnostic, offset};
use async_lsp::lsp_types::{Diagnostic, Location, Position, Range, TextEdit, Url, WorkspaceEdit};
use std::collections::{HashMap, HashSet};
use std::path::{self as path, Path, PathBuf};

pub mod generated;
mod intelligence;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Symbol(Url, usize);

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
                && self
                    .resolve(&target, &name.text, &mut HashSet::new())
                    .is_none()
                && !self.unavailable(&target, &name.text, &mut HashSet::new())
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
            if self
                .resolve(&target, &reference.name.text, &mut HashSet::new())
                .is_none()
                && !self.unavailable(&target, &reference.name.text, &mut HashSet::new())
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
    fn unavailable(&self, uri: &Url, name: &str, visited: &mut HashSet<Url>) -> bool {
        if !visited.insert(uri.clone()) {
            return false;
        }
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
                Some(target) => self.unavailable(
                    &target,
                    import.name.as_ref().map_or(name, |n| n.text.as_str()),
                    visited,
                ),
            })
    }

    pub fn load(roots: &[PathBuf], open: &HashMap<Url, String>) -> Self {
        let mut documents = HashMap::new();
        for root in roots {
            for entry in ignore::WalkBuilder::new(root)
                .follow_links(false)
                .build()
                .flatten()
            {
                let path = entry.path();
                if path.extension().is_some_and(|ext| ext == "type")
                    && entry.file_type().is_some_and(|kind| kind.is_file())
                    && let (Ok(uri), Ok(text)) =
                        (Url::from_file_path(path), std::fs::read_to_string(path))
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
        // Follow imports outside workspace roots as well (including unopened files).
        loop {
            let mut pending = HashSet::new();
            for (uri, doc) in &documents {
                for path in doc
                    .imports
                    .iter()
                    .map(|i| i.path.as_str())
                    .chain(doc.references.iter().filter_map(|r| r.path.as_deref()))
                {
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
        Self { documents }
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
                return self.resolve(&module_uri(uri, &import.path)?, name, visited);
            }
        }
        let mut found = None;
        for import in doc.imports.iter().filter(|i| i.name.is_none()) {
            if let Some(target) = module_uri(uri, &import.path)
                && let Some(symbol) = self.resolve(&target, name, &mut visited.clone())
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
                && let Some(symbol) = self.resolve(&target, &name.text, &mut HashSet::new())
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
                self.resolve(
                    target.as_ref().unwrap_or(uri),
                    &reference.name.text,
                    &mut HashSet::new(),
                )
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
                symbol = self.resolve(
                    &module_uri(&symbol.0, &import.path)?,
                    &import.name.as_ref()?.text,
                    &mut HashSet::new(),
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
            if edits.is_empty() {
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
            changes.insert(uri.clone(), edits);
        }
        Ok(Some(WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        }))
    }
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
