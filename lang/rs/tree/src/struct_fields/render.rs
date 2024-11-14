use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;

use super::{RSRender, RSStructFields};

impl RSRender for RSStructFields {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> String {
        match self {
            RSStructFields::Resolved(fields) => {
                if fields.len() == 0 {
                    return ";".into();
                }

                let fields_indent = indent.increment();
                let fields = fields
                    .iter()
                    .map(|property| property.render(&fields_indent, config) + ",")
                    .collect::<Vec<String>>()
                    .join("\n");

                format!(" {{\n{fields}\n{indent}}}", indent = indent.string)
            }

            RSStructFields::Unresolved(_span, _, _) => {
                panic!("Attempted to render unresolved struct fields")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::*;

    #[test]
    fn test_render_fields() {
        assert_eq!(
            RSStructFields::Resolved(vec![
                RSProperty {
                    doc: None,
                    attributes: vec![],
                    name: "name".into(),
                    descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                },
                RSProperty {
                    doc: None,
                    attributes: vec![],
                    name: "age".into(),
                    descriptor: RSDescriptor::Primitive(RSPrimitive::Int),
                }
            ])
            .render(&rs_indent(), &Default::default()),
            r#" {
    name: String,
    age: isize,
}"#
        );
    }

    #[test]
    fn test_render_empty() {
        assert_eq!(
            RSStructFields::Resolved(vec![]).render(&rs_indent(), &Default::default()),
            ";"
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            RSStructFields::Resolved(vec![
                RSProperty {
                    doc: None,
                    attributes: vec![],
                    name: "name".into(),
                    descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                },
                RSProperty {
                    doc: None,
                    attributes: vec![],
                    name: "age".into(),
                    descriptor: RSDescriptor::Primitive(RSPrimitive::Int),
                }
            ])
            .render(&rs_indent().increment(), &Default::default()),
            r#" {
        name: String,
        age: isize,
    }"#
        );
    }
}
