use genotype_parser::tree::{
    identifier::GTIdentifier, import::GTImport, inline_import::GTInlineImport, path::GTPath,
};
use genotype_visitor::visitor::GTVisitor;

use crate::path::GTProjectPath;

pub struct GTProjectVisitor {
    deps: Vec<GTPath>,
    pub exports: Vec<GTIdentifier>,
}

impl GTProjectVisitor {
    pub fn new() -> Self {
        Self {
            deps: vec![],
            exports: vec![],
        }
    }

    pub fn deps(
        &self,
        parent: &GTProjectPath,
    ) -> Result<Vec<GTProjectPath>, Box<dyn std::error::Error>> {
        let paths = self
            .deps
            .iter()
            .map(|path| parent.resolve(&path.0))
            .collect::<Result<_, _>>()?;
        Ok(paths)
    }
}

impl GTVisitor for GTProjectVisitor {
    fn visit_alias(&mut self, alias: &genotype_parser::tree::alias::GTAlias) {
        self.exports.push(alias.name.clone());
    }

    fn visit_import(&mut self, import: &GTImport) {
        self.deps.push(import.path.clone());
    }

    fn visit_inline_import(&mut self, project: &GTInlineImport) {
        self.deps.push(project.path.clone());
    }
}
