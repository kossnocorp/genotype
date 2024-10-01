use std::collections::HashSet;

use genotype_parser::tree::{import::GTImport, inline_import::GTInlineImport};
use genotype_visitor::visitor::GTVisitor;

use crate::path::GTProjectPath;

pub struct GTProjectVisitor {
    deps: HashSet<String>,
}

impl GTProjectVisitor {
    pub fn new() -> Self {
        Self {
            deps: HashSet::new(),
        }
    }

    pub fn deps(
        &self,
        parent: &GTProjectPath,
    ) -> Result<HashSet<GTProjectPath>, Box<dyn std::error::Error>> {
        let paths = self
            .deps
            .iter()
            .map(|path| parent.resolve(path))
            .collect::<Result<_, _>>()?;
        Ok(paths)
    }
}

impl GTVisitor for GTProjectVisitor {
    fn visit_import(&mut self, import: &GTImport) {
        self.deps.insert(import.path.clone());
    }

    fn visit_inline_import(&mut self, project: &GTInlineImport) {
        self.deps.insert(project.path.clone());
    }
}
