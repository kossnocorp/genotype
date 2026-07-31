use crate::prelude::internal::*;

#[derive(Default)]
struct GtcCompilationBuildInfoCleanUpState {
    removed_files: Vec<GtpCwdRelativePath>,
    errors: Vec<String>,
    warnings: Vec<String>,
}

impl GtcCompilationBuildInfoCleanUpState {
    fn to_diagnostics(&self) -> Option<Vec<GtDiagnostic>> {
        if self.removed_files.is_empty() && self.errors.is_empty() && self.warnings.is_empty() {
            return None;
        }

        let mut diagnostics = vec![];

        if !self.removed_files.is_empty() {
            diagnostics.push(GtDiagnostic::success((
                format!(
                    "Cleaned up {pluralized_files}",
                    pluralized_files =
                        pluralize("dist files", self.removed_files.len() as isize, true),
                ),
                self.removed_files
                    .iter()
                    .map(|file| format!("- {file}", file = file.as_str()))
                    .collect::<Vec<_>>()
                    .join("\n"),
            )));
        }

        if !self.errors.is_empty() {
            diagnostics.push(GtDiagnostic::error((
                format!(
                    "Failed to clean up {pluralized_files}",
                    pluralized_files = pluralize("dist files", self.errors.len() as isize, true),
                ),
                self.errors
                    .iter()
                    .map(|error| format!("- {error}"))
                    .collect::<Vec<_>>()
                    .join("\n"),
            )));
        }

        if !self.warnings.is_empty() {
            diagnostics.push(GtDiagnostic::warning((
                format!(
                    "Warnings while cleaning up {pluralized_files}",
                    pluralized_files = pluralize("dist files", self.warnings.len() as isize, true),
                ),
                self.warnings
                    .iter()
                    .map(|warning| format!("- {warning}"))
                    .collect::<Vec<_>>()
                    .join("\n"),
            )));
        }

        Some(diagnostics)
    }
}

impl<Backend: GtBackend + ?Sized> GtcCompilation<'_, '_, Backend> {
    pub(super) async fn refresh_build_info_hashes(&mut self, lang: GtpLang) -> Result<()> {
        let build_info = match lang {
            GtpLang::Ts => self.build_info.dist.ts.as_mut(),
            GtpLang::Rs => self.build_info.dist.rs.as_mut(),
            GtpLang::Py => self.build_info.dist.py.as_mut(),
        };

        let Some(build_info) = build_info else {
            return Ok(());
        };

        for (path, file_build_info) in build_info {
            let file_content = {
                let cwd_rel_path = self.project.resolve_build_info_path(path)?;
                self.backend
                    .read_file(&cwd_rel_path)
                    .await
                    .with_context(|| {
                        format!(
                            "Failed to read file `{path}` while refreshing build info hashes",
                            path = path.0
                        )
                    })?
            };

            file_build_info.hash = GtpSourceCodeHash::new(&file_content);
        }

        Ok(())
    }

    pub(super) async fn update_build_info_file(&self) -> Result<Vec<GtDiagnostic>> {
        let mut diagnostics = vec![];

        let config_file = &self.project.paths().config_file;
        let build_file_path = config_file.to_build_file_path();

        let prev_build_info = match self.backend.file_exists(&build_file_path).await {
            Ok(false) => None,

            Ok(true) => match self.backend.read_file(&build_file_path).await {
                Ok(build_file_source_code) => match GtpBuildInfo::parse(&build_file_source_code) {
                    Ok(build_file) => Some(build_file),

                    Err(err) => {
                        diagnostics.push(GtDiagnostic::error((
                            format!("Failed to parse build info file `{build_file_path}`"),
                            format!("{err:?}"),
                        )));
                        return Ok(diagnostics);
                    }
                },

                Err(err) => {
                    diagnostics.push(GtDiagnostic::error(format!(
                        "Failed to read `{build_file_path}`: {err}"
                    )));
                    return Ok(diagnostics);
                }
            },

            Err(err) => {
                diagnostics.push(GtDiagnostic::error(format!(
                    "Failed to check if `{build_file_path}` exists: {err}"
                )));
                return Ok(diagnostics);
            }
        };

        if let Some(prev_build_info) = prev_build_info
            && self.project.config().build.cleanup
        {
            let mut clean_up_state = GtcCompilationBuildInfoCleanUpState::default();
            self.clean_up_dist(&mut clean_up_state, &prev_build_info.dist, GtpLang::Ts)
                .await?;

            self.clean_up_dist(&mut clean_up_state, &prev_build_info.dist, GtpLang::Rs)
                .await?;

            self.clean_up_dist(&mut clean_up_state, &prev_build_info.dist, GtpLang::Py)
                .await?;

            if let Some(clean_up_diagnostics) = clean_up_state.to_diagnostics() {
                diagnostics.extend(clean_up_diagnostics);
            }
        }

        let toml_str = self.build_info.to_toml_string();

        match toml_str {
            Ok(source) => {
                let source = oxc_toml::format(&source, oxc_toml::Options::default());
                if let Err(err) = self.backend.write_file(&build_file_path, &source).await {
                    diagnostics.push(GtDiagnostic::error(format!(
                        "Failed to write build info file `{build_file_path}`: {err}"
                    )));
                }
            }

            Err(err) => diagnostics.push(GtDiagnostic::error(format!(
                "Failed to serialize build info file `{build_file_path}`: {err}"
            ))),
        }

        Ok(diagnostics)
    }

