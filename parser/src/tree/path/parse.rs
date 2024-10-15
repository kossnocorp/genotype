use pest::iterators::Pair;
use std::{
    ffi::OsString,
    path::{Component, Path, PathBuf},
};

use crate::{parser::Rule, GTNode, GTNodeParseError, GTNodeParseResult, GTSpan};

use super::GTPath;

impl GTPath {
    pub fn split_parse(pair: Pair<'_, Rule>) -> GTNodeParseResult<(GTPath, (GTSpan, String))> {
        let span = pair.as_span();
        let span_start = span.start();
        let str = pair.as_str().to_string();
        let else_err = || GTNodeParseError::Internal(span.into(), GTNode::Path);

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
            Ok(path) => Ok(GTPath(span, path)),
            Err(_) => Err(GTNodeParseError::Internal(span, GTNode::Path)),
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
    type Error = GTNodeParseError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        GTPath::parse(pair.as_span().into(), pair.as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
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
        let source_code = GTSourceCode::new(
            "module.type".into(),
            r#"use author/./*
        use ../user/../user/User
        use ./././misc/order/{Order, SomethingElse}
        
        Order = {
            book: book/Book
            user: ./misc/../misc/./user/User
        }"#
            .into(),
        );
        let parse = GTModule::parse(source_code.clone()).unwrap();
        assert_eq!(
            parse.module,
            GTModule {
                source_code,
                doc: None,
                imports: vec![
                    GTImport {
                        path: GTPath::parse((4, 12).into(), "author").unwrap(),
                        reference: GTImportReference::Glob,
                    },
                    GTImport {
                        path: GTPath::parse((27, 42).into(), "../user").unwrap(),
                        reference: GTImportReference::Name(GTIdentifier::new(
                            (43, 47).into(),
                            "User".into()
                        )),
                    },
                    GTImport {
                        path: GTPath::parse((60, 76).into(), "./misc/order").unwrap(),
                        reference: GTImportReference::Names(vec![
                            GTImportName::Name(GTIdentifier::new((78, 83).into(), "Order".into()),),
                            GTImportName::Name(GTIdentifier::new(
                                (85, 98).into(),
                                "SomethingElse".into()
                            ),),
                        ],),
                    },
                ],
                aliases: vec![GTAlias {
                    doc: None,
                    name: GTIdentifier::new((117, 122).into(), "Order".into()),
                    descriptor: GTDescriptor::Object(GTObject {
                        extensions: vec![],
                        properties: vec![
                            GTProperty {
                                doc: None,
                                name: GTKey::new((139, 143).into(), "book".into()),
                                descriptor: GTDescriptor::InlineImport(GTInlineImport {
                                    name: GTIdentifier::new((150, 154).into(), "Book".into()),
                                    path: GTPath::parse((145, 149).into(), "book").unwrap(),
                                },),
                                required: true,
                            },
                            GTProperty {
                                doc: None,
                                name: GTKey::new((167, 171).into(), "user".into()),
                                descriptor: GTDescriptor::InlineImport(GTInlineImport {
                                    name: GTIdentifier::new((195, 199).into(), "User".into()),
                                    path: GTPath::parse((173, 194).into(), "./misc/user").unwrap(),
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
