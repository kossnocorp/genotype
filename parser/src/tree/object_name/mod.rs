use crate::*;

#[derive(Debug, PartialEq, Clone)]
pub enum GTObjectName {
    Named(GTIdentifier),
    Anonymous(GTSpan, GTObjectNameParent),
    Alias(GTIdentifier, GTObjectNameParent),
}

impl GTObjectName {}

impl From<GTIdentifier> for GTObjectName {
    fn from(value: GTIdentifier) -> Self {
        GTObjectName::Named(value)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum GTObjectNameParent {
    Alias(GTIdentifier),
    Property(GTIdentifier, Vec<GTKey>),
}

impl GTObjectNameParent {
    pub fn to_identifier(&self, span: GTSpan) -> GTIdentifier {
        match self {
            GTObjectNameParent::Alias(identifier) => {
                GTIdentifier::new(span, format!("{}Obj", identifier.1))
            }

            GTObjectNameParent::Property(identifier, keys) => {
                let keys = keys
                    .iter()
                    .map(|key| Self::capitalize(&key.1))
                    .collect::<Vec<_>>()
                    .join("");
                GTIdentifier::new(span, format!("{}{}", identifier.1, keys))
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
        let parent = GTObjectNameParent::Alias(GTIdentifier::new((0, 5).into(), "Name".into()));
        assert_eq!(
            parent.to_identifier((5, 10).into()),
            GTIdentifier::new((5, 10).into(), "NameObj".into())
        );
    }

    #[test]
    fn test_parent_to_identifier_property() {
        let parent = GTObjectNameParent::Property(
            GTIdentifier::new((0, 5).into(), "User".into()),
            vec![
                GTKey::new((5, 10).into(), "name".into()),
                GTKey::new((10, 15).into(), "first".into()),
            ],
        );
        assert_eq!(
            parent.to_identifier((15, 20).into()),
            GTIdentifier::new((15, 20).into(), "UserNameFirst".into())
        );
    }
}
