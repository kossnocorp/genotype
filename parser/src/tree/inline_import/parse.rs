use pest::iterators::Pair;

use crate::{
    diagnostic::error::GTNodeParseError,
    parser::Rule,
    tree::{identifier::GTIdentifier, path::GTPath},
    GTContext,
};

use super::GTInlineImport;

impl GTInlineImport {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> Result<Self, GTNodeParseError> {
        let span = pair.as_span().into();
        let (path, (name_span, name)) = GTPath::split_parse(pair)?;

        context.resolve.deps.insert(path.clone());

        Ok(GTInlineImport {
            span,
            path,
            name: GTIdentifier::new(name_span, name),
        })
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
        let mut pairs =
            GenotypeParser::parse(Rule::inline_import, "./path/to/module/Name").unwrap();
        assert_eq!(
            GTInlineImport::parse(pairs.next().unwrap(), &mut GTContext::new()).unwrap(),
            GTInlineImport {
                span: (0, 21).into(),
                name: GTIdentifier::new((17, 21).into(), "Name".into()),
                path: GTPath::parse((0, 16).into(), "./path/to/module").unwrap(),
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
