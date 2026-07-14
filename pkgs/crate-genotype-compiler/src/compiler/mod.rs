use crate::prelude::internal::*;

mod state;
pub use state::*;

mod system;
pub use system::*;

mod remote;
pub use remote::*;

#[allow(async_fn_in_trait)]
pub trait GtCompiler<Props> {
    fn new(props: Props) -> Result<Self>
    where
        Self: Sized;

    fn state(&self) -> &GtCompilerState;

    fn state_mut(&mut self) -> &mut GtCompilerState;

    fn backend(&self) -> &impl GtBackend;

    async fn load_in_project(&mut self) -> Result<GtcMetaLoadedProject> {
        let config_path = match self.state() {
            GtCompilerState::New { config_path, .. } => config_path.clone(),
            _ => {
                return Err(self
                    .bypass_report_error(miette!(
                        "Cannot load project as the compiler has already loaded a project."
                    ))
                    .await);
            }
        };

        let project = self
            .backend()
            .create_project(config_path.as_ref())
            .await
            .with_context(|| "Failed to load project".to_string());

        match project {
            Ok(project) => {
                let meta = GtcMetaLoadedProject {
                    paths: GtcMetaLoadedProjectPaths {
                        src: project.paths().src.to_string(),
                    },
                };
                *self.state_mut() = GtCompilerState::LoadedProject {
                    project,
                    meta: meta.clone(),
                };
                Ok(meta)
            }

            Err(err) => Err(self.bypass_report_error(err).await),
        }
    }

    async fn load_in_modules(&mut self) -> Result<GtcMetaLoadedModules> {
        let project = match self.state() {
            GtCompilerState::New { .. } => {
                return Err(self
                    .bypass_report_error(miette!(
                        "Cannot load modules as the compiler has not yet loaded a project."
                    ))
                    .await);
            }
            GtCompilerState::LoadedProject { project, .. }
            | GtCompilerState::LoadedModules { project, .. }
            | GtCompilerState::Compiled { project, .. } => project.clone(),
        };

        let project = self
            .backend()
            .load_all_modules(project)
            .await
            .with_context(|| "Failed to load project modules".to_string());

        match project {
            Ok(project) => {
                let mut modules = project
                    .modules()
                    .keys()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>();
                modules.sort();
                let meta = GtcMetaLoadedModules {
                    paths: GtcMetaLoadedProjectPaths {
                        src: project.paths().src.to_string(),
                    },
                    modules,
                };
                *self.state_mut() = GtCompilerState::LoadedModules {
                    project,
                    meta: meta.clone(),
                };
                Ok(meta)
            }

            Err(err) => Err(self.bypass_report_error(err).await),
        }
    }

    async fn compile(&mut self) -> Result<GtcMetaCompiled> {
        let project = match self.state() {
            GtCompilerState::New { .. } => {
                return Err(self
                    .bypass_report_error(miette!(
                        "Cannot compile as the compiler has not yet loaded a project."
                    ))
                    .await);
            }
            GtCompilerState::LoadedProject { .. } => {
                return Err(self
                    .bypass_report_error(miette!(
                        "Cannot compile as the compiler has not yet loaded modules."
                    ))
                    .await);
            }
            GtCompilerState::LoadedModules { project, .. }
            | GtCompilerState::Compiled { project, .. } => project.clone(),
        };

        let mut compilation = GtcCompilation::new(&project, self.backend());
        let exit_code = compilation
            .compile()
            .await
            .with_context(|| "Failed to compile project modules.".to_string());

        match exit_code {
            Ok(exit_code) => {
                let meta = GtcMetaCompiled {
                    exit_code,
                    paths: compilation.meta_paths(),
                    modules: compilation.meta_modules(),
                };
                *self.state_mut() = GtCompilerState::Compiled {
                    project,
                    meta: meta.clone(),
                };
                Ok(meta)
            }

            Err(err) => Err(self.bypass_report_error(err).await),
        }
    }

    async fn compile_once(props: Props) -> Result<GtcMetaCompiled>
    where
        Self: Sized,
    {
        let mut compiler = Self::new(props)?;

        compiler.load_in_project().await?;
        compiler.load_in_modules().await?;

        compiler.compile().await
    }

    async fn bypass_report_error(&self, err: Report) -> Report {
        let _ = self
            .backend()
            .report_diagnostic(&GtDiagnostic::error(miette!("{err:?}")))
            .await;
        err
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;

    #[test]
    fn enriches_and_returns_metadata_for_each_stage() {
        let base_path: GtpCwdRelativeOrAbsoluteStringPath = ".".into();
        let config_path: GtpCwdRelativeOrAbsoluteStringPath =
            "../crate-genotype-lang-ts-project/examples/basic/genotype.toml".into();
        let mut compiler = GtcSystem::new((&base_path, Some(&config_path))).unwrap();

        assert!(matches!(compiler.state(), GtCompilerState::New { .. }));

        let project_meta = block_on(compiler.load_in_project()).unwrap();
        assert_eq!(
            project_meta.paths.src,
            "../crate-genotype-lang-ts-project/examples/basic/src"
        );
        assert!(matches!(
            compiler.state(),
            GtCompilerState::LoadedProject { meta, .. } if meta == &project_meta
        ));

        let modules_meta = block_on(compiler.load_in_modules()).unwrap();
        assert!(!modules_meta.modules.is_empty());
        assert!(matches!(
            compiler.state(),
            GtCompilerState::LoadedModules { meta, .. } if meta == &modules_meta
        ));

        let compiled_meta = block_on(compiler.compile()).unwrap();
        assert_eq!(compiled_meta.exit_code, 0);
        assert!(matches!(
            compiler.state(),
            GtCompilerState::Compiled { meta, .. } if meta == &compiled_meta
        ));
    }
}
