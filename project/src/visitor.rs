use genotype_parser::tree::{import::Import, reference::Reference};
use genotype_visitor::visitor::Visitor;

pub struct ProjectVisitor {
    pub deps: Vec<String>,
}

impl Visitor for ProjectVisitor {
    fn visit_import(&mut self, import: &Import) {
        self.deps.push(import.path.clone());
    }

    fn visit_reference(&mut self, project: &Reference) {
        self.deps.push(project.path.clone());
    }
}
