use crate::prelude::internal::*;
use rayon::Scope;
use std::sync::Arc;
use std::sync::Mutex;

/// Parallel project loader trait. It implements the project loader trait with parallel loading
/// capabilities. It is used by the system project runtime.
pub trait GtpLoaderParallel: GtpLoader {
    fn load_module_in_scope<'a>(
        &'a self,
        scope: &Scope<'a>,
        project: Arc<Mutex<&'a mut GtProject>>,
        path: GtpModulePath,
    ) where
        Self: Sync + Send,
    {
        let project = Arc::clone(&project);
        scope.spawn(move |scope| {
            {
                let mut project = project.lock().expect("Failed to lock project mutex");
                if !project.init_module(&path) {
                    return;
                }
            }

            let module_source = self.read_file(path.cwd_relative_path());

            match module_source {
                Ok(source) => {
                    let source_code = NamedSource::new(path.as_str(), source.clone());
                    let parse = GtModule::parse(path.into(), source_code);

                    match parse {
                        Ok(parse) => {
                            todo!()
                        }

                        Err(error) => {
                            let mut project = project.lock().expect("Failed to lock project mutex");
                            project.set_module(
                                &path,
                                GtpModuleState::Error(GtpModuleError::Read(
                                    path.clone(),
                                    error.to_string(),
                                )),
                            );
                        }
                    }
                }

                Err(error) => {
                    let mut project = project.lock().expect("Failed to lock project mutex");
                    project.set_module(
                        &path,
                        GtpModuleState::Error(GtpModuleError::Read(
                            path.clone(),
                            error.to_string(),
                        )),
                    );
                }
            }
        });
    }
}

impl<Type: GtpLoaderParallel + GtpSource + Sync + Send + ?Sized> GtpLoader for Type {
    /// Loads all project modules in parallel.
    fn load_all_modules(&self, project: &mut GtProject) -> Result<()> {
        println!(">>>>>>>>>>>> project.paths: {:?}", project.paths);

        let entry_module_paths = self
            .glob(project.paths.entry.as_ref())?
            .into_iter()
            .map(|path| path.into())
            .collect::<Vec<GtpModulePath>>();

        ensure!(
            entry_module_paths.len() > 0,
            "no module files found for entry pattern '{}'",
            project.paths.entry.display()
        );

        let project = Arc::new(Mutex::new(project));

        rayon::scope(|scope| {
            let project = Arc::clone(&project);

            for entry_module_path in entry_module_paths {
                let project = Arc::clone(&project);
                scope.spawn(move |scope| {
                    self.load_module_in_scope(scope, project, entry_module_path);
                });
            }
        });

        Ok(())
    }
}
