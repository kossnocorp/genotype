use crate::{alias::RSAlias, r#struct::RSStruct, RSDoc, RSIdentifier};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum RSDefinition {
    Alias(RSAlias),
    Struct(RSStruct),
}

impl RSDefinition {
    pub fn name(&self) -> &RSIdentifier {
        match self {
            Self::Alias(alias) => &alias.name,
            Self::Struct(class) => &class.name,
        }
    }

    pub fn doc(&self) -> &Option<RSDoc> {
        match self {
            Self::Alias(alias) => &alias.doc,
            Self::Struct(class) => &class.doc,
        }
    }
}

impl From<RSStruct> for RSDefinition {
    fn from(class: RSStruct) -> Self {
        RSDefinition::Struct(class)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_name() {
        assert_eq!(
            *RSDefinition::Alias(RSAlias {
                doc: None,
                name: "Name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::Boolean),
            })
            .name(),
            "Name".into(),
        );

        assert_eq!(
            *RSDefinition::Struct(RSStruct {
                doc: None,
                name: "Name".into(),
                extensions: vec![],
                properties: vec![],
            })
            .name(),
            "Name".into(),
        );
    }

    #[test]
    fn test_doc() {
        assert_eq!(
            *RSDefinition::Alias(RSAlias {
                doc: Some("Hello, world!".into()),
                name: "Name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::Boolean),
            })
            .doc(),
            Some("Hello, world!".into()),
        );

        assert_eq!(
            *RSDefinition::Struct(RSStruct {
                doc: Some("Hello, world!".into()),
                name: "Name".into(),
                extensions: vec![],
                properties: vec![],
            })
            .doc(),
            Some("Hello, world!".into()),
        );
    }
}
