use pest::iterators::Pair;

use crate::{parser::Rule, GTNode, GTNodeParseError, GTNodeParseResult, GTSpan};

use super::GTPath;

impl GTPath {
    pub fn parse(pair: Pair<'_, Rule>) -> GTNodeParseResult<(GTPath, (GTSpan, String))> {
        let span = pair.as_span();
        let span_start = span.start();
        let str = pair.as_str().to_string();

        let name_index = str
            .rfind("/")
            .ok_or_else(|| GTNodeParseError::Internal(pair.as_span().into(), GTNode::Path))?;

        let path = &str[..name_index];
        let path_span = (span_start, span_start + name_index).into();

        let name = &str[name_index + 1..];
        let name_span = (span_start + name_index + 1, span.end()).into();

        Ok((GTPath::new(path_span, path), (name_span, name.into())))
    }
}

impl From<Pair<'_, Rule>> for GTPath {
    fn from(pair: Pair<'_, Rule>) -> Self {
        GTPath::new(pair.as_span().into(), pair.as_str())
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
            GTPath::new((0, 14).into(), "./hello/world/"),
            pairs.next().unwrap().into()
        );
    }

    #[test]
    fn test_split() {
        let mut pairs = GenotypeParser::parse(Rule::inline_import, "./hello/World").unwrap();
        let result = GTPath::parse(pairs.next().unwrap()).unwrap();
        assert_eq!(
            result,
            (
                GTPath::new((0, 7).into(), "./hello"),
                ((8, 13).into(), "World".into())
            )
        );
    }

    #[test]
    fn test_normalize() {
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
                        path: GTPath::new((4, 12).into(), "author"),
                        reference: GTImportReference::Glob,
                    },
                    GTImport {
                        path: GTPath::new((27, 42).into(), "../user"),
                        reference: GTImportReference::Name(GTIdentifier::new(
                            (43, 47).into(),
                            "User".into()
                        )),
                    },
                    GTImport {
                        path: GTPath::new((60, 76).into(), "./misc/order"),
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
                                name: "book".into(),
                                descriptor: GTDescriptor::InlineImport(GTInlineImport {
                                    name: GTIdentifier::new((150, 154).into(), "Book".into()),
                                    path: GTPath::new((145, 149).into(), "book"),
                                },),
                                required: true,
                            },
                            GTProperty {
                                doc: None,
                                name: "user".into(),
                                descriptor: GTDescriptor::InlineImport(GTInlineImport {
                                    name: GTIdentifier::new((195, 199).into(), "User".into()),
                                    path: GTPath::new((173, 194).into(), "./misc/user"),
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
