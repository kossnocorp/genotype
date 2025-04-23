use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for RSStructFields {
    type RenderState = RSRenderState;

    type RenderContext = RSRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        match self {
            RSStructFields::Tuple(descriptors) => {
                if descriptors.len() == 0 {
                    return Ok(";".into());
                }

                let descriptors = descriptors
                    .iter()
                    .map(|descriptor| {
                        descriptor
                            .render(state, context)
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

                let fields = fields
                    .iter()
                    .map(|property| {
                        property
                            .render(state.indent_inc(), context)
                            .map(|result| result + ",")
                    })
                    .collect::<Result<Vec<String>>>()?
                    .join("\n");

                Ok(format!(
                    " {{\n{fields}\n{indent}}}",
                    indent = state.indent_str()
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
            .render(Default::default(), &mut Default::default())
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
                .render(Default::default(), &mut Default::default())
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
            .render(
                RSRenderState::default().indent_inc(),
                &mut Default::default()
            )
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
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            "(pub String, pub isize);"
        );
    }

    #[test]
    fn test_render_empty_tuple() {
        assert_eq!(
            RSStructFields::Tuple(vec![])
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            ";"
        );
    }
}
