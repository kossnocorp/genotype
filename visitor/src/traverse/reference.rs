use genotype_parser::tree::reference::GTReference;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTReference {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor) {
        visitor.visit_reference(self);
        self.1.traverse(visitor);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visitor::mock::*;
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse() {
        let mut visitor = GTMockVisitor::new();
        let identifier = GTIdentifier::new((0, 0).into(), "Name".into());
        let mut reference = GTReference((0, 0).into(), identifier.clone().into());
        reference.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::Reference(reference.clone()),
                GTMockVisited::Identifier(identifier.clone()),
            ]
        );
    }
}
