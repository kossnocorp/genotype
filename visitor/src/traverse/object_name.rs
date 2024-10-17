use genotype_parser::*;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTObjectName {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor) {
        visitor.visit_object_name(self);

        match self {
            GTObjectName::Named(name) => name.traverse(visitor),

            GTObjectName::Anonymous(_, parent) => match parent {
                GTObjectNameParent::Alias(identifier) => identifier.traverse(visitor),

                GTObjectNameParent::Property(identifier, keys) => {
                    identifier.traverse(visitor);
                    for key in keys {
                        key.traverse(visitor);
                    }
                }
            },

            GTObjectName::Alias(identifier) => identifier.traverse(visitor),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visitor::mock::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse_named() {
        let mut visitor = GTMockVisitor::new();
        let identifier = GTIdentifier::new((0, 0).into(), "Name".into());
        let mut name = GTObjectName::Named(identifier.clone());
        name.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::ObjectName(name),
                GTMockVisited::Identifier(identifier)
            ]
        );
    }

    #[test]
    fn test_traverse_anonymous_alias() {
        let mut visitor = GTMockVisitor::new();
        let identifier = GTIdentifier::new((0, 0).into(), "Name".into());
        let mut name =
            GTObjectName::Anonymous((0, 0).into(), GTObjectNameParent::Alias(identifier.clone()));
        name.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::ObjectName(name),
                GTMockVisited::Identifier(identifier)
            ]
        );
    }

    #[test]
    fn test_traverse_anonymous_property() {
        let mut visitor = GTMockVisitor::new();
        let identifier = GTIdentifier::new((0, 0).into(), "Name".into());
        let key1 = GTKey::new((0, 0).into(), "key1".into());
        let key2 = GTKey::new((0, 0).into(), "key2".into());
        let mut name = GTObjectName::Anonymous(
            (0, 0).into(),
            GTObjectNameParent::Property(identifier.clone(), vec![key1.clone(), key2.clone()]),
        );
        name.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::ObjectName(name),
                GTMockVisited::Identifier(identifier),
                GTMockVisited::Key(key1),
                GTMockVisited::Key(key2)
            ]
        );
    }

    #[test]
    fn test_traverse_alias() {
        let mut visitor = GTMockVisitor::new();
        let identifier = GTIdentifier::new((0, 0).into(), "Name".into());
        let mut name = GTObjectName::Alias(identifier.clone());
        name.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::ObjectName(name),
                GTMockVisited::Identifier(identifier)
            ]
        );
    }
}
