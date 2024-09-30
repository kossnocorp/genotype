use std::collections::HashSet;

use genotype_project::project::GTProject;

use crate::module::TSProjectModule;

#[derive(Debug, PartialEq, Clone)]
pub struct TSProject {
    pub modules: HashSet<TSProjectModule>,
}

impl From<GTProject> for TSProject {
    fn from(project: GTProject) -> Self {
        let modules = project
            .modules
            .into_iter()
            .map(|module| TSProjectModule::from(module))
            .collect();

        Self { modules }
    }
}

#[cfg(test)]
mod tests {

    use std::path::PathBuf;

    use genotype_lang_ts_tree::definition::TSDefinition;
    use genotype_lang_ts_tree::import::TSImport;
    use genotype_lang_ts_tree::import_name::TSImportName;
    use genotype_lang_ts_tree::import_reference::TSImportReference;
    use genotype_lang_ts_tree::interface::TSInterface;
    use genotype_lang_ts_tree::module::TSModule;
    use genotype_lang_ts_tree::name::TSName;
    use genotype_lang_ts_tree::primitive::TSPrimitive;
    use genotype_lang_ts_tree::property::TSProperty;
    use genotype_lang_ts_tree::type_descriptor::TSTypeDescriptor;
    use genotype_parser::tree::alias::GTAlias;
    use genotype_parser::tree::descriptor::GTDescriptor;
    use genotype_parser::tree::import::GTImport;
    use genotype_parser::tree::import_reference::GTImportReference;
    use genotype_parser::tree::module::GTModule;
    use genotype_parser::tree::name::GTName;
    use genotype_parser::tree::object::GTObject;
    use genotype_parser::tree::primitive::GTPrimitive;
    use genotype_parser::tree::property::GTProperty;
    use pretty_assertions::assert_eq;

    use super::*;
    use genotype_project::module::GTProjectModule;
    use genotype_project::project::GTProject;

    #[test]
    fn test_convert_base() {
        assert_eq!(
            TSProject {
                modules: vec![
                    TSProjectModule {
                        path: PathBuf::from("./examples/basic/author.type"),
                        module: TSModule {
                            doc: None,
                            imports: vec![],
                            definitions: vec![TSDefinition::Interface(TSInterface {
                                name: TSName("Author".to_string()),
                                properties: vec![TSProperty {
                                    name: TSName("name".to_string()),
                                    descriptor: TSTypeDescriptor::Primitive(TSPrimitive::String),
                                    required: true,
                                }],
                            })]
                        },
                    },
                    TSProjectModule {
                        path: PathBuf::from("./examples/basic/book.type"),
                        module: TSModule {
                            doc: None,
                            imports: vec![TSImport {
                                path: "./author".to_string(),
                                reference: TSImportReference::Named(vec![TSImportName::Name(
                                    TSName("Author".into())
                                )]),
                            }],
                            definitions: vec![TSDefinition::Interface(TSInterface {
                                name: TSName("Book".to_string()),
                                properties: vec![
                                    TSProperty {
                                        name: TSName("title".to_string()),
                                        descriptor: TSTypeDescriptor::Primitive(
                                            TSPrimitive::String
                                        ),
                                        required: true,
                                    },
                                    TSProperty {
                                        name: TSName("author".to_string()),
                                        descriptor: TSTypeDescriptor::Name(TSName(
                                            "Author".to_string(),
                                        )),
                                        required: true,
                                    },
                                ],
                            })],
                        },
                    },
                ]
                .into_iter()
                .collect()
            },
            GTProject {
                modules: vec![
                    GTProjectModule {
                        path: PathBuf::from("./examples/basic/author.type"),
                        module: GTModule {
                            doc: None,
                            imports: vec![],
                            aliases: vec![GTAlias {
                                doc: None,
                                name: GTName("Author".to_string()),
                                descriptor: GTDescriptor::Object(GTObject {
                                    properties: vec![GTProperty {
                                        doc: None,
                                        name: GTName("name".to_string()),
                                        descriptor: GTDescriptor::Primitive(GTPrimitive::String),
                                        required: true,
                                    }],
                                }),
                            }],
                        },
                    },
                    GTProjectModule {
                        path: PathBuf::from("./examples/basic/book.type"),
                        module: GTModule {
                            doc: None,
                            imports: vec![GTImport {
                                path: "./author".to_string(),
                                reference: GTImportReference::Name(GTName("Author".into())),
                            }],
                            aliases: vec![GTAlias {
                                doc: None,
                                name: GTName("Book".to_string()),
                                descriptor: GTDescriptor::Object(GTObject {
                                    properties: vec![
                                        GTProperty {
                                            doc: None,
                                            name: GTName("title".to_string()),
                                            descriptor: GTDescriptor::Primitive(
                                                GTPrimitive::String
                                            ),
                                            required: true,
                                        },
                                        GTProperty {
                                            doc: None,
                                            name: GTName("author".to_string()),
                                            descriptor: GTDescriptor::Name(GTName(
                                                "Author".to_string(),
                                            )),
                                            required: true,
                                        },
                                    ],
                                }),
                            }],
                        },
                    },
                ]
                .into_iter()
                .collect()
            }
            .into(),
        )
    }
}
