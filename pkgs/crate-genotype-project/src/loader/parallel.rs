use crate::prelude::internal::*;
use std::sync::mpsc;

/// Parallel project loader trait. It implements the project loader trait with parallel loading
/// capabilities. It is used by the system project runtime.
pub trait GtpLoaderParallel {}

impl<Type: GtpLoaderParallel + GtpSource + ?Sized> GtpLoader for Type {
    /// Loads all project modules in parallel.
    fn load_all_modules(&self, project: &mut GtProject) -> Result<()> {
        println!(">>>>>>>>>>>> project.paths: {:?}", project.paths);

        let module_paths = self
            .glob(project.paths.entry.as_ref())?
            .into_iter()
            .map(|path| path.into())
            .collect::<Vec<GtpModulePath>>();

        ensure!(
            module_paths.len() > 0,
            "no module files found for entry pattern '{}'",
            project.paths.entry.display()
        );

        // let (tx, rx) = mpsc::channel();

        println!("-------------------------");
        println!("Module paths: {:?}", module_paths);

        bail!("WIP");

        Ok(())
    }
}
