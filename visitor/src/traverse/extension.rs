use genotype_parser::tree::GTExtension;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTExtension {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor) {
        visitor.visit_extension(self);
        self.reference.traverse(visitor);
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
        let mut extension = GTExtension {
            span: (0, 0).into(),
            reference: GTIdentifier::new((0, 0).into(), "Name".into()).into(),
        };
        extension.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::Extension(extension.clone()),
                GTMockVisited::Reference(extension.reference.clone()),
                GTMockVisited::Identifier(extension.reference.1.clone()),
            ]
        );
    }
}
