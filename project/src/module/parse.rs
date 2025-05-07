use crate::prelude::internal::*;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Clone)]
pub struct GTProjectModuleParse(pub GtSrcRelativePath, pub GTModuleParse);

impl<'a> GTProjectModuleParse {
    pub fn try_new(config: &GtConfig, src_relative_path: GtSrcRelativePath) -> Result<Self> {
        let module_path = config.src_path().join(&src_relative_path);
        let code = read_to_string(module_path.as_str())
            .map_err(|_| GTProjectError::NotFound(module_path.as_str().into()))?;

        let source_code = NamedSource::new(module_path.as_str(), code.clone());
        let parse = GTModule::parse((&src_relative_path).into(), source_code)?;
        Ok(Self(src_relative_path, parse))
    }

    pub fn deps(&self) -> Result<Vec<GtSrcRelativePath>> {
        let mut paths = vec![];
        for dep in self.1.resolve.deps.iter() {
            if dep.kind() == GTPathKind::Package {
                continue;
            }
            paths.push(self.0.join_tree(dep));
        }
        Ok(paths)
    }
}
