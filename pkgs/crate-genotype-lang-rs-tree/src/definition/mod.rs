use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum RsDefinition {
    Alias(#[visit] RsAlias),
    Struct(#[visit] RsStruct),
    Enum(#[visit] RsEnum),
}

impl RsDefinition {
    pub fn name(&self) -> &RsIdentifier {
        match self {
            Self::Alias(alias) => &alias.name,
            Self::Struct(r#struct) => &r#struct.name,
            Self::Enum(r#enum) => &r#enum.name,
        }
    }

    pub fn doc(&self) -> &Option<RsDoc> {
        match self {
            Self::Alias(alias) => &alias.doc,
            Self::Struct(r#struct) => &r#struct.doc,
            Self::Enum(r#enum) => &r#enum.doc,
        }
    }

    pub fn id(&self) -> &GtDefinitionId {
        match self {
            Self::Alias(alias) => &alias.id,
            Self::Struct(r#struct) => &r#struct.id,
            Self::Enum(r#enum) => &r#enum.id,
        }
    }
}

impl GtlDefinition for RsDefinition {}

impl From<RsStruct> for RsDefinition {
    fn from(r#struct: RsStruct) -> Self {
        RsDefinition::Struct(r#struct)
    }
}

impl From<RsEnum> for RsDefinition {
    fn from(r#enum: RsEnum) -> Self {
        RsDefinition::Enum(r#enum)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use genotype_parser::GtDefinitionId;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_name() {
        assert_eq!(
            *RsDefinition::Alias(RsAlias {
                id: GtDefinitionId("module".into(), "Name".into()),
                doc: None,
                name: "Name".into(),
                descriptor: RsDescriptor::Primitive(RsPrimitive::Boolean),
            })
            .name(),
            "Name".into(),
        );

        assert_eq!(
            *RsDefinition::Struct(RsStruct {
                id: GtDefinitionId("module".into(), "Name".into()),
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
            *RsDefinition::Alias(RsAlias {
                id: GtDefinitionId("module".into(), "Name".into()),
                doc: Some("Hello, world!".into()),
                name: "Name".into(),
                descriptor: RsDescriptor::Primitive(RsPrimitive::Boolean),
            })
            .doc(),
            Some("Hello, world!".into()),
        );

        assert_eq!(
            *RsDefinition::Struct(RsStruct {
                id: GtDefinitionId("module".into(), "Name".into()),
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
