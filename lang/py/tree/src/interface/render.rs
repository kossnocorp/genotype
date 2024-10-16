use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use crate::{PYOptions, PYRender};

use super::PYInterface;

impl PYRender for PYInterface {
    fn render(&self, indent: &GTIndent, options: &PYOptions) -> String {
        let prop_indent = indent.increment();

        let properties = self
            .properties
            .iter()
            .map(|property| property.render(&prop_indent, options) + ";")
            .collect::<Vec<String>>()
            .join("\n");

        let extensions = self
            .extensions
            .iter()
            .map(|extension| extension.render(indent))
            .collect::<Vec<String>>()
            .join(", ");

        format!(
            "{}interface {}{} {}\n{}{}{}",
            indent.string,
            self.name.render(indent),
            if extensions.len() > 0 {
                format!(" extends {}", extensions)
            } else {
                "".into()
            },
            "{",
            properties,
            if properties.len() > 0 { "\n" } else { "" },
            indent.format("}")
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
            PYInterface {
                name: "Name".into(),
                extensions: vec![],
                properties: vec![]
            }
            .render(&py_indent(), &PYOptions::default()),
            "interface Name {\n}"
        );
    }

    #[test]
    fn test_render_properties() {
        assert_eq!(
            PYInterface {
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
            }
            .render(&py_indent(), &PYOptions::default()),
            r#"interface Name {
    name: str;
    age?: int;
}"#
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            PYInterface {
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
            }
            .render(&py_indent().increment(), &PYOptions::default()),
            r#"    interface Name {
        name: str;
        age?: int;
    }"#
        );
    }

    #[test]
    fn test_render_extensions() {
        assert_eq!(
            PYInterface {
                name: "Name".into(),
                extensions: vec!["Hello".into(), "World".into()],
                properties: vec![PYProperty {
                    name: "name".into(),
                    descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                    required: true
                },]
            }
            .render(&py_indent().increment(), &PYOptions::default()),
            r#"    interface Name extends Hello, World {
        name: str;
    }"#
        );
    }
}
