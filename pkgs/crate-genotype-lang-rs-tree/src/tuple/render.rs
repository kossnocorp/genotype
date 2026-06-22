use crate::prelude::internal::*;

impl<'context> GtlRender<'context, RsRenderTypes> for RsTuple {
    fn render(
        &self,
        state: RsRenderState,
        context: &mut RsRenderContext,
    ) -> RsRenderResult<String> {
        let descriptors = self
            .descriptors
            .iter()
            .map(|d| d.render(state, context))
            .collect::<Result<Vec<_>, _>>()?
            .join(", ");
        Ok(format!("({descriptors})"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_tuple() {
        assert_snapshot!(
            RsTuple {
                descriptors: vec![
                    RsDescriptor::Primitive(RsPrimitive::String),
                    RsDescriptor::Primitive(RsPrimitive::IntSize),
                ]
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"(String, isize)"
        );
    }

    #[test]
    fn test_render_empty_tuple() {
        assert_snapshot!(
            RsTuple {
                descriptors: vec![]
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"()"
        );
    }
}
