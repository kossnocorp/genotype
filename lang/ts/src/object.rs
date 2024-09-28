use genotype_lang_core::{indent::Indent, node::Node};

use crate::property::TSProperty;

pub struct TSObject {
    pub properties: Vec<TSProperty>,
}

impl Node for TSObject {
    fn render(&self, indent: &Indent) -> String {
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
        indent::ts_indent, name::TSName, primitive::TSPrimitive, type_descriptor::TSTypeDescriptor,
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
                        optional: false
                    },
                    TSProperty {
                        name: TSName("age".to_string()),
                        descriptor: TSTypeDescriptor::Primitive(TSPrimitive::Number),
                        optional: true
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
                        optional: false
                    },
                    TSProperty {
                        name: TSName("age".to_string()),
                        descriptor: TSTypeDescriptor::Primitive(TSPrimitive::Number),
                        optional: true
                    }
                ]
            }
            .render(&indent),
            "{\n    name: string,\n    age?: number\n  }"
        );
    }
}
