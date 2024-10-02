use genotype_parser::tree::identifier::GTIdentifier;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTIdentifier {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor) {
        visitor.visit_identifier(self);
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
        let mut identifier = GTIdentifier("Name".into());
        identifier.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![GTMockVisited::Identifier(identifier.clone()),]
        );
    }
}
