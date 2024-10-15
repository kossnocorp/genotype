use pest::iterators::Pair;

use crate::{
    diagnostic::error::GTNodeParseError,
    parser::Rule,
    tree::{identifier::GTIdentifier, path::GTPath, GTResolve},
};

use super::GTInlineImport;

impl GTInlineImport {
    pub fn parse(pair: Pair<'_, Rule>, resolve: &mut GTResolve) -> Result<Self, GTNodeParseError> {
        let (path, (name_span, name)) = GTPath::split_parse(pair)?;

        resolve.deps.insert(path.clone());

        Ok(GTInlineImport {
            path,
            name: GTIdentifier::new(name_span, name),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use pretty_assertions::assert_eq;

    use crate::*;

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
