use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};
use genotype_lang_rs_config::RSLangConfig;

use crate::RSRender;

use super::RSModule;

impl RSRender for RSModule {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> String {
        let mut blocks = vec![];

        let doc = self
            .doc
            .as_ref()
            .map(|doc| doc.render(indent))
            .unwrap_or_default();

        if !doc.is_empty() {
            blocks.push(doc);
        }

        let imports = self
            .imports
            .iter()
            .map(|import| import.render(indent))
            .collect::<Vec<String>>()
            .join("\n");

        if !imports.is_empty() {
            blocks.push(imports);
        }

        let definitions = self
            .definitions
            .iter()
            .map(|definition| definition.render(indent, config))
            .collect::<Vec<String>>()
            .join("\n\n\n");

        if !definitions.is_empty() {
            blocks.push(definitions);
        }

        blocks.join("\n\n\n") + "\n"
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::*;

    #[test]
    fn test_render() {
        assert_eq!(
            RSModule {
                doc: None,
                imports: vec![
                    RSImport {
                        path: "self::path::to::module".into(),
                        reference: RSImportReference::Module,
                        dependency: RSDependency::Local("self::path::to::module".into())
                    },
                    RSImport {
                        path: "self::path::to::module".into(),
                        reference: RSImportReference::Named(vec![
                            RSImportName::Name("Name".into()),
                            RSImportName::Alias("Name".into(), "Alias".into()),
                        ]),
                        dependency: RSDependency::Local("self::path::to::module".into())
                    }
                ],
                definitions: vec![
                    RSDefinition::Alias(RSAlias {
                        doc: None,
                        name: "Name".into(),
                        descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                    }),
                    RSDefinition::Struct(RSStruct {
                        doc: None,
                        attributes: vec![],
                        name: "Name".into(),
                        fields: vec![
                            RSProperty {
                                doc: None,
                                attributes: vec![],
                                name: "name".into(),
                                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                            },
                            RSProperty {
                                doc: None,
                                attributes: vec![],
                                name: "age".into(),
                                descriptor: RSDescriptor::Primitive(RSPrimitive::Int),
                            }
                        ]
                        .into(),
                    }),
                ]
            }
            .render(&rs_indent(), &Default::default()),
            r#"use self::path::to::module;
use self::path::to::module::{Name, Name as Alias};


type Name = String;


struct Name {
    name: String,
    age: isize,
}
"#
        );
    }

    #[test]
    fn test_render_doc() {
        assert_eq!(
            RSModule {
                doc: Some(RSDoc::new("Hello, world!", true)),
                imports: vec![RSImport {
                    path: "self::path::to::module".into(),
                    reference: RSImportReference::Module,
                    dependency: RSDependency::Local("self::path::to::module".into())
                },],
                definitions: vec![RSDefinition::Alias(RSAlias {
                    doc: None,
                    name: "Name".into(),
                    descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                })]
            }
            .render(&rs_indent(), &Default::default()),
            r#"//! Hello, world!


use self::path::to::module;


type Name = String;
"#
        );
    }
}
