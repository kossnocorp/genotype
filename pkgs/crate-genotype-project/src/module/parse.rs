use crate::prelude::internal::*;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Clone)]
pub struct GtProjectModuleParse(pub GtModulePath, pub GtModuleParse);

impl GtProjectModuleParse {
    pub fn try_new(config: &GtConfig, module_path: GtModulePath) -> Result<Self> {
        let cwd_relative_module_path = config.src_path().join(&module_path.clone().into());
        let code = read_to_string(cwd_relative_module_path.as_str())
            .map_err(|_| GtProjectError::NotFound(cwd_relative_module_path.as_str().into()))?;

        let source_code = NamedSource::new(module_path.as_str(), code.clone());
        let parse = GtModule::parse((&module_path).into(), source_code)?;
        Ok(Self(module_path, parse))
    }

    pub fn deps(&self) -> Result<Vec<GtModulePath>> {
        let mut paths = vec![];
        for dep in self.1.resolve.deps.iter() {
            if dep.kind() == GtPathKind::Package {
                continue;
            }
            paths.push(self.0.resolve(dep));
        }
        Ok(paths)
    }
}
