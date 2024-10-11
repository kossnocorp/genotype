use pest::iterators::Pair;

use crate::parser::Rule;

use super::GTPath;

impl TryFrom<Pair<'_, Rule>> for GTPath {
    type Error = Box<dyn std::error::Error>;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        Ok(GTPath::new(pair.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::tree::*;

    #[test]
    fn test_parse_normalize() {
        let code = r#"use author/./*
        use ../user/../user/User
        use ./././misc/order/{Order, SomethingElse}
        
        Order = {
            book: book/Book
            user: ./misc/../misc/./user/User
        }"#;
        let parse = GTModule::parse(code.into()).unwrap();
        assert_eq!(
            parse.module,
            GTModule {
                doc: None,
                imports: vec![
                    GTImport {
                        path: "author".into(),
                        reference: GTImportReference::Glob,
                    },
                    GTImport {
                        path: "../user".into(),
                        reference: GTImportReference::Name("User".into()),
                    },
                    GTImport {
                        path: "./misc/order".into(),
                        reference: GTImportReference::Names(vec![
                            GTImportName::Name("Order".into(),),
                            GTImportName::Name("SomethingElse".into(),),
                        ],),
                    },
                ],
                aliases: vec![GTAlias {
                    doc: None,
                    name: "Order".into(),
                    descriptor: GTDescriptor::Object(GTObject {
                        extensions: vec![],
                        properties: vec![
                            GTProperty {
                                doc: None,
                                name: "book".into(),
                                descriptor: GTDescriptor::InlineImport(GTInlineImport {
                                    name: "Book".into(),
                                    path: "book".into(),
                                },),
                                required: true,
                            },
                            GTProperty {
                                doc: None,
                                name: "user".into(),
                                descriptor: GTDescriptor::InlineImport(GTInlineImport {
                                    name: "User".into(),
                                    path: "./misc/user".into(),
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
