use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};
use genotype_lang_py_config::PYLangConfig;

use crate::PYRender;

use super::PYClass;

impl PYRender for PYClass {
    fn render(&self, indent: &GTIndent, config: &PYLangConfig) -> String {
        let prop_indent = indent.increment();

        let properties = self
            .properties
            .iter()
            .map(|property| property.render(&prop_indent, config))
            .collect::<Vec<String>>()
            .join("\n");

        let mut extensions = vec!["Model".to_string()];
        extensions.extend(
            self.extensions
                .iter()
                .map(|extension| extension.render(indent, config))
                .collect::<Vec<_>>(),
        );
        let extensions = extensions.join(", ");

        format!(
            "{}class {}{}:{}{}{}",
            indent.string,
            self.name.render(indent),
            if extensions.len() > 0 {
                format!("({})", extensions)
            } else {
                "".into()
            },
            self.doc
                .as_ref()
                .map_or(Default::default(), |doc| "\n".to_string()
                    + &doc.render(&indent.increment())
                    + if properties.len() > 0 { "\n\n" } else { "" }),
            if self.doc.is_none() && properties.len() > 0 {
                "\n"
            } else {
                ""
            },
            properties,
        )
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::*;

    #[test]
    fn test_render_empty() {
        assert_eq!(
            PYClass {
                doc: None,
                name: "Name".into(),
                extensions: vec![],
                properties: vec![]
            }
            .render(&py_indent(), &Default::default()),
            r#"class Name(Model):"#
        );
    }

    #[test]
    fn test_render_properties() {
        assert_eq!(
            PYClass {
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
                ]
            }
            .render(&py_indent(), &Default::default()),
            r#"class Name(Model):
    name: str
    age: Optional[int] = None"#
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            PYClass {
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
                ]
            }
            .render(&py_indent().increment(), &Default::default()),
            r#"    class Name(Model):
        name: str
        age: Optional[int] = None"#
        );
    }

    #[test]
    fn test_render_extensions() {
        assert_eq!(
            PYClass {
                doc: None,
                name: "Name".into(),
                extensions: vec![
                    PYReference::new("Hello".into(), false).into(),
                    PYReference::new("World".into(), false).into()
                ],
                properties: vec![PYProperty {
                    doc: None,
                    name: "name".into(),
                    descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                    required: true
                },]
            }
            .render(&py_indent(), &Default::default()),
            r#"class Name(Model, Hello, World):
    name: str"#
        );
    }

    #[test]
    fn test_render_doc_empty() {
        assert_eq!(
            PYClass {
                doc: Some(PYDoc("Hello, world!".into())),
                name: "Name".into(),
                extensions: vec![],
                properties: vec![]
            }
            .render(&py_indent(), &Default::default()),
            r#"class Name(Model):
    """Hello, world!""""#
        );
    }

    #[test]
    fn test_render_doc_properties() {
        assert_eq!(
            PYClass {
                doc: Some(PYDoc("Hello, world!".into())),
                name: "Name".into(),
                extensions: vec![],
                properties: vec![PYProperty {
                    doc: None,
                    name: "name".into(),
                    descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                    required: true
                },]
            }
            .render(&py_indent(), &Default::default()),
            r#"class Name(Model):
    """Hello, world!"""

    name: str"#
        );
    }
}
