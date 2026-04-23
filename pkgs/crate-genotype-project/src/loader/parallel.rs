use crate::prelude::internal::*;
use std::sync::mpsc;

/// Parallel project loader trait. It implements the project loader trait with parallel loading
/// capabilities. It is used by the system project runtime.
pub trait GtpLoaderParallel {}

impl<Type: GtpLoaderParallel + GtpSource + ?Sized> GtpLoader for Type {
    /// Loads all project modules in parallel.
    fn load_all_modules(&self, project: &mut GtProject) -> Result<()> {
        println!(
            ">>>>>>>>>>>> project.entry_path(): {}",
            project.entry_path().display()
        );
        println!(
            ">>>>>>>>>>>> project.root_path(): {}",
            project.root_path().display()
        );
        let entry_path = project.entry_path().to_cwd_path(project.root_path());
        println!(">>>>>>>>>>>> Entry path: {}", entry_path.display());
        let src_path = project.src_path();
        let module_paths = self
            .glob(&entry_path)?
            .into_iter()
            .map(|path| path.to_module_path(&src_path))
            .collect::<Vec<_>>();

        ensure!(
            module_paths.len() > 0,
            "no module files found for entry pattern '{}'",
            entry_path.display()
        );

        // let (tx, rx) = mpsc::channel();

        println!("-------------------------");
        println!("Module paths: {:?}", module_paths);

        bail!("WIP");

        Ok(())
    }
}
