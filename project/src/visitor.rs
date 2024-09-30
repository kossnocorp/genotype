use genotype_parser::tree::{import::GTImport, reference::GTReference};
use genotype_visitor::visitor::GTVisitor;

pub struct GTProjectVisitor {
    pub deps: Vec<String>,
}

impl GTVisitor for GTProjectVisitor {
    fn visit_import(&mut self, import: &GTImport) {
        self.deps.push(import.path.clone());
    }

    fn visit_reference(&mut self, project: &GTReference) {
        self.deps.push(project.path.clone());
    }
}
