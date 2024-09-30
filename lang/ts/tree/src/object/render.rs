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
        indent::ts_indent, name::TSName, primitive::TSPrimitive, property::TSProperty,
        type_descriptor::TSTypeDescriptor,
    };

    use super::*;

    #[test]
    fn test_render_empty() {
        let indent = ts_indent();
        assert_eq!(TSObject { properties: vec![] }.render(&indent), "{\n}");
    }

    #[test]
    fn test_render_properties() {
        let indent = ts_indent();
        assert_eq!(
            TSObject {
                properties: vec![
                    TSProperty {
                        name: TSName("name".to_string()),
                        descriptor: TSTypeDescriptor::Primitive(TSPrimitive::String),
                        required: true
                    },
                    TSProperty {
                        name: TSName("age".to_string()),
                        descriptor: TSTypeDescriptor::Primitive(TSPrimitive::Number),
                        required: false
                    }
                ]
            }
            .render(&indent),
            "{\n  name: string,\n  age?: number\n}"
        );
    }

    #[test]
    fn test_render_indent() {
        let indent = ts_indent().increment();
        assert_eq!(
            TSObject {
                properties: vec![
                    TSProperty {
                        name: TSName("name".to_string()),
                        descriptor: TSTypeDescriptor::Primitive(TSPrimitive::String),
                        required: true
                    },
                    TSProperty {
                        name: TSName("age".to_string()),
                        descriptor: TSTypeDescriptor::Primitive(TSPrimitive::Number),
                        required: false
                    }
                ]
            }
            .render(&indent),
            "{\n    name: string,\n    age?: number\n  }"
        );
    }
}
