use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum PYDefinition {
    Alias(PYAlias),
    Class(PYClass),
    Newtype(PYNewtype),
    Embed(PYEmbedDefinition),
}

impl PYDefinition {
    pub fn name(&self) -> &PYIdentifier {
        match self {
            Self::Alias(alias) => &alias.name,
            Self::Class(class) => &class.name,
            Self::Newtype(newtype) => &newtype.name,
            Self::Embed(embed) => &embed.name,
        }
    }

    pub fn doc(&self) -> &Option<PYDoc> {
        match self {
            Self::Alias(alias) => &alias.doc,
            Self::Class(class) => &class.doc,
            Self::Newtype(newtype) => &newtype.doc,
            Self::Embed(_) => &None,
        }
    }

    pub fn references(&self) -> Vec<&PYIdentifier> {
        match self {
            Self::Alias(alias) => alias.references.iter().collect(),
            Self::Class(class) => class.references.iter().collect(),
            Self::Newtype(_) | Self::Embed(_) => vec![],
        }
    }
}

impl GtlDefinition for PYDefinition {}

impl From<PYClass> for PYDefinition {
    fn from(class: PYClass) -> Self {
        PYDefinition::Class(class)
    }
}

impl From<PYAlias> for PYDefinition {
    fn from(alias: PYAlias) -> Self {
        PYDefinition::Alias(alias)
    }
}

impl From<PYNewtype> for PYDefinition {
    fn from(newtype: PYNewtype) -> Self {
        PYDefinition::Newtype(newtype)
    }
}

impl From<PYEmbedDefinition> for PYDefinition {
    fn from(embed: PYEmbedDefinition) -> Self {
        PYDefinition::Embed(embed)
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

        assert_eq!(
            *PYDefinition::Embed(PYEmbedDefinition {
                name: "Name".into(),
                embed: Default::default()
            })
            .name(),
            "Name".into()
        )
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

        assert_eq!(
            *PYDefinition::Embed(PYEmbedDefinition {
                name: "Name".into(),
                embed: Default::default()
            })
            .doc(),
            None
        );
    }
}
