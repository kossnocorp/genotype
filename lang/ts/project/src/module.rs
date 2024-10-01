use std::hash::{Hash, Hasher};

use genotype_lang_ts_converter::module::convert_to_ts_module;
use genotype_lang_ts_tree::module::TSModule;
use genotype_project::{module::GTProjectModule, path::GTProjectPath};

#[derive(Debug, Clone)]
pub struct TSProjectModule {
    pub path: GTProjectPath,
    pub module: TSModule,
}

impl From<GTProjectModule> for TSProjectModule {
    fn from(module: GTProjectModule) -> Self {
        Self {
            path: module.path.clone(),
            module: convert_to_ts_module(module.module),
        }
    }
}

impl PartialEq for TSProjectModule {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for TSProjectModule {}

impl Hash for TSProjectModule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}
