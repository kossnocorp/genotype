//! Maps source declarations to configured output modules. Native language servers
//! locate the declarations inside those modules; we never guess generated spans.
use super::*;
use genotype_core::GtModuleId;
use genotype_project::GtpConfig;
use genotype_project_core::GtpLang;
use serde::{Deserialize, Serialize};
use std::path::Component;

pub enum GeneratedSymbols {}

impl async_lsp::lsp_types::request::Request for GeneratedSymbols {
    type Params = GeneratedSymbolsParams;
    type Result = Vec<GeneratedSymbol>;
    const METHOD: &'static str = "genotype/generatedSymbols";
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeneratedSymbolsParams {
    pub uri: Url,
    pub position: Option<Position>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GeneratedSymbol {
    pub name: String,
    pub source: Location,
    pub targets: Vec<GeneratedTarget>,
    #[serde(rename = "renameUnsupported", skip_serializing_if = "Option::is_none")]
    pub rename_unsupported: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GeneratedTarget {
    pub uri: Url,
    pub language: String,
}

#[cfg(test)]
pub fn generated_symbols(
    roots: &[PathBuf],
    open: &HashMap<Url, String>,
    params: GeneratedSymbolsParams,
) -> Result<Vec<GeneratedSymbol>, String> {
    Cache::default().query(roots, open, params)
}

#[derive(Default)]
pub struct Cache {
    configs: Option<HashSet<PathBuf>>,
    mappings: HashMap<PathBuf, Result<Vec<GeneratedSymbol>, String>>,
    projects: HashMap<PathBuf, Project>,
    external_configs: HashMap<PathBuf, Option<String>>,
}

impl Cache {
    pub fn project_roots(&mut self, roots: &[PathBuf]) -> Vec<PathBuf> {
        let configs = self.configs.get_or_insert_with(|| {
            roots
                .iter()
                .flat_map(|root| {
                    ignore::WalkBuilder::new(root)
                        .build()
                        .flatten()
                        .filter(|entry| entry.file_name() == "genotype.toml")
                        .map(|entry| entry.into_path())
                })
                .collect()
        });
        let mut projects: Vec<_> = configs
            .iter()
            .filter_map(|path| path.parent().map(Path::to_path_buf))
            .collect();
        for root in roots {
            if !projects.iter().any(|project| project.starts_with(root)) {
                projects.push(root.clone());
            }
        }
        projects
    }

    pub fn invalidate(&mut self, uri: &Url, open: &HashMap<Url, String>) {
        let Ok(path) = uri.to_file_path() else { return };
        if path.file_name().is_some_and(|name| name == "genotype.toml") {
            if let Some(configs) = &mut self.configs {
                if path.is_file() || open.contains_key(uri) {
                    configs.insert(path.clone());
                } else {
                    configs.remove(&path);
                }
            }
            self.projects.clear();
            self.mappings.clear();
        } else if path.extension().is_some_and(|ext| ext == "type") {
            for (root, project) in &mut self.projects {
                if project.documents.contains_key(uri)
                    || project.references_module(uri)
                    || in_source(root, &path)
                    || (open.contains_key(uri) && path.starts_with(root))
                {
                    project.update(uri, open);
                    project.prune(std::slice::from_ref(root), open);
                    self.mappings.remove(&root.join("genotype.toml"));
                }
            }
        }
    }

    pub fn query(
        &mut self,
        roots: &[PathBuf],
        open: &HashMap<Url, String>,
        params: GeneratedSymbolsParams,
    ) -> Result<Vec<GeneratedSymbol>, String> {
        let path = params
            .uri
            .to_file_path()
            .map_err(|_| "Expected a file URI")?;
        let selected = if let Some(position) = params.position {
            let root = project_root(&path);
            let project = self
                .projects
                .entry(root.clone())
                .or_insert_with(|| Project::load(&[root], open));
            if project.refresh_external(roots, open) {
                project.prune(&[project_root(&path)], open);
                self.mappings.clear();
            }
            let Some((_, symbol)) = project.selected(&params.uri, position) else {
                return Ok(vec![]);
            };
            Some(Location::new(
                symbol.0.clone(),
                project.documents[&symbol.0].range(&(symbol.1..symbol.1)),
            ))
        } else {
            None
        };
        let requested_uri = selected.as_ref().map_or(&params.uri, |symbol| &symbol.uri);
        self.project_roots(roots);
        let configs = self.configs.as_mut().unwrap();
        let path = requested_uri
            .to_file_path()
            .map_err(|_| "Expected a file URI")?;
        let config = project_root(&path).join("genotype.toml");
        if config.is_file() {
            configs.insert(config);
        }
        let mut results = vec![];
        for config_path in configs.iter() {
            let root = config_path.parent().unwrap().to_path_buf();
            if !roots
                .iter()
                .any(|workspace| config_path.starts_with(workspace))
            {
                let text = Url::from_file_path(config_path)
                    .ok()
                    .and_then(|uri| open.get(&uri).cloned())
                    .or_else(|| std::fs::read_to_string(config_path).ok());
                if self
                    .external_configs
                    .get(config_path)
                    .is_some_and(|old| old != &text)
                {
                    self.mappings.remove(config_path);
                    self.projects.remove(&root);
                }
                self.external_configs.insert(config_path.clone(), text);
            }
            if let Some(project) = self.projects.get_mut(&root)
                && project.refresh_external(roots, open)
            {
                project.prune(std::slice::from_ref(&root), open);
                self.mappings.remove(config_path);
            }
            let mappings = self.mappings.entry(config_path.clone()).or_insert_with(|| {
                let project = self
                    .projects
                    .entry(root.clone())
                    .or_insert_with(|| Project::load(&[root], open));
                build_mappings(config_path, open, project)
            });
            match mappings {
                Ok(mappings) => results.extend(
                    mappings
                        .iter()
                        .filter(|mapping| {
                            if let Some(symbol) = &selected {
                                mapping.source.uri == symbol.uri
                                    && mapping.source.range.start == symbol.range.start
                            } else {
                                mapping
                                    .targets
                                    .iter()
                                    .any(|target| target.uri == params.uri)
                            }
                        })
                        .cloned(),
                ),
                Err(error) if path.starts_with(config_path.parent().unwrap()) => {
                    return Err(error.clone());
                }
                Err(_) => {}
            }
        }
        merge_mappings(&mut results);
        Ok(results)
    }
}

fn build_mappings(
    config_path: &Path,
    open: &HashMap<Url, String>,
    project: &Project,
) -> Result<Vec<GeneratedSymbol>, String> {
    let mut results = vec![];
    let text = Url::from_file_path(config_path)
        .ok()
        .and_then(|uri| open.get(&uri).cloned())
        .or_else(|| std::fs::read_to_string(config_path).ok());
    let Some(text) = text else {
        return Ok(results);
    };
    let config = match GtpConfig::parse(text) {
        Ok(config) => config,
        Err(error) => {
            return Err(format!("Cannot read {}: {error}", config_path.display()));
        }
    };
    let root = normalize(&config_path.parent().unwrap().join(config.root.to_string()));
    let src = normalize(&root.join(config.src.to_string()));
    let dist = normalize(&root.join(config.dist.to_string()));
    let reachable = project.reachable(
        &[config_path.parent().unwrap().to_path_buf()],
        &HashMap::new(),
    );
    // Use rename identity, not navigation identity: a local import alias or
    // generic parameter must not rename the original exported declaration.
    for (uri, doc) in &project.documents {
        if !reachable.contains(uri) {
            continue;
        }
        let Ok(path) = uri.to_file_path() else {
            continue;
        };
        let Ok(relative) = path.strip_prefix(&src) else {
            continue;
        };
        let module = GtModuleId(
            relative
                .with_extension("")
                .to_string_lossy()
                .replace('\\', "/"),
        );
        for name in &doc.definitions {
            let mut targets = vec![];
            for (lang, language) in [
                (GtpLang::Ts, "typescript"),
                (GtpLang::Rs, "rust"),
                (GtpLang::Py, "python"),
            ] {
                if !config.lang_enabled(lang) {
                    continue;
                }
                let target = config.lang(lang);
                let mut output = dist.join(target.dist_relative_pkg_path().to_string());
                if config.lang_package_enabled(target) {
                    output = output.join(target.pkg_dir_relative_src_dir_path().to_string());
                }
                output = normalize(
                    &output.join(target.pkg_src_dir_relative_module_path(&module).to_string()),
                );
                let uri = Url::from_file_path(output).map_err(|_| "Invalid generated file path")?;
                targets.push(GeneratedTarget {
                    uri,
                    language: language.into(),
                });
            }
            if !targets.is_empty() {
                results.push(GeneratedSymbol {
                    name: name.text.clone(),
                    source: Location::new(uri.clone(), doc.range(&name.span)),
                    rename_unsupported: (doc.entities.get(&name.span.start).is_some_and(|entity| entity.has_derived_types)
                        && targets.iter().any(|target| matches!(target.language.as_str(), "rust" | "python")))
                        .then(|| "This type generates additional named object types in Rust or Python. Coordinated rename of those derived types is not supported yet.".into()),
                    targets,
                });
            }
        }
    }
    Ok(results)
}

fn merge_mappings(results: &mut Vec<GeneratedSymbol>) {
    results.sort_by(|a, b| {
        a.source
            .uri
            .as_str()
            .cmp(b.source.uri.as_str())
            .then(a.name.cmp(&b.name))
    });
    results.dedup_by(|a, b| {
        if a.source != b.source || a.name != b.name {
            return false;
        }
        if b.rename_unsupported.is_none() {
            b.rename_unsupported = a.rename_unsupported.take();
        }
        for target in a.targets.drain(..) {
            if !b.targets.iter().any(|other| other.uri == target.uri) {
                b.targets.push(target);
            }
        }
        true
    });
    for result in results {
        result.targets.sort_by_key(|target| {
            (
                match target.language.as_str() {
                    "typescript" => 0,
                    "rust" => 1,
                    _ => 2,
                },
                target.uri.clone(),
            )
        });
    }
}

fn normalize(path: &Path) -> PathBuf {
    let mut result = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                result.pop();
            }
            other => result.push(other.as_os_str()),
        }
    }
    result
}
