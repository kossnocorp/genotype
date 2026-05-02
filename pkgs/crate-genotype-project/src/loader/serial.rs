use crate::prelude::internal::*;
use std::cell::RefCell;

/// Serial project loader trait. It implements the project loader trait without spawning threads.
/// It is useful for single-threaded runtimes such as WebAssembly.
pub trait GtpLoaderSerial: GtpLoader<RefCell<GtProject>> {
    /// Loads a module and its dependencies recursively.
    fn load_module_recursive(
        &self,
        project: &RefCell<GtProject>,
        path: GtpModulePath,
    ) -> Result<()> {
        if let Some(dep_paths) = self.load_project_module(project, path)? {
            for dep_path in dep_paths {
                self.load_module_recursive(project, dep_path)?;
            }
        }
        Ok(())
    }
}

impl<Type: GtpLoaderSerial + GtpSource + ?Sized> GtpLoader<RefCell<GtProject>> for Type {
    /// Loads module entries serially.
    fn load_module_entries(
        &self,
        project: GtProject,
        module_entries: Vec<GtpModulePath>,
    ) -> Result<GtProject> {
        let project = RefCell::new(project);
        for entry_module_path in module_entries {
            self.load_module_recursive(&project, entry_module_path)?;
        }
        Ok(project.into_inner())
    }

    /// Initializes the module.
    fn init_project_module(
        &self,
        project: &RefCell<GtProject>,
        path: &GtpModulePath,
    ) -> Result<Option<GtModuleId>> {
        let mut project = project.borrow_mut();
        project.init_module(path)
    }

    /// Sets the module state.
    fn set_project_module(
        &self,
        project: &RefCell<GtProject>,
        path: &GtpModulePath,
        state: GtpModule,
    ) -> Result<()> {
        let mut project = project.borrow_mut();
        project.set_module(path, state);
        Ok(())
    }
}
