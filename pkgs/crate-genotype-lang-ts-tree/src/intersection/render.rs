use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsIntersection {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let descriptors = self
            .descriptors
            .iter()
            .map(|d| d.render(state, context))
            .collect::<Result<Vec<_>>>()?;

        if context.is_zod_mode() {
            let mut iterator = descriptors.into_iter();
            let first = iterator.next().unwrap_or_else(|| "z.any()".into());
            Ok(iterator.fold(first, |acc, descriptor| {
                format!("z.intersection({acc}, {descriptor})")
            }))
        } else {
            Ok(descriptors.join(" & "))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_union() {
        assert_snapshot!(
            render_node(
                Tst::intersection(vec_into![
                    Tst::object(vec![Tst::property("hello", Tst::primitive_string())]),
                    Tst::reference("World"),
                ]),
            ),
            @"
        {
          hello: string
        } & World
        "
        );
    }

    #[test]
    fn test_render_intersection_zod_mode() {
        let mut context = Tst::render_context_zod();

        assert_snapshot!(
            render_node_with(
                Tst::intersection(vec_into![
                    Tst::primitive_string(),
                    Tst::primitive_number(),
                ]),
                &mut context,
            ),
            @"z.intersection(z.string(), z.number())"
        );
    }
}
