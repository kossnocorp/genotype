use std::{
    collections::HashSet,
    fs::read_to_string,
    hash::{Hash, Hasher},
};

use genotype_parser::tree::{module::GTModule, name::GTName};
use genotype_visitor::{traverse::GTTraverse, visitor::GTVisitor};

use crate::{path::GTProjectPath, visitor::GTProjectVisitor};

#[derive(Debug, Clone)]
pub struct GTProjectModule {
    pub path: GTProjectPath,
    pub module: GTModule,
    pub deps: HashSet<GTProjectPath>,
    pub exports: HashSet<GTName>,
}

impl GTProjectModule {
    pub fn load(path: GTProjectPath) -> Result<Self, Box<dyn std::error::Error>> {
        let code = read_to_string(&path)?;
        let module = GTModule::parse(code)?;
        let exports = HashSet::new();

        let mut visitor = GTProjectVisitor::new();
        module.traverse(&mut visitor);

        let deps = visitor.deps(&path)?;

        Ok(Self {
            path,
            module,
            deps,
            exports,
        })
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
