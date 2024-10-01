use genotype_project::{module::GTProjectModule, path::GTProjectPath};

use crate::path::GTProjectOutPath;

pub trait GTProjectModuleOut {
    fn generate(
        root: &GTProjectPath,
        module: &GTProjectModule,
        out: &GTProjectOutPath,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;
}
