use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for RSStructFields {
    type RenderContext = RSRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        match self {
            RSStructFields::Tuple(descriptors) => {
                if descriptors.len() == 0 {
                    return Ok(";".into());
                }

                let descriptors = descriptors
                    .iter()
                    .map(|descriptor| {
                        descriptor
                            .render(context)
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

                let mut fields_context = context.indent_inc();
                let fields = fields
                    .iter()
                    .map(|property| {
                        property
                            .render(&mut fields_context)
                            .map(|result| result + ",")
                    })
                    .collect::<Result<Vec<String>>>()?
                    .join("\n");

                Ok(format!(
                    " {{\n{fields}\n{indent}}}",
                    indent = context.indent_str()
                ))
            }

            RSStructFields::Unresolved(span, _, _) => {
                Err(RSError::UnresolvedStructFields(span.clone()).into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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
                    descriptor: RSDescriptor::Primitive(RSPrimitive::IntSize),
                }
            ])
            .render(&mut Default::default())
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
                .render(&mut Default::default())
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
                    descriptor: RSDescriptor::Primitive(RSPrimitive::IntSize),
                }
            ])
            .render(&mut RSRenderContext::default().indent_inc())
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
                RSDescriptor::Primitive(RSPrimitive::IntSize),
            ])
            .render(&mut Default::default())
            .unwrap(),
            "(pub String, pub isize);"
        );
    }

    #[test]
    fn test_render_empty_tuple() {
        assert_eq!(
            RSStructFields::Tuple(vec![])
                .render(&mut Default::default())
                .unwrap(),
            ";"
        );
    }
}
