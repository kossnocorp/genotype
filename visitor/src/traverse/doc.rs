use genotype_parser::tree::doc::GTDoc;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTDoc {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor) {
        visitor.visit_doc(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visitor::mock::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse() {
        let mut visitor = GTMockVisitor::new();
        let mut doc = GTDoc("Hello, world!".into());
        doc.traverse(&mut visitor);
        assert_eq!(visitor.visited, vec![GTMockVisited::Doc(doc.clone()),]);
    }
}
