use genotype_parser::tree::{import::GTImport, inline_import::GTInlineImport};
use genotype_visitor::visitor::GTVisitor;

pub struct GTProjectVisitor {
    pub deps: Vec<String>,
}

impl GTVisitor for GTProjectVisitor {
    fn visit_import(&mut self, import: &GTImport) {
        self.deps.push(import.path.clone());
    }

    fn visit_inline_import(&mut self, project: &GTInlineImport) {
        self.deps.push(project.path.clone());
    }
}
