use crate::prelude::internal::*;

impl GTInlineImport {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> Result<Self, GTParseError> {
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
    use super::*;
    use insta::assert_ron_snapshot;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse() {
        let mut pairs =
            GenotypeParser::parse(Rule::inline_import, "./path/to/module/Name").unwrap();
        assert_eq!(
            GTInlineImport::parse(pairs.next().unwrap(), &mut GTContext::new("module".into()))
                .unwrap(),
            GTInlineImport {
                span: (0, 21).into(),
                name: GTIdentifier::new((17, 21).into(), "Name".into()),
                path: GTPath::parse((0, 16).into(), "./path/to/module").unwrap(),
            }
        );
    }

    #[test]
    fn test_parse_deps_base() {
        let source_code = NamedSource::new(
            "module.type",
            r#"Order: {
                book: book/Book,
                user: ./misc/user/User
            }"#
            .into(),
        );
        let parse = GTModule::parse("module".into(), source_code).unwrap();
        assert_ron_snapshot!(
            parse.resolve.deps,
            @r#"
        [
          GTPath(GTSpan(31, 35), Unresolved, "book"),
          GTPath(GTSpan(64, 75), Unresolved, "./misc/user"),
        ]
        "#
        );
    }

    #[test]
    fn test_parse_deps_normalize() {
        let source_code = NamedSource::new(
            "module.type",
            r#"Order: {
                book: book/Book,
                user: ./misc/../misc/./user/User
            }"#
            .into(),
        );
        let parse = GTModule::parse("module".into(), source_code).unwrap();
        assert_ron_snapshot!(
            parse.resolve.deps,
            @r#"
        [
          GTPath(GTSpan(31, 35), Unresolved, "book"),
          GTPath(GTSpan(64, 85), Unresolved, "./misc/user"),
        ]
        "#
        );
    }
}
