use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for RsStructFields {
    type RenderState = RsRenderState;

    type RenderContext = RsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        match self {
            RsStructFields::Newtype(descriptors) => {
                if descriptors.is_empty() {
                    return Ok("()".into());
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

            RsStructFields::Resolved(fields) => {
                if fields.is_empty() {
                    return Ok(" {}".into());
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

            RsStructFields::Unit => Ok(";".into()),

            RsStructFields::Unresolved(span, _, _) => {
                Err(RsError::UnresolvedStructFields(*span).into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_fields() {
        assert_snapshot!(
            RsStructFields::Resolved(vec![
                RsField {
                    doc: None,
                    attributes: vec![],
                    name: "name".into(),
                    descriptor: RsDescriptor::Primitive(RsPrimitive::String),
                },
                RsField {
                    doc: None,
                    attributes: vec![],
                    name: "age".into(),
                    descriptor: RsDescriptor::Primitive(RsPrimitive::IntSize),
                }
            ])
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"
         {
            pub name: String,
            pub age: isize,
        }
        "
        );
    }

    #[test]
    fn test_render_empty() {
        assert_snapshot!(
            RsStructFields::Resolved(vec![])
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @" {}"
        );
    }

    #[test]
    fn test_render_indent() {
        assert_snapshot!(
            RsStructFields::Resolved(vec![
                RsField {
                    doc: None,
                    attributes: vec![],
                    name: "name".into(),
                    descriptor: RsDescriptor::Primitive(RsPrimitive::String),
                },
                RsField {
                    doc: None,
                    attributes: vec![],
                    name: "age".into(),
                    descriptor: RsDescriptor::Primitive(RsPrimitive::IntSize),
                }
            ])
            .render(
                RsRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            @"
        {
               pub name: String,
               pub age: isize,
           }
        "
        );
    }

    #[test]
    fn test_render_newtype() {
        assert_snapshot!(
            RsStructFields::Newtype(vec![
                RsDescriptor::Primitive(RsPrimitive::String),
                RsDescriptor::Primitive(RsPrimitive::IntSize),
            ])
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"(pub String, pub isize);"
        );
    }

    #[test]
    fn test_render_empty_newtype() {
        assert_snapshot!(
            RsStructFields::Newtype(vec![])
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"()"
        );
    }

    #[test]
    fn test_render_unit() {
        assert_snapshot!(
            RsStructFields::Unit
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @";"
        );
    }
}
