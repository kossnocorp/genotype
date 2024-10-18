use crate::{alias::PYAlias, class::PYClass, PYIdentifier};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum PYDefinition {
    Alias(PYAlias),
    Class(PYClass),
}

impl PYDefinition {
    pub fn name(&self) -> &PYIdentifier {
        match self {
            Self::Alias(alias) => &alias.name,
            Self::Class(class) => &class.name,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_name() {
        assert_eq!(
            *PYDefinition::Alias(PYAlias {
                name: "Name".into(),
                descriptor: PYDescriptor::Primitive(PYPrimitive::Boolean),
            })
            .name(),
            "Name".into(),
        );

        assert_eq!(
            *PYDefinition::Class(PYClass {
                name: "Name".into(),
                extensions: vec![],
                properties: vec![],
            })
            .name(),
            "Name".into(),
        );
    }
}
