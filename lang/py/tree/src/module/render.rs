use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};
use genotype_lang_py_config::PYLangConfig;

use crate::PYRender;

use super::PYModule;

impl PYRender for PYModule {
    fn render(&self, indent: &GTIndent, config: &PYLangConfig) -> String {
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
                    }),
                    PYDefinition::Class(PYClass {
                        doc: None,
                        name: "Name".into(),
                        extensions: vec![],
                        properties: vec![
                            PYProperty {
                                name: "name".into(),
                                descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                                required: true
                            },
                            PYProperty {
                                name: "age".into(),
                                descriptor: PYDescriptor::Primitive(PYPrimitive::Int),
                                required: false
                            }
                        ]
                    }),
                ]
            }
            .render(&py_indent(), &Default::default()),
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
                }),]
            }
            .render(&py_indent(), &Default::default()),
            r#""""Hello, world!"""


import .path.to.module as name


type Name = str
"#
        );
    }
}
