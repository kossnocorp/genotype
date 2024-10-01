use std::collections::HashSet;

use genotype_project::{path::GTProjectPath, project::GTProject};

use crate::module::TSProjectModule;

#[derive(Debug, PartialEq, Clone)]
pub struct TSProject {
    path: GTProjectPath,
    pub modules: HashSet<TSProjectModule>,
}

impl From<GTProject> for TSProject {
    fn from(project: GTProject) -> Self {
        let modules = project
            .modules
            .into_iter()
            .map(|module| TSProjectModule::from(module))
            .collect();

        Self {
            path: project.path,
            modules,
        }
    }
}

#[cfg(test)]
mod tests {
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
                path: "./examples/basic".try_into().unwrap(),
                modules: vec![
                    TSProjectModule {
                        path: "./examples/basic/author.type".try_into().unwrap(),
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
                        path: "./examples/basic/book.type".try_into().unwrap(),
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
                path: "./examples/basic".try_into().unwrap(),
                modules: vec![
                    GTProjectModule {
                        path: "./examples/basic/author.type".try_into().unwrap(),
                        deps: vec![].into_iter().collect(),
                        exports: vec![].into_iter().collect(),
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
                        path: "./examples/basic/book.type".try_into().unwrap(),
                        deps: vec![].into_iter().collect(),
                        exports: vec![].into_iter().collect(),
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
