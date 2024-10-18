use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use crate::{PYOptions, PYRender};

use super::PYClass;

impl PYRender for PYClass {
    fn render(&self, indent: &GTIndent, options: &PYOptions) -> String {
        let prop_indent = indent.increment();

        let properties = self
            .properties
            .iter()
            .map(|property| property.render(&prop_indent, options))
            .collect::<Vec<String>>()
            .join("\n");

        let mut extensions = vec!["Model".to_string()];
        extensions.extend(
            self.extensions
                .iter()
                .map(|extension| extension.render(indent, options))
                .collect::<Vec<_>>(),
        );
        let extensions = extensions.join(", ");

        format!(
            "{}class {}{}:{}{}",
            indent.string,
            self.name.render(indent),
            if extensions.len() > 0 {
                format!("({})", extensions)
            } else {
                "".into()
            },
            if properties.len() > 0 { "\n" } else { "" },
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
                name: "Name".into(),
                extensions: vec![],
                properties: vec![]
            }
            .render(&py_indent(), &PYOptions::default()),
            r#"class Name(Model):"#
        );
    }

    #[test]
    fn test_render_properties() {
        assert_eq!(
            PYClass {
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
            r#"class Name(Model):
    name: str
    age: Optional[int] = None"#
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            PYClass {
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
            r#"    class Name(Model):
        name: str
        age: Optional[int] = None"#
        );
    }

    #[test]
    fn test_render_extensions() {
        assert_eq!(
            PYClass {
                name: "Name".into(),
                extensions: vec![
                    PYReference::new("Hello".into(), false).into(),
                    PYReference::new("World".into(), false).into()
                ],
                properties: vec![PYProperty {
                    name: "name".into(),
                    descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                    required: true
                },]
            }
            .render(&py_indent(), &PYOptions::default()),
            r#"class Name(Model, Hello, World):
    name: str"#
        );
    }
}