    async fn clean_up_dist(
        &self,
        clean_up_state: &mut GtcCompilationBuildInfoCleanUpState,
        prev_dist: &GtpBuildInfoDist,
        lang: GtpLang,
    ) -> Result<()> {
        let prev_build_info = Self::build_info_lang(prev_dist, lang);
        let cur_build_info = Self::build_info_lang(&self.build_info.dist, lang);

        match (prev_build_info, cur_build_info) {
            (Some(files), None) => {
                self.clean_up_dist_files(clean_up_state, files, |_| true)
                    .await
            }

            (Some(prev_files), Some(cur_files)) => {
                self.clean_up_dist_files(clean_up_state, prev_files, |prev_path| {
                    !cur_files.contains_key(prev_path)
                })
                .await
            }

            _ => Ok(()),
        }
    }

    async fn clean_up_dist_files<ShouldRemoveFn: Fn(&GtpBuildInfoPath) -> bool>(
        &self,
        clean_up_state: &mut GtcCompilationBuildInfoCleanUpState,
        files: &GtpBuildInfoDistFiles,
        should_remove: ShouldRemoveFn,
    ) -> Result<()> {
        for build_info_path in files.keys() {
            if !should_remove(build_info_path) {
                continue;
            }

            let cwd_rel_path = self.project.resolve_build_info_path(build_info_path)?;

            match self.backend.file_exists(&cwd_rel_path).await {
                Ok(true) => {
                    // Proceed to try to remove the file
                }

                Ok(false) => {
                    continue;
                }

                Err(err) => {
                    clean_up_state
                        .errors
                        .push(format!("Failed to check if `{cwd_rel_path}` exists: {err}"));
                    continue;
                }
            }

            match self.backend.read_file(&cwd_rel_path).await {
                Ok(file_content) => {
                    let file_hash = GtpSourceCodeHash::new(&file_content);
                    let expected_hash = &files[build_info_path].hash;

                    if &file_hash != expected_hash {
                        clean_up_state.warnings.push(format!(
                            "File `{cwd_rel_path}` contents changed, skipping removal"
                        ));
                        continue;
                    }
                }

                Err(err) => {
                    clean_up_state
                        .errors
                        .push(format!("Failed to read file `{cwd_rel_path}`: {err}"));
                    continue;
                }
            }

            match self.backend.remove_file(&cwd_rel_path).await {
                Ok(()) => {
                    clean_up_state.removed_files.push(cwd_rel_path);
                }

                Err(err) => {
                    clean_up_state
                        .errors
                        .push(format!("Failed to remove file `{cwd_rel_path}`: {err}"));
                }
            }
        }

        Ok(())
    }

    fn build_info_lang(dist: &GtpBuildInfoDist, lang: GtpLang) -> Option<&GtpBuildInfoDistFiles> {
        match lang {
            GtpLang::Ts => dist.ts.as_ref(),
            GtpLang::Rs => dist.rs.as_ref(),
            GtpLang::Py => dist.py.as_ref(),
        }
    }
}
