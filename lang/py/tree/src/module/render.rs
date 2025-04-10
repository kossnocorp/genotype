use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for PYModule {
    type RenderContext = PYRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        let mut blocks = vec![];

        if let Some(doc) = &self.doc {
            blocks.push(doc.render(context)?);
        }

        let imports = Self::join_imports(
            &self
                .imports
                .iter()
                .map(|import| import.render(context))
                .collect::<Result<Vec<_>>>()?,
        );

        if !imports.is_empty() {
            blocks.push(imports);
        }

        let definitions = Self::join_definitions(
            &self
                .definitions
                .iter()
                .map(|definition| definition.render(context))
                .collect::<Result<Vec<_>>>()?,
        );

        if !definitions.is_empty() {
            blocks.push(definitions);
        }

        Ok(Self::join_blocks(&blocks))
    }
}

impl GtlRenderModule for PYModule {
    fn join_definitions(definitions: &Vec<String>) -> String {
        definitions.join("\n\n\n")
    }

    fn join_blocks(blocks: &Vec<String>) -> String {
        blocks.join("\n\n\n") + "\n"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render() {
        assert_eq!(
            PYModule {
                doc: None,
                imports: vec![
                    PYImport {
                        path: ".path.to.module".into(),
                        reference: PYImportReference::Default(Some("name".into())),
                        dependency: PYDependency::Local(".path.to.module".into())
                    },
                    PYImport {
                        path: ".path.to.module".into(),
                        reference: PYImportReference::Named(vec![
                            PYImportName::Name("Name".into()),
                            PYImportName::Alias("Name".into(), "Alias".into()),
                        ]),
                        dependency: PYDependency::Local(".path.to.module".into())
                    }
                ],
                definitions: vec![
                    PYDefinition::Alias(PYAlias {
                        doc: None,
                        name: "Name".into(),
                        descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                        references: vec![],
                    }),
                    PYDefinition::Class(PYClass {
                        doc: None,
                        name: "Name".into(),
                        extensions: vec![],
                        properties: vec![
                            PYProperty {
                                doc: None,
                                name: "name".into(),
                                descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                                required: true
                            },
                            PYProperty {
                                doc: None,
                                name: "age".into(),
                                descriptor: PYDescriptor::Primitive(PYPrimitive::Int),
                                required: false
                            }
                        ],
                        references: vec![],
                    }),
                ]
            }
            .render(&mut Default::default())
            .unwrap(),
            r#"import .path.to.module as name
from .path.to.module import Name, Name as Alias


type Name = str


class Name(Model):
    name: str
    age: Optional[int] = None
"#
        );
    }

    #[test]
    fn test_render_doc() {
        assert_eq!(
            PYModule {
                doc: Some(PYDoc("Hello, world!".into())),
                imports: vec![PYImport {
                    path: ".path.to.module".into(),
                    reference: PYImportReference::Default(Some("name".into())),
                    dependency: PYDependency::Local(".path.to.module".into())
                },],
                definitions: vec![PYDefinition::Alias(PYAlias {
                    doc: None,
                    name: "Name".into(),
                    descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                    references: vec![],
                })]
            }
            .render(&mut Default::default())
            .unwrap(),
            r#""""Hello, world!"""


import .path.to.module as name


type Name = str
"#
        );
    }
}
