use crate::prelude::internal::*;

impl GtProject {
    pub fn build_info_src(&self) -> Result<GtpBuildInfoSrc> {
        let config_hash = self.config.source().hash.clone();
        let modules = self.build_info_src_modules()?;

        Ok(GtpBuildInfoSrc {
            config_hash,
            modules,
        })
    }

    fn build_info_src_modules(&self) -> Result<GtpBuildInfoSrcModules> {
        self.modules
            .iter()
            .map(|(path, module)| match module {
                GtpModule::TypeChecked(type_checked) => {
                    let build_info_path = self.build_info_path(path)?;
                    Ok(Some((build_info_path, type_checked.build_info())))
                }

                _ => Ok(None),
            })
            .filter_map(Result::transpose)
            .collect()
    }

    pub fn build_info_path<Path: GtpCwdRelativePathWrapper>(
        &self,
        path: &Path,
    ) -> Result<GtpBuildInfoPath> {
        let config_dir = self.paths.config_file.to_config_dir_path();
        let config_dir_rel_path = path
            .relative_path_to(&config_dir)
            .with_context(|| "Failed to resolve build info path")?;
        let build_info_path = GtpBuildInfoPath(config_dir_rel_path.to_string());
        Ok(build_info_path)
    }

    pub fn resolve_build_info_path(&self, path: &GtpBuildInfoPath) -> Result<GtpCwdRelativePath> {
        let config_dir = self.paths.config_file.to_config_dir_path();
        Ok(config_dir.join_str_as_cwd_relative_path(&path.0))
    }
}
