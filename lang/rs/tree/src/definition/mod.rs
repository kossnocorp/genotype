use crate::{alias::RSAlias, r#struct::RSStruct, RSDoc, RSEnum, RSIdentifier};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum RSDefinition {
    Alias(RSAlias),
    Struct(RSStruct),
    Enum(RSEnum),
}

impl RSDefinition {
    pub fn name(&self) -> &RSIdentifier {
        match self {
            Self::Alias(alias) => &alias.name,
            Self::Struct(class) => &class.name,
            Self::Enum(r#enum) => &r#enum.name,
        }
    }

    pub fn doc(&self) -> &Option<RSDoc> {
        match self {
            Self::Alias(alias) => &alias.doc,
            Self::Struct(class) => &class.doc,
            Self::Enum(r#enum) => &r#enum.doc,
        }
    }
}

impl From<RSStruct> for RSDefinition {
    fn from(class: RSStruct) -> Self {
        RSDefinition::Struct(class)
    }
}

impl From<RSEnum> for RSDefinition {
    fn from(r#enum: RSEnum) -> Self {
        RSDefinition::Enum(r#enum)
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
                attributes: vec![],
                name: "Name".into(),
                fields: vec![].into(),
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
                attributes: vec![],
                name: "Name".into(),
                fields: vec![].into(),
            })
            .doc(),
            Some("Hello, world!".into()),
        );
    }
}
