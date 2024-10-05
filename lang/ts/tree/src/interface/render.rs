use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSInterface;

impl GTRender for TSInterface {
    fn render(&self, indent: &GTIndent) -> String {
        let prop_indent = indent.increment();
        let properties = self
            .properties
            .iter()
            .map(|property| property.render(&prop_indent) + ";")
            .collect::<Vec<String>>()
            .join("\n");
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

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::{
        descriptor::TSDescriptor, indent::ts_indent, primitive::TSPrimitive, property::TSProperty,
    };

    #[test]
    fn test_render_empty() {
        assert_eq!(
            TSInterface {
                name: "Name".into(),
                properties: vec![]
            }
            .render(&ts_indent()),
            "interface Name {\n}"
        );
    }

    #[test]
    fn test_render_properties() {
        assert_eq!(
            TSInterface {
                name: "Name".into(),
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
            r#"interface Name {
  name: string;
  age?: number;
}"#
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            TSInterface {
                name: "Name".into(),
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
            r#"  interface Name {
    name: string;
    age?: number;
  }"#
        );
    }
}
