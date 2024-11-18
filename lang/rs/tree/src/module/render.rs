use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;
use miette::Result;

use crate::RSRender;

use super::RSModule;

impl RSRender for RSModule {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> Result<String> {
        let mut blocks = vec![];

        if let Some(doc) = &self.doc {
            let doc = doc.render(indent, config)?;
            if !doc.is_empty() {
                blocks.push(doc);
            }
        }

        let imports = self
            .imports
            .iter()
            .map(|import| import.render(indent, config))
            .collect::<Result<Vec<String>>>()?
            .join("\n");

        if !imports.is_empty() {
            blocks.push(imports);
        }

        let definitions = self
            .definitions
            .iter()
            .map(|definition| definition.render(indent, config))
            .collect::<Result<Vec<String>>>()?
            .join("\n\n");

        if !definitions.is_empty() {
            blocks.push(definitions);
        }

        Ok(blocks.join("\n\n") + "\n")
    }
}

#[cfg(test)]
mod tests {
    use genotype_parser::GTAliasId;
    use pretty_assertions::assert_eq;

    use crate::*;

    #[test]
    fn test_render() {
        assert_eq!(
            RSModule {
                id: "module".into(),
                doc: None,
                imports: vec![
                    RSUse {
                        path: "self::path::to::module".into(),
                        reference: RSUseReference::Module,
                        dependency: RSDependency::Local("self::path::to::module".into())
                    },
                    RSUse {
                        path: "self::path::to::module".into(),
                        reference: RSUseReference::Named(vec![
                            RSUseName::Name("Name".into()),
                            RSUseName::Alias("Name".into(), "Alias".into()),
                        ]),
                        dependency: RSDependency::Local("self::path::to::module".into())
                    }
                ],
                definitions: vec![
                    RSDefinition::Alias(RSAlias {
                        id: GTAliasId("module".into(), "Name".into()),
                        doc: None,
                        name: "Name".into(),
                        descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                    }),
                    RSDefinition::Struct(RSStruct {
                        id: GTAliasId("module".into(), "Name".into()),
                        doc: None,
                        attributes: vec![],
                        name: "Name".into(),
                        fields: vec![
                            RSField {
                                doc: None,
                                attributes: vec![],
                                name: "name".into(),
                                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                            },
                            RSField {
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
            .render(&rs_indent(), &Default::default())
            .unwrap(),
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
                id: "module".into(),
                doc: Some(RSDoc::new("Hello, world!", true)),
                imports: vec![RSUse {
                    path: "self::path::to::module".into(),
                    reference: RSUseReference::Module,
                    dependency: RSDependency::Local("self::path::to::module".into())
                },],
                definitions: vec![RSDefinition::Alias(RSAlias {
                    id: GTAliasId("module".into(), "Name".into()),
                    doc: None,
                    name: "Name".into(),
                    descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                })]
            }
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            r#"//! Hello, world!

use self::path::to::module;

type Name = String;
"#
        );
    }
}
