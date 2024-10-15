use pest::iterators::Pair;

use crate::*;

use super::GTObject;

impl GTObject {
    pub fn parse(pair: Pair<'_, Rule>, resolve: &mut GTResolve) -> GTNodeParseResult<Self> {
        let span = pair.as_span().into();
        let mut object = GTObject {
            span,
            extensions: vec![],
            properties: vec![],
        };

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::required_property | Rule::optional_property => {
                    object.properties.push(GTProperty::parse(pair, resolve)?);
                }

                Rule::extension_property => {
                    object.extensions.push(GTExtension::parse(pair, resolve)?);
                }

                _ => return Err(GTNodeParseError::Internal(object.span, GTNode::Object)),
            }
        }

        Ok(object)
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser;
    use pretty_assertions::assert_eq;
    use std::collections::HashSet;

    use crate::*;

    #[test]
    fn test_parse() {
        let mut pairs = GenotypeParser::parse(Rule::object, "{ hello: string }").unwrap();
        let mut resove = GTResolve::new();
        assert_eq!(
            GTObject::parse(pairs.next().unwrap(), &mut resove).unwrap(),
            GTObject {
                span: (0, 17).into(),
                extensions: vec![],
                properties: vec![GTProperty {
                    span: (2, 15).into(),
                    doc: None,
                    name: GTKey((2, 7).into(), "hello".into()),
                    descriptor: GTPrimitive::String((9, 15).into()).into(),
                    required: true,
                }]
            }
        );
    }

    #[test]
    fn test_parse_deps_base() {
        let source_code = crate::GTSourceCode::new(
            "module.type".into(),
            r#"Order = {
                book: book/Book
                user: ./misc/user/User
            }"#
            .into(),
        );
        let parse = GTModule::parse(source_code).unwrap();
        assert_eq!(
            parse.resolve.deps,
            HashSet::from_iter(vec![
                GTPath::parse((32, 36).into(), "book").unwrap(),
                GTPath::parse((64, 75).into(), "./misc/user").unwrap(),
            ])
        );
    }

    #[test]
    fn test_parse_deps_normalize() {
        let source_code = crate::GTSourceCode::new(
            "module.type".into(),
            r#"Order = {
                book: book/Book
                user: ./misc/../misc/./user/User
            }"#
            .into(),
        );
        let parse = GTModule::parse(source_code).unwrap();
        assert_eq!(
            parse.resolve.deps,
            HashSet::from_iter(vec![
                GTPath::parse((32, 36).into(), "book").unwrap(),
                GTPath::parse((64, 85).into(), "./misc/user").unwrap(),
            ])
        );
    }
}
