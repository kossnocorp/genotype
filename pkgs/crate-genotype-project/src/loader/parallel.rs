use crate::prelude::internal::*;

use rayon::Scope;
use std::sync::{Arc, Mutex};

/// Parallel project loader trait. It implements the project loader trait with parallel loading
/// capabilities. It is used by the system project runtime.
pub trait GtpLoaderParallel: GtpLoader<Arc<Mutex<GtProject>>> {
    /// Loads a module in a thread scope.
    fn load_module_in_scope<'a, Source>(
        &'a self,
        scope: &Scope<'a>,
        project: Arc<Mutex<GtProject>>,
        source: Source,
    ) where
        Self: Sync + Send,
        Source: Into<GtpModuleSource> + Sync + Send + 'a,
    {
        scope.spawn(move |scope| {
            let source = source.into();
            let deps_result = self.load_project_module(&project, &source);

            match deps_result {
                Ok(Some(deps)) => {
                    for dep_source in deps {
                        let project = Arc::clone(&project);
                        self.load_module_in_scope(scope, project, dep_source);
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

impl<Type: GtpLoaderParallel + GtpFileSource + Sync + Send + ?Sized>
    GtpLoader<Arc<Mutex<GtProject>>> for Type
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
    fn init_project_module(
        &self,
        project: &Arc<Mutex<GtProject>>,
        source: &GtpModuleSource,
    ) -> Result<Option<GtModuleId>> {
        let mut project = project
            .lock()
            .map_err(|_| miette!("failed to lock project mutex"))?;
        project.init_module(source)
    }

    /// Sets the module state.
    fn set_project_module(
        &self,
        project: &Arc<Mutex<GtProject>>,
        source: &GtpModuleSource,
        state: GtpModule,
    ) -> Result<()> {
        let mut project = project
            .lock()
            .map_err(|_| miette!("failed to lock project mutex"))?;
        project.set_module(source.path(), state);
        Ok(())
    }

    /// Adds module source to the project. It provides map of all module references.
    fn add_project_module_source(
        &self,
        project: &Arc<Mutex<GtProject>>,
        source: &GtpModuleSource,
    ) -> Result<()> {
        let mut project = project
            .lock()
            .map_err(|_| miette!("failed to lock project mutex"))?;
        project.add_module_source(source.clone());
        Ok(())
    }
}
