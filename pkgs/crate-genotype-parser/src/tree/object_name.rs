use crate::prelude::internal::*;

/// A name assigned to an object. It can be explicitly named,
#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum GtObjectName {
    /// Explicately given name.
    Named(#[visit] GtIdentifier),
    /// Name given to an anonymous object. It includes the name parent that helped
    /// to build the object name.
    Alias(#[visit] GtIdentifier, GtObjectNameParent),
}

impl GtObjectName {
    pub fn to_identifier(&self) -> GtIdentifier {
        match self {
            GtObjectName::Named(identifier) => identifier.clone(),
            GtObjectName::Alias(identifier, _) => identifier.clone(),
        }
    }
}

impl From<GtIdentifier> for GtObjectName {
    fn from(value: GtIdentifier) -> Self {
        GtObjectName::Named(value)
    }
}

impl From<String> for GtObjectName {
    fn from(value: String) -> Self {
        GtObjectName::Named(GtIdentifier::new(Default::default(), value.into()))
    }
}

/// The kind of parent that builds the object name.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum GtObjectNameParent {
    /// An alias parent.
    Alias(GtIdentifier),
    /// A property parent that starts with the root alias name and the keys path.
    Property(GtIdentifier, Vec<GtKey>),
}

impl GtObjectNameParent {
    pub fn to_identifier(&self, span: GtSpan) -> GtIdentifier {
        match self {
            GtObjectNameParent::Alias(identifier) => {
                GtIdentifier::new(span, format!("{}Obj", identifier.1).into())
            }

            GtObjectNameParent::Property(identifier, keys) => {
                let keys = keys
                    .iter()
                    .map(|key| Self::capitalize(key.1.as_ref()))
                    .collect::<Vec<_>>()
                    .join("");
                GtIdentifier::new(span, format!("{}{}", identifier.1, keys).into())
            }
        }
    }

    fn capitalize(str: &str) -> String {
        let mut chars = str.chars();
        match chars.next() {
            None => String::new(),
            Some(first_char) => first_char.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parent_to_identifier_alias() {
        let parent = GtObjectNameParent::Alias(GtIdentifier::new((0, 5).into(), "Name".into()));
        assert_eq!(
            parent.to_identifier((5, 10).into()),
            GtIdentifier::new((5, 10).into(), "NameObj".into())
        );
    }

    #[test]
    fn test_parent_to_identifier_property() {
        let parent = GtObjectNameParent::Property(
            GtIdentifier::new((0, 5).into(), "User".into()),
            vec![
                GtKey::new((5, 10).into(), "name".into()),
                GtKey::new((10, 15).into(), "first".into()),
            ],
        );
        assert_eq!(
            parent.to_identifier((15, 20).into()),
            GtIdentifier::new((15, 20).into(), "UserNameFirst".into())
        );
    }
}
