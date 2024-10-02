use std::{
    fs::read_to_string,
    hash::{Hash, Hasher},
};

use genotype_parser::tree::{identifier::GTIdentifier, module::GTModule};
use genotype_visitor::{traverse::GTTraverse, visitor::GTVisitor};

use crate::{path::GTProjectPath, visitor::GTProjectVisitor};

#[derive(Debug, PartialEq, Clone)]
pub struct GTProjectModule {
    pub path: GTProjectPath,
    pub module: GTModule,
    pub deps: Vec<GTProjectPath>,
    pub exports: Vec<GTIdentifier>,
}

impl GTProjectModule {
    pub fn load(path: GTProjectPath) -> Result<Self, Box<dyn std::error::Error>> {
        let code = read_to_string(&path)?;
        let module = GTModule::parse(code)?;

        let mut visitor = GTProjectVisitor::new();
        module.traverse(&mut visitor);

        let deps = visitor.deps(&path)?;
        let exports = visitor.exports;

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

impl Hash for GTProjectModule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}
