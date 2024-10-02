use genotype_parser::tree::inline_import::GTInlineImport;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTInlineImport {
    fn traverse(&self, visitor: &mut dyn GTVisitor) {
        visitor.visit_inline_import(self);
        self.name.traverse(visitor);
        self.path.traverse(visitor);
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
        let import = GTInlineImport {
            path: GTPath("./path/to/module".into()),
            name: "Name".into(),
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
