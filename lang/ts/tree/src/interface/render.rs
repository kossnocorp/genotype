use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSInterface;

impl GTRender for TSInterface {
    fn render(&self, indent: &GTIndent) -> String {
        let prop_indent = indent.increment();
        let properties = self
            .properties
            .iter()
            .map(|property| property.render(&prop_indent))
            .collect::<Vec<String>>()
            .join(";\n");
        format!(
            "{}interface {} {}\n{}{}{}",
            indent.string,
            self.name.render(indent),
            "{",
            properties,
            if properties.len() > 0 { "\n" } else { "" },
            indent.format("}")
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{
        indent::ts_indent, name::TSName, primitive::TSPrimitive, property::TSProperty,
        type_descriptor::TSTypeDescriptor,
    };

    #[test]
    fn test_render_empty() {
        let indent = ts_indent();
        assert_eq!(
            TSInterface {
                name: TSName("Name".to_string()),
                properties: vec![]
            }
            .render(&indent),
            "interface Name {\n}"
        );
    }

    #[test]
    fn test_render_properties() {
        let indent = ts_indent();
        assert_eq!(
            TSInterface {
                name: TSName("Name".to_string()),
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
            "interface Name {\n  name: string;\n  age?: number\n}"
        );
    }

    #[test]
    fn test_render_indent() {
        let indent = ts_indent().increment();
        assert_eq!(
            TSInterface {
                name: TSName("Name".to_string()),
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
            "  interface Name {\n    name: string;\n    age?: number\n  }"
        );
    }
}
