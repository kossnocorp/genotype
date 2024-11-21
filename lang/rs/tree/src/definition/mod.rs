use genotype_parser::GTDefinitionId;

use crate::{alias::RSAlias, r#struct::RSStruct, RSDoc, RSEnum, RSIdentifier, RSNewtype};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum RSDefinition {
    Alias(RSAlias),
    Struct(RSStruct),
    Enum(RSEnum),
    Newtype(RSNewtype),
}

impl RSDefinition {
    pub fn name(&self) -> &RSIdentifier {
        match self {
            Self::Alias(alias) => &alias.name,
            Self::Struct(r#struct) => &r#struct.name,
            Self::Enum(r#enum) => &r#enum.name,
            Self::Newtype(newtype) => &newtype.name,
        }
    }

    pub fn doc(&self) -> &Option<RSDoc> {
        match self {
            Self::Alias(alias) => &alias.doc,
            Self::Struct(r#struct) => &r#struct.doc,
            Self::Enum(r#enum) => &r#enum.doc,
            Self::Newtype(newtype) => &newtype.doc,
        }
    }

    pub fn id(&self) -> &GTDefinitionId {
        match self {
            Self::Alias(alias) => &alias.id,
            Self::Struct(r#struct) => &r#struct.id,
            Self::Enum(r#enum) => &r#enum.id,
            Self::Newtype(newtype) => &newtype.id,
        }
    }
}

impl From<RSStruct> for RSDefinition {
    fn from(r#struct: RSStruct) -> Self {
        RSDefinition::Struct(r#struct)
    }
}

impl From<RSEnum> for RSDefinition {
    fn from(r#enum: RSEnum) -> Self {
        RSDefinition::Enum(r#enum)
    }
}

impl From<RSNewtype> for RSDefinition {
    fn from(newtype: RSNewtype) -> Self {
        RSDefinition::Newtype(newtype)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use genotype_parser::GTDefinitionId;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_name() {
        assert_eq!(
            *RSDefinition::Alias(RSAlias {
                id: GTDefinitionId("module".into(), "Name".into()),
                doc: None,
                name: "Name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::Boolean),
            })
            .name(),
            "Name".into(),
        );

        assert_eq!(
            *RSDefinition::Struct(RSStruct {
                id: GTDefinitionId("module".into(), "Name".into()),
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
                id: GTDefinitionId("module".into(), "Name".into()),
                doc: Some("Hello, world!".into()),
                name: "Name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::Boolean),
            })
            .doc(),
            Some("Hello, world!".into()),
        );

        assert_eq!(
            *RSDefinition::Struct(RSStruct {
                id: GTDefinitionId("module".into(), "Name".into()),
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
