use pest::iterators::Pair;
use std::{
    ffi::OsString,
    path::{Component, Path, PathBuf},
};

use crate::{parser::Rule, GTNode, GTNodeParseResult, GTParseError, GTPathModuleId, GTSpan};

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
                .as_str(),
            "./path/module"
        );
        assert_eq!(
            GTPath::parse((0, 0).into(), "./path/./to/./module")
                .unwrap()
                .as_str(),
            "./path/to/module"
        );
        assert_eq!(
            GTPath::parse((0, 0).into(), "path/./to/./module/../module")
                .unwrap()
                .as_str(),
            "path/to/module"
        );
        assert_eq!(
            GTPath::parse((0, 0).into(), "./././path/./to/./module/../module")
                .unwrap()
                .as_str(),
            "./path/to/module"
        );
        assert_eq!(
            GTPath::parse((0, 0).into(), "../../../path/./to/./module/../module")
                .unwrap()
                .as_str(),
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
            
            Order = {
                book: book/Book
                user: ./misc/../misc/./user/User
            }"#
            .into(),
        );
        let parse = GTModule::parse("module".into(), source_code.clone()).unwrap();
        assert_eq!(
            parse.module,
            GTModule {
                id: "module".into(),
                source_code,
                doc: None,
                imports: vec![
                    GTImport {
                        span: (0, 14).into(),
                        path: GTPath::parse((4, 12).into(), "author").unwrap(),
                        reference: GTImportReference::Glob((13, 14).into()),
                    },
                    GTImport {
                        span: (27, 51).into(),
                        path: GTPath::parse((31, 46).into(), "../user").unwrap(),
                        reference: GTImportReference::Name(
                            (47, 51).into(),
                            GTIdentifier::new((47, 51).into(), "User".into())
                        ),
                    },
                    GTImport {
                        span: (64, 107).into(),
                        path: GTPath::parse((68, 84).into(), "./misc/order").unwrap(),
                        reference: GTImportReference::Names(
                            (85, 107).into(),
                            vec![
                                GTImportName::Name(
                                    (86, 91).into(),
                                    GTIdentifier::new((86, 91).into(), "Order".into())
                                ),
                                GTImportName::Name(
                                    (93, 106).into(),
                                    GTIdentifier::new((93, 106).into(), "SomethingElse".into()),
                                ),
                            ],
                        ),
                    },
                ],
                aliases: vec![GTAlias {
                    id: GTAliasId("module".into(), "Order".into()),
                    span: (133, 237).into(),
                    doc: None,
                    attributes: vec![],
                    name: GTIdentifier::new((133, 138).into(), "Order".into()),
                    descriptor: GTDescriptor::Object(GTObject {
                        span: (141, 237).into(),
                        name: GTIdentifier::new((133, 138).into(), "Order".into()).into(),
                        extensions: vec![],
                        properties: vec![
                            GTProperty {
                                span: (159, 174).into(),
                                doc: None,
                                attributes: vec![],
                                name: GTKey::new((159, 163).into(), "book".into()),
                                descriptor: GTDescriptor::InlineImport(GTInlineImport {
                                    span: (165, 174).into(),
                                    name: GTIdentifier::new((170, 174).into(), "Book".into()),
                                    path: GTPath::parse((165, 169).into(), "book").unwrap(),
                                },),
                                required: true,
                            },
                            GTProperty {
                                span: (191, 223).into(),
                                doc: None,
                                attributes: vec![],
                                name: GTKey::new((191, 195).into(), "user".into()),
                                descriptor: GTDescriptor::InlineImport(GTInlineImport {
                                    span: (197, 223).into(),
                                    name: GTIdentifier::new((219, 223).into(), "User".into()),
                                    path: GTPath::parse((197, 218).into(), "./misc/user").unwrap(),
                                },),
                                required: true,
                            },
                        ],
                    },),
                },],
            }
        );
    }
}
