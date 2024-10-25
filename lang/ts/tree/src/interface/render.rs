use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use crate::TSDoc;

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

        let extensions = self
            .extensions
            .iter()
            .map(|extension| extension.render(indent))
            .collect::<Vec<String>>()
            .join(", ");

        TSDoc::with_doc(
            &self.doc,
            indent,
            format!(
                "{}export interface {}{} {}\n{}{}{}",
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
            ),
            false,
        )
    }
}

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::*;

    #[test]
    fn test_render_empty() {
        assert_eq!(
            TSInterface {
                doc: None,
                name: "Name".into(),
                extensions: vec![],
                properties: vec![]
            }
            .render(&ts_indent()),
            "export interface Name {\n}"
        );
    }

    #[test]
    fn test_render_properties() {
        assert_eq!(
            TSInterface {
                doc: None,
                name: "Name".into(),
                extensions: vec![],
                properties: vec![
                    TSProperty {
                        doc: None,
                        name: "name".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                        required: true
                    },
                    TSProperty {
                        doc: None,
                        name: "age".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::Number),
                        required: false
                    }
                ]
            }
            .render(&ts_indent()),
            r#"export interface Name {
  name: string;
  age?: number;
}"#
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            TSInterface {
                doc: None,
                name: "Name".into(),
                extensions: vec![],
                properties: vec![
                    TSProperty {
                        doc: None,
                        name: "name".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                        required: true
                    },
                    TSProperty {
                        doc: None,
                        name: "age".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::Number),
                        required: false
                    }
                ]
            }
            .render(&ts_indent().increment()),
            r#"  export interface Name {
    name: string;
    age?: number;
  }"#
        );
    }

    #[test]
    fn test_render_extensions() {
        assert_eq!(
            TSInterface {
                doc: None,
                name: "Name".into(),
                extensions: vec!["Hello".into(), "World".into()],
                properties: vec![TSProperty {
                    doc: None,
                    name: "name".into(),
                    descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                    required: true
                },]
            }
            .render(&ts_indent()),
            r#"export interface Name extends Hello, World {
  name: string;
}"#
        );
    }

    #[test]
    fn test_render_doc() {
        assert_eq!(
            TSInterface {
                doc: Some(TSDoc("Hello, world!".into())),
                name: "Name".into(),
                extensions: vec![],
                properties: vec![]
            }
            .render(&ts_indent()),
            r#"/** Hello, world! */
export interface Name {
}"#
        );
    }
}
