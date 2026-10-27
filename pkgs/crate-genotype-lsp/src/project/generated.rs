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

#[derive(Debug, Deserialize, Serialize)]
pub struct GeneratedSymbol {
    pub name: String,
    pub source: Location,
    pub targets: Vec<GeneratedTarget>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeneratedTarget {
    pub uri: Url,
    pub language: String,
}

pub fn generated_symbols(
    roots: &[PathBuf],
    open: &HashMap<Url, String>,
    params: GeneratedSymbolsParams,
) -> Result<Vec<GeneratedSymbol>, String> {
    let selected = if let Some(position) = params.position {
        let path = params
            .uri
            .to_file_path()
            .map_err(|_| "Expected a file URI")?;
        let project = Project::load(&[project_root(&path)], open);
        let Some((_, symbol)) = project.selected(&params.uri, position) else {
            return Ok(vec![]);
        };
        Some(symbol)
    } else {
        None
    };
    let requested_uri = selected.as_ref().map_or(&params.uri, |symbol| &symbol.0);
    let mut configs = HashSet::new();
    for root in roots {
        for entry in ignore::WalkBuilder::new(root).build().flatten() {
            if entry.file_name() == "genotype.toml" {
                configs.insert(entry.path().to_path_buf());
            }
        }
    }
    if let Ok(path) = requested_uri.to_file_path() {
        let config = project_root(&path).join("genotype.toml");
        if config.is_file() {
            configs.insert(config);
        }
    }
    let mut results = vec![];
    for config_path in configs {
        let text = Url::from_file_path(&config_path)
            .ok()
            .and_then(|uri| open.get(&uri).cloned())
            .or_else(|| std::fs::read_to_string(&config_path).ok());
        let Some(text) = text else {
            continue;
        };
        let config = match GtpConfig::parse(text) {
            Ok(config) => config,
            Err(error) => {
                if requested_uri
                    .to_file_path()
                    .is_ok_and(|path| path.starts_with(config_path.parent().unwrap()))
                {
                    return Err(format!("Cannot read {}: {error}", config_path.display()));
                }
                continue;
            }
        };
        let root = normalize(&config_path.parent().unwrap().join(config.root.to_string()));
        let src = normalize(&root.join(config.src.to_string()));
        let dist = normalize(&root.join(config.dist.to_string()));
        let requested_path = requested_uri
            .to_file_path()
            .map_err(|_| "Expected a file URI")?;
        if params.position.is_some() && !requested_path.starts_with(&src) {
            continue;
        }
        if params.position.is_none()
            && ![GtpLang::Ts, GtpLang::Rs, GtpLang::Py]
                .into_iter()
                .any(|lang| {
                    config.lang_enabled(lang)
                        && requested_path.starts_with(normalize(
                            &dist.join(config.lang(lang).dist_relative_pkg_path().to_string()),
                        ))
                })
        {
            continue;
        }
        let project = Project::load(std::slice::from_ref(&src), open);
        // Use rename identity, not navigation identity: a local import alias or
        // generic parameter must not rename the original exported declaration.
        for (uri, doc) in &project.documents {
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
                if params.position.is_some()
                    && selected.as_ref() != Some(&Symbol(uri.clone(), name.span.start))
                {
                    continue;
                }
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
                    let uri =
                        Url::from_file_path(output).map_err(|_| "Invalid generated file path")?;
                    targets.push(GeneratedTarget {
                        uri,
                        language: language.into(),
                    });
                }
                if params.position.is_none()
                    && !targets.iter().any(|target| target.uri == params.uri)
                {
                    continue;
                }
                if !targets.is_empty() {
                    results.push(GeneratedSymbol {
                        name: name.text.clone(),
                        source: Location::new(uri.clone(), doc.range(&name.span)),
                        targets,
                    });
                }
            }
        }
    }
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
        for target in a.targets.drain(..) {
            if !b.targets.iter().any(|other| other.uri == target.uri) {
                b.targets.push(target);
            }
        }
        true
    });
    for result in &mut results {
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
    Ok(results)
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
