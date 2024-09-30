use std::{
    fs::read_to_string,
    hash::{Hash, Hasher},
    path::PathBuf,
};

use genotype_parser::tree::module::GTModule;
use genotype_visitor::{traverse::GTTraverse, visitor::GTVisitor};

#[derive(Debug, Clone)]
pub struct GTProjectModule {
    pub path: PathBuf,
    pub module: GTModule,
}

impl TryFrom<PathBuf> for GTProjectModule {
    type Error = Box<dyn std::error::Error>;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let code = read_to_string(&path)?;
        let module = code.try_into()?;
        Ok(GTProjectModule { path, module })
    }
}

impl GTTraverse for GTProjectModule {
    fn traverse(&self, visitor: &mut dyn GTVisitor) {
        self.module.traverse(visitor);
    }
}

impl PartialEq for GTProjectModule {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for GTProjectModule {}

impl Hash for GTProjectModule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}
