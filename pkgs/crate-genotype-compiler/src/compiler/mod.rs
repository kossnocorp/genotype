use crate::prelude::internal::*;

mod state;
pub use state::*;

mod meta;
pub use meta::*;

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

    fn meta(&self) -> &GtcMetaState;

    fn meta_mut(&mut self) -> &mut GtcMetaState;

    fn backend(&self) -> &impl GtBackend;

    async fn load_in_project(&mut self) -> Result<()> {
        let stage_started_at = std::time::Instant::now();
        match self.state() {
            GtCompilerState::New { config_path } => {
                let project = self
                    .backend()
                    .create_project(config_path.as_ref())
                    .await
                    .with_context(|| format!("Failed to load project"));

                match project {
                    Ok(project) => {
                        *self.state_mut() = GtCompilerState::LoadedProject { project };
                        let started_at =
                            self.meta().timing.started_at().unwrap_or(stage_started_at);
                        self.meta_mut().timing = GtcMetaTimingState::LoadedProject {
                            started_at,
                            load_project: stage_started_at.elapsed(),
                        };
                        Ok(())
                    }

                    Err(err) => Err(self.bypass_report_error(err).await),
                }
            }

            _ => Err(self
                .bypass_report_error(miette!(
                    "Cannot load project as the compiler has already loaded a project."
                ))
                .await),
        }
    }

    async fn load_in_modules(&mut self) -> Result<()> {
        let stage_started_at = std::time::Instant::now();
        match self.state() {
            GtCompilerState::New { .. } => Err(self
                .bypass_report_error(miette!(
                    "Cannot load modules as the compiler has not yet loaded a project."
                ))
                .await),

            GtCompilerState::LoadedProject { project }
            | GtCompilerState::LoadedModules { project } => {
                let project = self
                    .backend()
                    .load_all_modules(project.clone())
                    .await
                    .with_context(|| format!("Failed to load project modules"));

                match project {
                    Ok(project) => {
                        *self.state_mut() = GtCompilerState::LoadedModules { project };
                        let (started_at, load_project) = match &self.meta().timing {
                            GtcMetaTimingState::LoadedProject {
                                started_at,
                                load_project,
                            }
                            | GtcMetaTimingState::LoadedModules {
                                started_at,
                                load_project,
                                ..
                            }
                            | GtcMetaTimingState::Compiled {
                                started_at,
                                load_project,
                                ..
                            } => (*started_at, *load_project),
                            _ => (stage_started_at, Default::default()),
                        };
                        self.meta_mut().timing = GtcMetaTimingState::LoadedModules {
                            started_at,
                            load_project,
                            load_modules: stage_started_at.elapsed(),
                        };
                        Ok(())
                    }

                    Err(err) => Err(self.bypass_report_error(err).await),
                }
            }
        }
    }

    async fn compile(&mut self) -> Result<GtMeta> {
        let stage_started_at = std::time::Instant::now();
        match self.state() {
            GtCompilerState::New { .. } => Err(self
                .bypass_report_error(miette!(
                    "Cannot compile as the compiler has not yet loaded a project."
                ))
                .await),

            GtCompilerState::LoadedProject { .. } => Err(self
                .bypass_report_error(miette!(
                    "Cannot compile as the compiler has not yet loaded modules."
                ))
                .await),

            GtCompilerState::LoadedModules { project } => {
                let mut compilation = GtcCompilation::new(project, self.backend());

                let exit_code = compilation
                    .compile()
                    .await
                    .with_context(|| format!("Failed to compile project modules."));

                match exit_code {
                    Ok(exit_code) => {
                        let modules = compilation.meta_modules();
                        let (started_at, load_project, load_modules) = match &self.meta().timing {
                            GtcMetaTimingState::LoadedModules {
                                started_at,
                                load_project,
                                load_modules,
                            }
                            | GtcMetaTimingState::Compiled {
                                started_at,
                                load_project,
                                load_modules,
                                ..
                            } => (*started_at, *load_project, *load_modules),
                            _ => (stage_started_at, Default::default(), Default::default()),
                        };
                        let timing = GtMetaTiming {
                            total_ms: duration_ms(started_at.elapsed()),
                            load_project_ms: duration_ms(load_project),
                            load_modules_ms: duration_ms(load_modules),
                            compile_ms: duration_ms(stage_started_at.elapsed()),
                        };
                        self.meta_mut().timing = GtcMetaTimingState::Compiled {
                            started_at,
                            load_project,
                            load_modules,
                            timing: timing.clone(),
                        };
                        Ok(GtMeta {
                            exit_code,
                            timing,
                            modules,
                        })
                    }

                    Err(err) => Err(self.bypass_report_error(err).await),
                }
            }
        }
    }

    async fn compile_once(props: Props) -> Result<GtMeta>
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
