use crate::prelude::internal::*;

use std::cell::RefCell;

pub struct GtpLoaderSerialKind;

/// Serial project loader trait. It implements the project loader trait without spawning threads.
/// It is useful for single-threaded runtimes such as WebAssembly.
pub trait GtpLoaderSerial<FileSourceKind>:
    GtpLoader<GtpLoaderSerialKind, FileSourceKind, ProjectRef = RefCell<GtProject>>
{
    /// Loads a module and its dependencies recursively.
    fn load_module_recursive<LoaderModule>(
        &self,
        project: &RefCell<GtProject>,
        module: LoaderModule,
    ) -> Result<()>
    where
        LoaderModule: Into<GtpModuleSource>,
    {
        let module = module.into();
        if let Some(dep_paths) = self.load_project_module(project, &module)? {
            for dep_path in dep_paths {
                self.load_module_recursive(project, dep_path)?;
            }
        }
        Ok(())
    }
}

impl<Type, FileSourceKind> GtpLoader<GtpLoaderSerialKind, FileSourceKind> for Type
where
    Type: GtpLoaderSerial<FileSourceKind> + GtpFileSource<FileSourceKind> + ?Sized,
{
    type ProjectRef = RefCell<GtProject>;

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
        source: &GtpModuleSource,
    ) -> Result<Option<GtModuleId>> {
        let mut project = project.borrow_mut();
        project.init_module(source)
    }

    /// Sets the module state.
    fn set_project_module(
        &self,
        project: &RefCell<GtProject>,
        source: &GtpModuleSource,
        state: GtpModule,
    ) -> Result<()> {
        let mut project = project.borrow_mut();
        project.set_module(source.path(), state);
        Ok(())
    }

    /// Adds module source to the project. It provides map of all module references.
    fn add_project_module_source(
        &self,
        project: &RefCell<GtProject>,
        source: &GtpModuleSource,
    ) -> Result<()> {
        let mut project = project.borrow_mut();
        project.add_module_source(source.clone());
        Ok(())
    }
}
