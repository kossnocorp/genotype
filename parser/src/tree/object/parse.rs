use pest::iterators::Pair;

use crate::*;

use super::GTObject;

impl GTObject {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> GTNodeParseResult<Self> {
        let span: GTSpan = pair.as_span().into();

        let name = context.name_object(span.clone())?;

        // It is an explicitely named object, so we need to add an anonymous parent so following
        // children don't get the object name.
        let named = matches!(name, GTObjectName::Named(_));
        if named {
            context.enter_parent(GTContextParent::Anonymous);
        }

        let mut object = GTObject {
            span: span.clone(),
            name,
            extensions: vec![],
            properties: vec![],
        };

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::required_property | Rule::optional_property => {
                    object.properties.push(GTProperty::parse(pair, context)?);
                }

                Rule::extension_property => {
                    object.extensions.push(GTExtension::parse(pair, context)?);
                }

                _ => return Err(GTParseError::Internal(object.span, GTNode::Object)),
            }
        }

        if named {
            context.exit_parent(span, GTNode::Object)?;
        }

        Ok(object)
    }
}

#[cfg(test)]
mod tests {
    use indexmap::IndexSet;
    use miette::NamedSource;
    use pest::Parser;
    use pretty_assertions::assert_eq;

    use crate::*;

    #[test]
    fn test_parse() {
        let mut pairs = GenotypeParser::parse(Rule::object, "{ hello: string }").unwrap();
        let mut context = GTContext {
            module_id: "module".into(),
            resolve: GTModuleResolve::new(),
            parents: vec![GTContextParent::Alias(GTIdentifier::new(
                (0, 5).into(),
                "Hello".into(),
            ))],
            claimed_names: Default::default(),
        };
        assert_eq!(
            GTObject::parse(pairs.next().unwrap(), &mut context).unwrap(),
            GTObject {
                span: (0, 17).into(),
                name: GTObjectName::Named(GTIdentifier::new((0, 5).into(), "Hello".into())),
                extensions: vec![],
                properties: vec![GTProperty {
                    span: (2, 15).into(),
                    doc: None,
                    attributes: vec![],
                    name: GTKey((2, 7).into(), "hello".into()),
                    descriptor: GTPrimitive::String((9, 15).into()).into(),
                    required: true,
                }]
            }
        );
    }

    #[test]
    fn test_parse_deps_base() {
        let source_code = NamedSource::new(
            "module.type",
            r#"Order = {
                book: book/Book
                user: ./misc/user/User
            }"#
            .into(),
        );
        let parse = GTModule::parse("module".into(), source_code).unwrap();
        assert_eq!(
            parse.resolve.deps,
            IndexSet::<_, std::collections::hash_map::RandomState>::from_iter(vec![
                GTPath::parse((32, 36).into(), "book").unwrap(),
                GTPath::parse((64, 75).into(), "./misc/user").unwrap(),
            ])
        );
    }

    #[test]
    fn test_parse_deps_normalize() {
        let source_code = NamedSource::new(
            "module.type",
            r#"Order = {
                book: book/Book
                user: ./misc/../misc/./user/User
            }"#
            .into(),
        );
        let parse = GTModule::parse("module".into(), source_code).unwrap();
        assert_eq!(
            parse.resolve.deps,
            IndexSet::<_, std::collections::hash_map::RandomState>::from_iter(vec![
                GTPath::parse((32, 36).into(), "book").unwrap(),
                GTPath::parse((64, 85).into(), "./misc/user").unwrap(),
            ])
        );
    }

    #[test]
    fn test_parse_name() {
        let mut pairs = GenotypeParser::parse(Rule::object, "{ hello: string }").unwrap();
        let mut context = GTContext {
            module_id: "module".into(),
            resolve: GTModuleResolve::new(),
            parents: vec![
                GTContextParent::Alias(GTIdentifier::new((0, 5).into(), "Hello".into())),
                GTContextParent::Anonymous,
            ],
            claimed_names: Default::default(),
        };
        assert_eq!(
            GTObject::parse(pairs.next().unwrap(), &mut context).unwrap(),
            GTObject {
                span: (0, 17).into(),
                name: GTObjectName::Alias(
                    GTIdentifier::new((0, 17).into(), "HelloObj".into()),
                    GTObjectNameParent::Alias(GTIdentifier::new((0, 5).into(), "Hello".into()))
                ),
                extensions: vec![],
                properties: vec![GTProperty {
                    span: (2, 15).into(),
                    doc: None,
                    attributes: vec![],
                    name: GTKey((2, 7).into(), "hello".into()),
                    descriptor: GTPrimitive::String((9, 15).into()).into(),
                    required: true,
                }]
            }
        );
    }
}
