use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSObject;

impl GTRender for TSObject {
    fn render(&self, indent: &GTIndent) -> String {
        let prop_indent = indent.increment();
        let properties = self
            .properties
            .iter()
            .map(|property| property.render(&prop_indent))
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
    use crate::{
        descriptor::TSDescriptor, indent::ts_indent, primitive::TSPrimitive, property::TSProperty,
    };

    use super::*;

    #[test]
    fn test_render_empty() {
        assert_eq!(TSObject { properties: vec![] }.render(&ts_indent()), "{\n}");
    }

    #[test]
    fn test_render_properties() {
        assert_eq!(
            TSObject {
                properties: vec![
                    TSProperty {
                        name: "name".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                        required: true
                    },
                    TSProperty {
                        name: "age".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::Number),
                        required: false
                    }
                ]
            }
            .render(&ts_indent()),
            "{\n  name: string,\n  age?: number\n}"
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            TSObject {
                properties: vec![
                    TSProperty {
                        name: "name".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                        required: true
                    },
                    TSProperty {
                        name: "age".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::Number),
                        required: false
                    }
                ]
            }
            .render(&ts_indent().increment()),
            "{\n    name: string,\n    age?: number\n  }"
        );
    }
}
