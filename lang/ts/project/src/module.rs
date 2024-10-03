use std::{
    hash::{Hash, Hasher},
    path::PathBuf,
};

use genotype_lang_core_project::module::GTProjectModuleOut;
use genotype_lang_ts_converter::module::convert_to_ts_module;
use genotype_lang_ts_tree::module::TSModule;
use genotype_project::module::GTProjectModule;

#[derive(Debug, PartialEq, Clone)]
pub struct TSProjectModule {
    pub path: PathBuf,
    pub module: TSModule,
}

impl GTProjectModuleOut for TSProjectModule {
    fn generate(
        root: &PathBuf,
        module: &GTProjectModule,
        out: &PathBuf,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let path = out
            .as_path()
            .join(
                module
                    .path
                    .as_path()
                    .strip_prefix(root.as_path())?
                    .with_extension("ts"),
            )
            .into();

        Ok(Self {
            path,
            module: convert_to_ts_module(&module.module),
        })
    }
}

impl Hash for TSProjectModule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}
