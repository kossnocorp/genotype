use pest::iterators::Pair;
use std::{
    ffi::OsString,
    path::{Component, Path, PathBuf},
};

use crate::{GTNode, GTNodeParseResult, GTParseError, GTPathModuleId, GTSpan, parser::Rule};

use super::GTPath;

impl GTPath {
    pub fn split_parse(pair: Pair<'_, Rule>) -> GTNodeParseResult<(GTPath, (GTSpan, String))> {
        let span = pair.as_span();
        let span_start = span.start();
        let str = pair.as_str().to_string();
        let else_err = || GTParseError::Internal(span.into(), GTNode::Path);

        let name_index = str.rfind("/").ok_or_else(else_err)?;

        let path_str = str.get(..name_index).ok_or_else(else_err)?;
        let path_span = (span_start, span_start + name_index).into();
        let path = GTPath::parse(path_span, path_str)?;

        let name = str.get(name_index + 1..).ok_or_else(else_err)?;
        let name_span = (span_start + name_index + 1, span.end()).into();

        Ok((path, (name_span, name.into())))
    }

    pub fn parse(span: GTSpan, path: &str) -> GTNodeParseResult<Self> {
        match Self::normalize_path(path) {
            Ok(path) => Ok(GTPath(span, GTPathModuleId::Unresolved, path)),
            Err(_) => Err(GTParseError::Internal(span, GTNode::Path)),
        }
    }

    fn normalize_path(path: &str) -> Result<String, OsString> {
        let mut result = PathBuf::new();
        let mut components = Path::new(path).components().peekable();

        let mut leading = Vec::new();
        while let Some(&component) = components.peek() {
            match component {
                Component::CurDir | Component::ParentDir => {
                    leading.push(component.as_os_str().to_owned());
                    components.next();
                }
                _ => break,
            }
        }

        for component in components {
            match component {
                Component::CurDir => {
                    // Skip redundant .
                }

                Component::ParentDir => {
                    // Pop the last component if possible, else push ..
                    if !result.pop() {
                        result.push("..");
                    }
                }

                Component::Normal(c) => {
                    result.push(c);
                }

                _ => {}
            }
        }

        let mut normalized = PathBuf::new();
        for component in leading {
            normalized.push(component);
        }
        normalized.push(result);

        normalized.into_os_string().into_string().into()
    }
}

impl TryFrom<Pair<'_, Rule>> for GTPath {
    type Error = GTParseError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        GTPath::parse(pair.as_span().into(), pair.as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use insta::assert_ron_snapshot;
    use miette::NamedSource;
    use pest::Parser;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse() {
        let mut pairs = GenotypeParser::parse(Rule::path, "./hello/world/").unwrap();
        assert_eq!(
            GTPath::parse((0, 14).into(), "./hello/world/"),
            pairs.next().unwrap().try_into()
        );
    }

    #[test]
    fn test_split() {
        let mut pairs = GenotypeParser::parse(Rule::inline_import, "./hello/World").unwrap();
        let result = GTPath::split_parse(pairs.next().unwrap()).unwrap();
        assert_eq!(
            result,
            (
                GTPath::parse((0, 7).into(), "./hello").unwrap(),
                ((8, 13).into(), "World".into())
            )
        );
    }

    #[test]
    fn test_normalize_path() {
        assert_eq!(
            GTPath::parse((0, 0).into(), "./path/to/../module")
                .unwrap()
                .source_str(),
            "./path/module"
        );
        assert_eq!(
            GTPath::parse((0, 0).into(), "./path/./to/./module")
                .unwrap()
                .source_str(),
            "./path/to/module"
        );
        assert_eq!(
            GTPath::parse((0, 0).into(), "path/./to/./module/../module")
                .unwrap()
                .source_str(),
            "path/to/module"
        );
        assert_eq!(
            GTPath::parse((0, 0).into(), "./././path/./to/./module/../module")
                .unwrap()
                .source_str(),
            "./path/to/module"
        );
        assert_eq!(
            GTPath::parse((0, 0).into(), "../../../path/./to/./module/../module")
                .unwrap()
                .source_str(),
            "../../../path/to/module"
        );
    }

    #[test]
    fn test_normalize_source() {
        let source_code = NamedSource::new(
            "module.type",
            r#"use author/./*
            use ../user/../user/User
            use ./././misc/order/{Order, SomethingElse}

            Order: {
                book: book/Book
                user: ./misc/../misc/./user/User
            }"#
            .into(),
        );
        let parse = GTModule::parse("module".into(), source_code.clone()).unwrap();
        assert_ron_snapshot!(
            parse.module,
            @r#"
        GTModule(
          id: GTModuleId("module"),
          doc: None,
          imports: [
            GTImport(
              span: GTSpan(0, 14),
              path: GTPath(GTSpan(4, 12), Unresolved, "author"),
              reference: Glob(GTSpan(13, 14)),
            ),
            GTImport(
              span: GTSpan(27, 51),
              path: GTPath(GTSpan(31, 46), Unresolved, "../user"),
              reference: Name(GTSpan(47, 51), GTIdentifier(GTSpan(47, 51), "User")),
            ),
            GTImport(
              span: GTSpan(64, 107),
              path: GTPath(GTSpan(68, 84), Unresolved, "./misc/order"),
              reference: Names(GTSpan(85, 107), [
                Name(GTSpan(86, 91), GTIdentifier(GTSpan(86, 91), "Order")),
                Name(GTSpan(93, 106), GTIdentifier(GTSpan(93, 106), "SomethingElse")),
              ]),
            ),
          ],
          aliases: [
            GTAlias(
              id: GTDefinitionId(GTModuleId("module"), "Order"),
              span: GTSpan(121, 224),
              doc: None,
              attributes: [],
              name: GTIdentifier(GTSpan(121, 126), "Order"),
              descriptor: Object(GTObject(
                span: GTSpan(128, 224),
                name: Named(GTIdentifier(GTSpan(121, 126), "Order")),
                extensions: [],
                properties: [
                  GTProperty(
                    span: GTSpan(146, 161),
                    doc: None,
                    attributes: [],
                    name: GTKey(GTSpan(146, 150), "book"),
                    descriptor: InlineImport(GTInlineImport(
                      span: GTSpan(152, 161),
                      name: GTIdentifier(GTSpan(157, 161), "Book"),
                      path: GTPath(GTSpan(152, 156), Unresolved, "book"),
                    )),
                    required: true,
                  ),
                  GTProperty(
                    span: GTSpan(178, 210),
                    doc: None,
                    attributes: [],
                    name: GTKey(GTSpan(178, 182), "user"),
                    descriptor: InlineImport(GTInlineImport(
                      span: GTSpan(184, 210),
                      name: GTIdentifier(GTSpan(206, 210), "User"),
                      path: GTPath(GTSpan(184, 205), Unresolved, "./misc/user"),
                    )),
                    required: true,
                  ),
                ],
              )),
            ),
          ],
        )
        "#
        );
    }
}
