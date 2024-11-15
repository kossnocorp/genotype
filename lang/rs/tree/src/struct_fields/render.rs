use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;
use miette::Result;

use crate::error::RSError;

use super::{RSRender, RSStructFields};

impl RSRender for RSStructFields {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> Result<String> {
        match self {
            RSStructFields::Resolved(fields) => {
                if fields.len() == 0 {
                    return Ok(";".into());
                }

                let fields_indent = indent.increment();
                let fields = fields
                    .iter()
                    .map(|property| {
                        property
                            .render(&fields_indent, config)
                            .map(|result| result + ",")
                    })
                    .collect::<Result<Vec<String>>>()?
                    .join("\n");

                Ok(format!(" {{\n{fields}\n{indent}}}", indent = indent.string))
            }

            RSStructFields::Unresolved(span, _, _) => {
                Err(RSError::UnresolvedStructFields(span.clone()).into())
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
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            r#" {
    name: String,
    age: isize,
}"#
        );
    }

    #[test]
    fn test_render_empty() {
        assert_eq!(
            RSStructFields::Resolved(vec![])
                .render(&rs_indent(), &Default::default())
                .unwrap(),
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
            .render(&rs_indent().increment(), &Default::default())
            .unwrap(),
            r#" {
        name: String,
        age: isize,
    }"#
        );
    }
}
