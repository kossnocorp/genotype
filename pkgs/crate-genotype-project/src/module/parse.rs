use crate::prelude::internal::*;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Clone)]
pub struct GtpModuleParse(pub GtpSrcDirRelativeModulePath, pub GtModuleParse);

impl GtpModuleParse {
    pub fn try_new(
        src_dir_path: &GtpSrcDirPath,
        config: &GtpConfig,
        module_path: GtpSrcDirRelativeModulePath,
    ) -> Result<Self> {
        let cwd_relative_module_path = src_dir_path.join_as_cwd_relative_path(&module_path);
        let code = read_to_string(cwd_relative_module_path.as_str())
            .map_err(|_| GtpError::NotFound(cwd_relative_module_path.as_str().into()))?;

        let source_code = NamedSource::new(module_path.as_str(), code.clone());
        let parse = GtModule::parse((&module_path).into(), source_code)?;
        Ok(Self(module_path, parse))
    }

    pub fn deps(&self) -> Result<Vec<GtpSrcDirRelativeModulePath>> {
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
