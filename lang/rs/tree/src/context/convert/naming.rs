use crate::prelude::internal::*;
use heck::ToPascalCase;
use unicode_xid::UnicodeXID;

#[derive(Debug, PartialEq, Clone)]
pub enum RSContextParent {
    /// Alias parent. Defines the name that children can claim unless there is an anonymous parent
    /// between them.
    Alias(RSIdentifier),
    /// Anonymous parent that prevents children from taking the alias name, when they for example
    /// are part of a tuple.
    Anonymous,
    Definition(RSIdentifier),
    Field(RSFieldName),
    EnumVariant(RSIdentifier),
    Hoist,
}

impl RSContextParent {
    pub fn name(&self) -> RSConvertNameSegment {
        match self {
            Self::Alias(identifier) => identifier.0.clone(),
            Self::Definition(identifier) => identifier.0.clone(),
            Self::Field(key) => key.0.clone(),
            Self::EnumVariant(identifier) => identifier.0.clone(),
            Self::Anonymous => panic!("Cannot get name of anonymous parent"),
            Self::Hoist => panic!("Cannot get name of hoist parent"),
        }
        .into()
    }
}

impl From<RSFieldName> for RSContextParent {
    fn from(key: RSFieldName) -> Self {
        RSContextParent::Field(key)
    }
}

impl RSConvertContext {
    pub fn enter_parent(&mut self, parent: RSContextParent) {
        self.parents.push(parent);
    }

    pub fn exit_parent(&mut self) {
        // [TODO]
        self.parents.pop().expect("Expected parent to exist");
    }

    pub fn name_child(&self, name: Option<RSConvertNameSegment>) -> RSIdentifier {
        let mut segments = vec![];
        for parent in self.parents.iter().rev() {
            match parent {
                // [TODO] Kill hoist and variant altogether?
                RSContextParent::Hoist | RSContextParent::EnumVariant(_) => continue,

                _ => {
                    segments.push(parent.name());
                    if let RSContextParent::Definition(_) = parent {
                        break;
                    }
                }
            }
        }
        segments.reverse();
        if let Some(name) = name {
            segments.push(name);
        }

        let solo = segments.len() == 1;
        segments
            .iter()
            .map(|name| name.render(solo))
            .collect::<Vec<_>>()
            .join("")
            .into()
    }

    /// Tries claiming the alias from the parent, i.e. when naming literals:
    ///     HelloWorld = "hello-world"
    pub fn claim_alias(&mut self) -> Option<RSIdentifier> {
        if let Some(RSContextParent::Alias(identifier)) = self.parents.last() {
            return Some(identifier.clone());
        }
        None
    }

    /// Renders a name segment from the given literal.
    pub fn render_literal_name_segment(literal: &GTLiteral) -> String {
        match literal {
            GTLiteral::Null(_) => "Null".into(),
            GTLiteral::String(_, value) => value.to_pascal_case(),
            GTLiteral::Integer(_, value) => format!("{value}"),
            GTLiteral::Float(_, value) => {
                format!("{value}", value = format!("{:.}", value).replace('.', "_"))
            }
            GTLiteral::Boolean(_, value) => format!("{value}").to_pascal_case(),
        }
    }
}

pub enum RSConvertNameSegment {
    String(String),
    Literal(GTLiteral),
}

impl RSConvertNameSegment {
    pub fn render(&self, solo: bool) -> String {
        match self {
            RSConvertNameSegment::String(value) => value.to_pascal_case(),

            RSConvertNameSegment::Literal(literal) => {
                let prefix = if solo {
                    "Lit".to_string()
                } else {
                    "".to_string()
                };

                match literal {
                    GTLiteral::Null(_) => "Null".into(),
                    GTLiteral::String(_, value) => {
                        let str = value.to_pascal_case();
                        let first = str.chars().next().unwrap_or_default();
                        if first == '_' || UnicodeXID::is_xid_start(first) {
                            str
                        } else {
                            format!("{prefix}{str}")
                        }
                    }
                    GTLiteral::Integer(_, value) => format!("{prefix}{value}"),
                    GTLiteral::Float(_, value) => {
                        format!(
                            "{value}",
                            value = format!("{prefix}{:.}", value).replace('.', "_")
                        )
                    }
                    GTLiteral::Boolean(_, value) => format!("{value}").to_pascal_case(),
                }
            }
        }
    }
}

impl From<&str> for RSConvertNameSegment {
    fn from(value: &str) -> Self {
        RSConvertNameSegment::String(value.into())
    }
}

impl From<String> for RSConvertNameSegment {
    fn from(value: String) -> Self {
        RSConvertNameSegment::String(value)
    }
}

impl From<GTLiteral> for RSConvertNameSegment {
    fn from(value: GTLiteral) -> Self {
        RSConvertNameSegment::Literal(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_name_child() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Definition("Person".into()));
        context.enter_parent(RSContextParent::Field("name".into()));
        assert_eq!(
            context.name_child(Some("value".into())),
            "PersonNameValue".into()
        );
    }

    #[test]
    fn test_name_hoisted_child() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Definition("Person".into()));
        context.enter_parent(RSContextParent::Field("name".into()));
        context.enter_parent(RSContextParent::Hoist);
        context.enter_parent(RSContextParent::Definition("Name".into()));
        assert_eq!(context.name_child(Some("union".into())), "NameUnion".into());
    }

    #[test]
    fn test_name_literal_number_child() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Definition("Person".into()));
        context.enter_parent(RSContextParent::Field("v".into()));
        assert_eq!(
            context.name_child(Some(GTLiteral::Integer(Default::default(), 1).into())),
            "PersonV1".into()
        );
    }

    #[test]
    fn test_name_solo_literal_number_child() {
        let context = RSConvertContext::empty("module".into());
        assert_eq!(
            context.name_child(Some(GTLiteral::Integer(Default::default(), 1).into())),
            "Lit1".into()
        );
    }

    #[test]
    fn test_name_invalid_literal_child() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Definition("Person".into()));
        context.enter_parent(RSContextParent::Field("v".into()));
        assert_eq!(
            context.name_child(Some(
                GTLiteral::String(Default::default(), "1".into()).into()
            )),
            "PersonV1".into()
        );
    }

    #[test]
    fn test_name_solo_invalid_literal_child() {
        let context = RSConvertContext::empty("module".into());
        assert_eq!(
            context.name_child(Some(
                GTLiteral::String(Default::default(), "1".into()).into()
            )),
            "Lit1".into()
        );
    }

    #[test]
    fn test_claim_alias_deep() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Alias("Person".into()));
        assert_eq!(context.claim_alias(), Some("Person".into()));
        context.enter_parent(RSContextParent::Anonymous);
        assert_eq!(context.claim_alias(), None);
    }
}
