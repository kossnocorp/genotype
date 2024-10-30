use genotype_lang_py_tree::reference::PYReference;

use crate::visitor::PYVisitor;

use super::PYTraverse;

impl PYTraverse for PYReference {
    fn traverse(&mut self, visitor: &mut dyn PYVisitor) {
        visitor.visit_reference(self);
        self.identifier.traverse(visitor);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visitor::mock::*;
    use genotype_lang_py_tree::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse() {
        let mut visitor = PYMockVisitor::new();
        let identifier = PYIdentifier("Name".into());
        let mut reference = PYReference {
            identifier: identifier.clone(),
            forward: false,
        };
        reference.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                PYMockVisited::Reference(reference.clone()),
                PYMockVisited::Identifier(identifier.clone()),
            ]
        );
    }
}