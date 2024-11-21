use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;
use miette::Result;

use crate::error::RSError;

use super::{RSRender, RSStructFields};

impl RSRender for RSStructFields {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> Result<String> {
        match self {
            RSStructFields::Tuple(descriptors) => {
                if descriptors.len() == 0 {
                    return Ok(";".into());
                }

                let descriptors = descriptors
                    .iter()
                    .map(|descriptor| {
                        descriptor
                            .render(indent, config)
                            .map(|result| format!("pub {result}"))
                    })
                    .collect::<Result<Vec<String>>>()?
                    .join(", ");

                Ok(format!("({descriptors});"))
            }

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
                RSField {
                    doc: None,
                    attributes: vec![],
                    name: "name".into(),
                    descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                },
                RSField {
                    doc: None,
                    attributes: vec![],
                    name: "age".into(),
                    descriptor: RSDescriptor::Primitive(RSPrimitive::Int),
                }
            ])
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            r#" {
    pub name: String,
    pub age: isize,
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
                RSField {
                    doc: None,
                    attributes: vec![],
                    name: "name".into(),
                    descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                },
                RSField {
                    doc: None,
                    attributes: vec![],
                    name: "age".into(),
                    descriptor: RSDescriptor::Primitive(RSPrimitive::Int),
                }
            ])
            .render(&rs_indent().increment(), &Default::default())
            .unwrap(),
            r#" {
        pub name: String,
        pub age: isize,
    }"#
        );
    }

    #[test]
    fn test_render_tuple() {
        assert_eq!(
            RSStructFields::Tuple(vec![
                RSDescriptor::Primitive(RSPrimitive::String),
                RSDescriptor::Primitive(RSPrimitive::Int),
            ])
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            "(pub String, pub isize);"
        );
    }

    #[test]
    fn test_render_empty_tuple() {
        assert_eq!(
            RSStructFields::Tuple(vec![])
                .render(&rs_indent(), &Default::default())
                .unwrap(),
            ";"
        );
    }
}
