use crate::prelude::internal::*;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Clone)]
pub struct GTProjectModuleParse(pub GtModulePath, pub GTModuleParse);

impl<'a> GTProjectModuleParse {
    pub fn try_new(config: &GtConfig, module_path: GtModulePath) -> Result<Self> {
        let cwd_relative_module_path = config.src_path().join(&module_path.clone().into());
        let code = read_to_string(cwd_relative_module_path.as_str())
            .map_err(|_| GTProjectError::NotFound(cwd_relative_module_path.as_str().into()))?;

        let source_code = NamedSource::new(module_path.as_str(), code.clone());
        let parse = GTModule::parse((&module_path).into(), source_code)?;
        Ok(Self(module_path, parse))
    }

    pub fn deps(&self) -> Result<Vec<GtModulePath>> {
        let mut paths = vec![];
        for dep in self.1.resolve.deps.iter() {
            if dep.kind() == GTPathKind::Package {
                continue;
            }
            paths.push(self.0.resolve(dep));
        }
        Ok(paths)
    }
}
