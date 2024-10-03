use genotype_parser::tree::*;
use genotype_visitor::visitor::GTVisitor;

use crate::path::GTProjectPath;

pub struct GTProjectLoadVisitor {
    deps: Vec<GTPath>,
    pub exports: Vec<GTIdentifier>,
}

impl GTProjectLoadVisitor {
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
            .map(|path| parent.resolve(path.as_str()))
            .collect::<Result<_, _>>()?;
        Ok(paths)
    }
}

impl GTVisitor for GTProjectLoadVisitor {
    fn visit_alias(&mut self, alias: &mut GTAlias) {
        self.exports.push(alias.name.clone());
    }

    fn visit_import(&mut self, import: &mut GTImport) {
        self.deps.push(import.path.clone());
    }

    fn visit_inline_import(&mut self, project: &mut GTInlineImport) {
        self.deps.push(project.path.clone());
    }
}
