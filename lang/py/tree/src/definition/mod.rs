use crate::{alias::PYAlias, class::PYClass, PYDoc, PYIdentifier};

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

    pub fn doc(&self) -> &Option<PYDoc> {
        match self {
            Self::Alias(alias) => &alias.doc,
            Self::Class(class) => &class.doc,
        }
    }
}

impl From<PYClass> for PYDefinition {
    fn from(class: PYClass) -> Self {
        PYDefinition::Class(class)
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
                doc: None,
                name: "Name".into(),
                descriptor: PYDescriptor::Primitive(PYPrimitive::Boolean),
                references: vec![],
            })
            .name(),
            "Name".into(),
        );

        assert_eq!(
            *PYDefinition::Class(PYClass {
                doc: None,
                name: "Name".into(),
                extensions: vec![],
                properties: vec![],
                references: vec![],
            })
            .name(),
            "Name".into(),
        );
    }

    #[test]
    fn test_doc() {
        assert_eq!(
            *PYDefinition::Alias(PYAlias {
                doc: Some(PYDoc("Hello, world!".into())),
                name: "Name".into(),
                descriptor: PYDescriptor::Primitive(PYPrimitive::Boolean),
                references: vec![],
            })
            .doc(),
            Some(PYDoc("Hello, world!".into())),
        );

        assert_eq!(
            *PYDefinition::Class(PYClass {
                doc: Some(PYDoc("Hello, world!".into())),
                name: "Name".into(),
                extensions: vec![],
                properties: vec![],
                references: vec![],
            })
            .doc(),
            Some(PYDoc("Hello, world!".into())),
        );
    }
}
