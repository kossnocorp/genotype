use crate::prelude::internal::*;
use heck::ToPascalCase;
use unicode_xid::UnicodeXID;

#[derive(Debug, PartialEq, Clone)]
pub enum RsContextParent {
    /// Alias parent. Defines the name that children can claim unless there is an anonymous parent
    /// between them.
    Alias(RsIdentifier),
    /// Anonymous parent that prevents children from taking the alias name, when they for example
    /// are part of a tuple.
    Anonymous,
    Definition(RsIdentifier),
    Field(RsFieldName),
    EnumVariant(RsIdentifier),
    Hoist,
}

impl RsContextParent {
    pub fn name(&self) -> RsConvertNameSegment {
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

impl From<RsFieldName> for RsContextParent {
    fn from(key: RsFieldName) -> Self {
        RsContextParent::Field(key)
    }
}

impl RsConvertContext {
    pub fn enter_parent(&mut self, parent: RsContextParent) {
        self.parents.push(parent);
    }

    pub fn exit_parent(&mut self) {
        // [TODO]
        self.parents.pop().expect("Expected parent to exist");
    }

    pub fn name_child(&self, name: Option<RsConvertNameSegment>) -> RsIdentifier {
        let mut segments = vec![];
        for parent in self.parents.iter().rev() {
            match parent {
                // [TODO] Kill hoist and variant altogether?
                RsContextParent::Hoist | RsContextParent::EnumVariant(_) => continue,

                _ => {
                    segments.push(parent.name());
                    if let RsContextParent::Definition(_) = parent {
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
    pub fn claim_alias(&mut self) -> Option<RsIdentifier> {
        if let Some(RsContextParent::Alias(identifier)) = self.parents.last() {
            return Some(identifier.clone());
        }
        None
    }

    /// Renders a name segment from the given literal.
    pub fn render_literal_name_segment(literal: &GtLiteralValue) -> String {
        match literal {
            GtLiteralValue::Null => "Null".into(),
            GtLiteralValue::String(value) => value.to_pascal_case(),
            GtLiteralValue::Integer(value) => format!("{value}"),
            GtLiteralValue::Float(value) => {
                format!("{:.}", value).replace('.', "_").to_string()
            }
            GtLiteralValue::Boolean(value) => format!("{value}").to_pascal_case(),
        }
    }
}

pub enum RsConvertNameSegment {
    String(String),
    Literal(GtLiteral),
}

impl RsConvertNameSegment {
    pub fn render(&self, solo: bool) -> String {
        match self {
            RsConvertNameSegment::String(value) => value.to_pascal_case(),

            RsConvertNameSegment::Literal(literal) => {
                let prefix = if solo {
                    "Lit".to_string()
                } else {
                    "".to_string()
                };

                match &literal.value {
                    GtLiteralValue::Null => "Null".into(),
                    GtLiteralValue::String(value) => {
                        let str = value.to_pascal_case();
                        let first = str.chars().next().unwrap_or_default();
                        if first == '_' || UnicodeXID::is_xid_start(first) {
                            str
                        } else {
                            format!("{prefix}{str}")
                        }
                    }
                    GtLiteralValue::Integer(value) => format!("{prefix}{value}"),
                    GtLiteralValue::Float(value) => {
                        format!("{prefix}{:.}", value).replace('.', "_").to_string()
                    }
                    GtLiteralValue::Boolean(value) => format!("{value}").to_pascal_case(),
                }
            }
        }
    }
}

impl From<&str> for RsConvertNameSegment {
    fn from(value: &str) -> Self {
        RsConvertNameSegment::String(value.into())
    }
}

impl From<String> for RsConvertNameSegment {
    fn from(value: String) -> Self {
        RsConvertNameSegment::String(value)
    }
}

impl From<Arc<str>> for RsConvertNameSegment {
    fn from(value: Arc<str>) -> Self {
        RsConvertNameSegment::String(value.to_string())
    }
}

impl From<GtLiteral> for RsConvertNameSegment {
    fn from(value: GtLiteral) -> Self {
        RsConvertNameSegment::Literal(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_name_child() {
        let mut context = RsConvertContext::empty("module".into());
        context.enter_parent(RsContextParent::Definition("Person".into()));
        context.enter_parent(RsContextParent::Field("name".into()));
        assert_eq!(
            context.name_child(Some("value".into())),
            "PersonNameValue".into()
        );
    }

    #[test]
    fn test_name_hoisted_child() {
        let mut context = RsConvertContext::empty("module".into());
        context.enter_parent(RsContextParent::Definition("Person".into()));
        context.enter_parent(RsContextParent::Field("name".into()));
        context.enter_parent(RsContextParent::Hoist);
        context.enter_parent(RsContextParent::Definition("Name".into()));
        assert_eq!(context.name_child(Some("union".into())), "NameUnion".into());
    }

    #[test]
    fn test_name_literal_number_child() {
        let mut context = RsConvertContext::empty("module".into());
        context.enter_parent(RsContextParent::Definition("Person".into()));
        context.enter_parent(RsContextParent::Field("v".into()));
        assert_eq!(
            context.name_child(Some(
                GtLiteral {
                    span: (0, 0).into(),
                    doc: None,
                    attributes: vec![],
                    value: GtLiteralValue::Integer(1),
                }
                .into()
            )),
            "PersonV1".into()
        );
    }

    #[test]
    fn test_name_solo_literal_number_child() {
        let context = RsConvertContext::empty("module".into());
        assert_eq!(
            context.name_child(Some(
                GtLiteral {
                    span: (0, 0).into(),
                    doc: None,
                    attributes: vec![],
                    value: GtLiteralValue::Integer(1),
                }
                .into()
            )),
            "Lit1".into()
        );
    }

    #[test]
    fn test_name_invalid_literal_child() {
        let mut context = RsConvertContext::empty("module".into());
        context.enter_parent(RsContextParent::Definition("Person".into()));
        context.enter_parent(RsContextParent::Field("v".into()));
        assert_eq!(
            context.name_child(Some(
                GtLiteral {
                    span: (0, 0).into(),
                    doc: None,
                    attributes: vec![],
                    value: GtLiteralValue::String("1".into()),
                }
                .into()
            )),
            "PersonV1".into()
        );
    }

    #[test]
    fn test_name_solo_invalid_literal_child() {
        let context = RsConvertContext::empty("module".into());
        assert_eq!(
            context.name_child(Some(
                GtLiteral {
                    span: (0, 0).into(),
                    doc: None,
                    attributes: vec![],
                    value: GtLiteralValue::String("1".into()),
                }
                .into()
            )),
            "Lit1".into()
        );
    }

    #[test]
    fn test_claim_alias_deep() {
        let mut context = RsConvertContext::empty("module".into());
        context.enter_parent(RsContextParent::Alias("Person".into()));
        assert_eq!(context.claim_alias(), Some("Person".into()));
        context.enter_parent(RsContextParent::Anonymous);
        assert_eq!(context.claim_alias(), None);
    }
}
