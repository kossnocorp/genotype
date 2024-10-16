use genotype_lang_core_tree::indent::GTIndent;

use crate::{PYOptions, PYRender};

use super::PYObject;

impl PYRender for PYObject {
    fn render(&self, indent: &GTIndent, options: &PYOptions) -> String {
        let prop_indent = indent.increment();
        let properties = self
            .properties
            .iter()
            .map(|property| property.render(&prop_indent, options))
            .collect::<Vec<String>>()
            .join(",\n");
        format!(
            "{}\n{}{}{}",
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

    use crate::{
        descriptor::PYDescriptor, indent::py_indent, primitive::PYPrimitive, property::PYProperty,
    };

    use super::*;

    #[test]
    fn test_render_empty() {
        assert_eq!(
            PYObject { properties: vec![] }.render(&py_indent(), &PYOptions::default()),
            "{\n}"
        );
    }

    #[test]
    fn test_render_properties() {
        assert_eq!(
            PYObject {
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
            "{\n    name: str,\n    age?: int\n}"
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            PYObject {
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
            "{\n        name: str,\n        age?: int\n    }"
        );
    }
}
