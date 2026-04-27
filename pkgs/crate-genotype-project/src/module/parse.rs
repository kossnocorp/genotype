use crate::prelude::internal::*;

// region: Module parse

/// Parsed project module state.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtpModuleParse {
    /// Module path.
    pub path: GtpModulePath,

    /// Module source code.
    pub source_code: String,

    /// Module parse.
    pub module_parse: GtModuleParse,
}

impl GtpModuleParse {
    /// Dependency module paths.
    pub fn dep_paths(&self) -> Vec<GtpModulePath> {
        let mut paths = vec![];
        for dep in self.module_parse.resolve.deps.iter() {
            if dep.kind() == GtPathKind::Package {
                continue;
            }
            paths.push(self.path.resolve_path_node(dep));
        }
        paths
    }
}

impl Into<GtpModule> for GtpModuleParse {
    fn into(self) -> GtpModule {
        GtpModule::Parsed(self)
    }
}

// endregion

// region: Module parsing

impl GtpModule {
    pub fn parse(
        path: &GtpModulePath,
        module_id: GtModuleId,
        source_code: String,
    ) -> Result<GtpModuleParse, GtpModuleError> {
        GtModule::parse(module_id, &source_code)
            .map_err(|error| GtpModuleError::Parse {
                path: path.clone(),
                error,
                source_code: source_code.clone(),
            })
            .map(|parse| GtpModuleParse {
                path: path.clone(),
                module_parse: parse,
                source_code,
            })
    }
}

// endregion
