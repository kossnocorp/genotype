use genotype_parser::tree::import_name::GTImportName;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTImportName {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor) {
        visitor.visit_import_name(self);
        match self {
            GTImportName::Name(name) => name.traverse(visitor),
            GTImportName::Alias(name, alias) => {
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
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse_name() {
        let mut visitor = GTMockVisitor::new();
        let identifier = GTIdentifier::new((0, 0).into(), "Name".into());
        let mut import_name = GTImportName::Name(identifier.clone());
        import_name.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::ImportName(import_name.clone()),
                GTMockVisited::Identifier(identifier.clone()),
            ]
        );
    }

    #[test]
    fn test_traverse_alias() {
        let mut visitor = GTMockVisitor::new();
        let identifier = GTIdentifier::new((0, 0).into(), "Name".into());
        let alias = GTIdentifier::new((0, 0).into(), "Alias".into());
        let mut import_name = GTImportName::Alias(identifier.clone(), alias.clone());
        import_name.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::ImportName(import_name.clone()),
                GTMockVisited::Identifier(identifier.clone()),
                GTMockVisited::Identifier(alias.clone()),
            ]
        );
    }
}
