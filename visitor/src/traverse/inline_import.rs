use genotype_parser::tree::inline_import::GTInlineImport;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTInlineImport {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor) {
        visitor.visit_inline_import(self);
        self.name.traverse(visitor);
        self.path.traverse(visitor);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visitor::mock::*;
    use genotype_parser::GTIdentifier;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse() {
        let mut visitor = GTMockVisitor::new();
        let mut import = GTInlineImport {
            path: "./path/to/module".into(),
            name: GTIdentifier::new((0, 0).into(), "Name".into()),
        };
        import.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::InlineImport(import.clone()),
                GTMockVisited::Identifier(import.name.clone()),
                GTMockVisited::Path(import.path.clone()),
            ]
        );
    }
}
