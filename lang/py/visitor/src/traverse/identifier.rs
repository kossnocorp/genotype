use genotype_lang_py_tree::identifier::PYIdentifier;

use crate::visitor::PYVisitor;

use super::PYTraverse;

impl PYTraverse for PYIdentifier {
    fn traverse(&mut self, visitor: &mut dyn PYVisitor) {
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
        let mut visitor = PYMockVisitor::new();
        let mut identifier = PYIdentifier("Name".into());
        identifier.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![PYMockVisited::Identifier(identifier.clone()),]
        );
    }
}
