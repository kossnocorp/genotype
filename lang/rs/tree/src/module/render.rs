use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for RSModule {
    type RenderContext = RSRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        let mut blocks = vec![];

        if let Some(doc) = &self.doc {
            let doc = doc.render(context)?;
            if !doc.is_empty() {
                blocks.push(doc);
            }
        }

        let imports = Self::join_imports(
            &self
                .imports
                .iter()
                .map(|import| import.render(context))
                .collect::<Result<Vec<String>>>()?,
        );

        if !imports.is_empty() {
            blocks.push(imports);
        }

        let definitions = Self::join_definitions(
            &self
                .definitions
                .iter()
                .map(|definition| definition.render(context))
                .collect::<Result<Vec<String>>>()?,
        );

        if !definitions.is_empty() {
            blocks.push(definitions);
        }

        Ok(Self::join_blocks(&blocks))
    }
}

impl GtlRenderModule for RSModule {}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_parser::GTDefinitionId;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render() {
        assert_eq!(
            RSModule {
                id: "module".into(),
                doc: None,
                imports: vec![
                    RSUse {
                        reference: RSUseReference::Module,
                        dependency: RSDependency::Local(RSPath(
                            "path/to/module".into(),
                            "self::path::to::module".into()
                        ))
                    },
                    RSUse {
                        reference: RSUseReference::Named(vec![
                            RSUseName::Name("Name".into()),
                            RSUseName::Alias("Name".into(), "Alias".into()),
                        ]),
                        dependency: RSDependency::Local(RSPath(
                            "path/to/module".into(),
                            "self::path::to::module".into()
                        ))
                    }
                ],
                definitions: vec![
                    RSDefinition::Alias(RSAlias {
                        id: GTDefinitionId("module".into(), "Name".into()),
                        doc: None,
                        name: "Name".into(),
                        descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                    }),
                    RSDefinition::Struct(RSStruct {
                        id: GTDefinitionId("module".into(), "Name".into()),
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
                                descriptor: RSDescriptor::Primitive(RSPrimitive::IntSize),
                            }
                        ]
                        .into(),
                    }),
                ]
            }
            .render(&mut Default::default())
            .unwrap(),
            r#"use self::path::to::module;
use self::path::to::module::{Name, Name as Alias};

pub type Name = String;

pub struct Name {
    pub name: String,
    pub age: isize,
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
                    reference: RSUseReference::Module,
                    dependency: RSDependency::Local(RSPath(
                        "path/to/module".into(),
                        "self::path::to::module".into()
                    ))
                },],
                definitions: vec![RSDefinition::Alias(RSAlias {
                    id: GTDefinitionId("module".into(), "Name".into()),
                    doc: None,
                    name: "Name".into(),
                    descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                })]
            }
            .render(&mut Default::default())
            .unwrap(),
            r#"//! Hello, world!

use self::path::to::module;

pub type Name = String;
"#
        );
    }
}
