use genotype_lang_py_tree::*;

use crate::visitor::PYVisitor;

use super::PYTraverse;

impl PYTraverse for PYImportName {
    fn traverse(&mut self, visitor: &mut dyn PYVisitor) {
        visitor.visit_import_name(self);
        match self {
            PYImportName::Name(name) => name.traverse(visitor),

            PYImportName::Alias(name, alias) => {
                name.traverse(visitor);
                alias.traverse(visitor);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visitor::mock::*;
    use genotype_lang_py_tree::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse_name() {
        let mut visitor = PYMockVisitor::new();
        let identifier = PYIdentifier("Name".into());
        let mut import_name = PYImportName::Name(identifier.clone());
        import_name.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                PYMockVisited::ImportName(import_name.clone()),
                PYMockVisited::Identifier(identifier.clone()),
            ]
        );
    }

    #[test]
    fn test_traverse_alias() {
        let mut visitor = PYMockVisitor::new();
        let identifier = PYIdentifier("Name".into());
        let alias = PYIdentifier("Alias".into());
        let mut import_name = PYImportName::Alias(identifier.clone(), alias.clone());
        import_name.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                PYMockVisited::ImportName(import_name.clone()),
                PYMockVisited::Identifier(identifier.clone()),
                PYMockVisited::Identifier(alias.clone()),
            ]
        );
    }
}
