use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum PyDefinition {
    Alias(#[visit] PyAlias),
    Class(#[visit] PyClass),
    Newtype(#[visit] PyNewtype),
    Embed(#[visit] PyEmbedDefinition),
}

impl PyDefinition {
    pub fn name(&self) -> &PyIdentifier {
        match self {
            Self::Alias(alias) => &alias.name,
            Self::Class(class) => &class.name,
            Self::Newtype(newtype) => &newtype.name,
            Self::Embed(embed) => &embed.name,
        }
    }

    pub fn doc(&self) -> &Option<PyDoc> {
        match self {
            Self::Alias(alias) => &alias.doc,
            Self::Class(class) => &class.doc,
            Self::Newtype(newtype) => &newtype.doc,
            Self::Embed(_) => &None,
        }
    }

    pub fn references(&self) -> Vec<&PyIdentifier> {
        match self {
            Self::Alias(alias) => alias.references.iter().collect(),
            Self::Class(class) => class.references.iter().collect(),
            Self::Newtype(_) | Self::Embed(_) => vec![],
        }
    }
}

impl GtlDefinition for PyDefinition {}

impl From<PyClass> for PyDefinition {
    fn from(class: PyClass) -> Self {
        PyDefinition::Class(class)
    }
}

impl From<PyAlias> for PyDefinition {
    fn from(alias: PyAlias) -> Self {
        PyDefinition::Alias(alias)
    }
}

impl From<PyNewtype> for PyDefinition {
    fn from(newtype: PyNewtype) -> Self {
        PyDefinition::Newtype(newtype)
    }
}

impl From<PyEmbedDefinition> for PyDefinition {
    fn from(embed: PyEmbedDefinition) -> Self {
        PyDefinition::Embed(embed)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_name() {
        assert_eq!(
            *PyDefinition::Alias(PyAlias {
                doc: None,
                name: "Name".into(),
                descriptor: PyDescriptor::Primitive(PyPrimitive::Boolean),
                references: vec![],
            })
            .name(),
            "Name".into(),
        );

        assert_eq!(
            *PyDefinition::Class(PyClass {
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
            *PyDefinition::Embed(PyEmbedDefinition {
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
            *PyDefinition::Alias(PyAlias {
                doc: Some(PyDoc("Hello, world!".into())),
                name: "Name".into(),
                descriptor: PyDescriptor::Primitive(PyPrimitive::Boolean),
                references: vec![],
            })
            .doc(),
            Some(PyDoc("Hello, world!".into())),
        );

        assert_eq!(
            *PyDefinition::Class(PyClass {
                doc: Some(PyDoc("Hello, world!".into())),
                name: "Name".into(),
                extensions: vec![],
                properties: vec![],
                references: vec![],
            })
            .doc(),
            Some(PyDoc("Hello, world!".into())),
        );

        assert_eq!(
            *PyDefinition::Embed(PyEmbedDefinition {
                name: "Name".into(),
                embed: Default::default()
            })
            .doc(),
            None
        );
    }
}
