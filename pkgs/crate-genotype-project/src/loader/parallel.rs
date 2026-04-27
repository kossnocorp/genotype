use crate::prelude::internal::*;

use rayon::Scope;
use std::sync::Arc;
use std::sync::Mutex;

/// Parallel project loader trait. It implements the project loader trait with parallel loading
/// capabilities. It is used by the system project runtime.
pub trait GtpLoaderParallel: GtpLoader<Arc<Mutex<GtProject>>> {
    /// Loads a module in a thread scope.
    fn load_module_in_scope<'a>(
        &'a self,
        scope: &Scope<'a>,
        project: Arc<Mutex<GtProject>>,
        path: GtpModulePath,
    ) where
        Self: Sync + Send,
    {
        scope.spawn(move |scope| {
            let dep_paths_result = self.load_project_module(&project, path);

            match dep_paths_result {
                Ok(Some(dep_paths)) => {
                    for dep_path in dep_paths {
                        let project = Arc::clone(&project);
                        self.load_module_in_scope(scope, project, dep_path);
                    }
                }

                // No dependencies to load.
                Ok(None) => {}

                // Got unrecoverable error during module loading.
                Err(err) => {
                    panic!("failed to load module: {err}")
                }
            }
        });
    }
}

impl<Type: GtpLoaderParallel + GtpSource + Sync + Send + ?Sized> GtpLoader<Arc<Mutex<GtProject>>>
    for Type
{
    /// Loads module entries in parallel.
    fn load_module_entries(
        &self,
        project: GtProject,
        module_entries: Vec<GtpModulePath>,
    ) -> Result<GtProject> {
        let project = Arc::new(Mutex::new(project));
        rayon::scope(|scope| {
            let project = Arc::clone(&project);
            for entry_module_path in module_entries {
                let project = Arc::clone(&project);
                scope.spawn(move |scope| {
                    self.load_module_in_scope(scope, project, entry_module_path)
                });
            }
        });

        Arc::try_unwrap(project)
            .map_err(|_| miette!("failed to unwrap project Arc"))?
            .into_inner()
            .map_err(|_| miette!("failed to lock project mutex"))
    }

    /// Initializes the module.
    fn init_project_module<'a>(
        &'a self,
        project: &Arc<Mutex<GtProject>>,
        path: &GtpModulePath,
    ) -> Result<Option<GtModuleId>> {
        let mut project = project
            .lock()
            .map_err(|_| miette!("failed to lock project mutex"))?;
        project.init_module(&path)
    }

    /// Sets the module state.
    fn set_project_module(
        &self,
        project: &Arc<Mutex<GtProject>>,
        path: &GtpModulePath,
        state: GtpModule,
    ) -> Result<()> {
        let mut project = project
            .lock()
            .map_err(|_| miette!("failed to lock project mutex"))?;
        project.set_module(path, state);
        Ok(())
    }
}
