use std::path::PathBuf;

use genotype_project::module::GTProjectModule;

pub trait GTProjectModuleOut {
    fn generate(
        root: &PathBuf,
        module: &GTProjectModule,
        out: &PathBuf,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;
}
