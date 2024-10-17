use genotype_parser::{GTIdentifier, GTObjectName};
use genotype_visitor::visitor::GTVisitor;

pub struct GTProjectVistor {
    pub object_aliases: Vec<GTIdentifier>,
}

impl GTProjectVistor {
    pub fn new() -> Self {
        GTProjectVistor {
            object_aliases: vec![],
        }
    }
}

impl GTVisitor for GTProjectVistor {
    fn visit_object_name(&mut self, name: &mut genotype_parser::GTObjectName) {
        match &name {
            GTObjectName::Anonymous(span, parent) => {
                let identifier = parent.to_identifier(span.clone());
                *name = GTObjectName::Alias(identifier.clone(), parent.clone());
                self.object_aliases.push(identifier);
            }

            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_parser::{GTIdentifier, GTObject, GTObjectName, GTObjectNameParent};
    use genotype_visitor::traverse::GTTraverse;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_named() {
        let mut visitor = GTProjectVistor::new();
        let name = GTObjectName::Named(GTIdentifier::new((0, 5).into(), "Hello".into()));
        let mut name_copy = name.clone();
        name_copy.traverse(&mut visitor);
        assert_eq!(name, name_copy);
        assert_eq!(visitor.object_aliases, vec![]);
    }

    #[test]
    fn test_anynonymous() {
        let mut visitor = GTProjectVistor::new();
        let mut name = GTObjectName::Anonymous(
            (5, 10).into(),
            GTObjectNameParent::Alias(GTIdentifier::new((0, 5).into(), "Hello".into())),
        );
        name.traverse(&mut visitor);
        assert_eq!(
            name,
            GTObjectName::Alias(
                GTIdentifier::new((5, 10).into(), "HelloObj".into()),
                GTObjectNameParent::Alias(GTIdentifier::new((0, 5).into(), "Hello".into())),
            )
        );
        assert_eq!(
            visitor.object_aliases,
            vec![GTIdentifier::new((5, 10).into(), "HelloObj".into())]
        );
    }

    #[test]
    fn test_alias() {
        let mut visitor = GTProjectVistor::new();
        let name = GTObjectName::Alias(
            GTIdentifier::new((5, 10).into(), "HelloObj".into()),
            GTObjectNameParent::Alias(GTIdentifier::new((0, 5).into(), "Hello".into())),
        );
        let mut name_copy = name.clone();
        name_copy.traverse(&mut visitor);
        assert_eq!(name, name_copy);
        assert_eq!(visitor.object_aliases, vec![]);
    }

    #[test]
    fn test_object() {
        let mut visitor = GTProjectVistor::new();
        let name = GTObjectName::Anonymous(
            (5, 10).into(),
            GTObjectNameParent::Alias(GTIdentifier::new((0, 5).into(), "Hello".into())),
        );
        let mut object = GTObject {
            span: (0, 0).into(),
            name,
            extensions: vec![],
            properties: vec![],
        };
        object.traverse(&mut visitor);
        assert_eq!(
            object.name,
            GTObjectName::Alias(
                GTIdentifier::new((5, 10).into(), "HelloObj".into()),
                GTObjectNameParent::Alias(GTIdentifier::new((0, 5).into(), "Hello".into())),
            )
        );
        assert_eq!(
            visitor.object_aliases,
            vec![GTIdentifier::new((5, 10).into(), "HelloObj".into())]
        );
    }
}
