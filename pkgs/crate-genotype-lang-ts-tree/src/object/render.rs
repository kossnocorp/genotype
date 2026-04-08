use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsObject {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let properties = self
            .properties
            .iter()
            .map(|property| property.render(state.indent_inc(), context))
            .collect::<Result<Vec<_>>>()?
            .join(",\n");

        if context.is_zod_mode() {
            return Ok(format!(
                "z.object({{\n{properties}{}{}",
                if !properties.is_empty() { "\n" } else { "" },
                state.indent_format("})")
            ));
        }

        Ok(format!(
            "{{\n{properties}{}{}",
            if !properties.is_empty() { "\n" } else { "" },
            state.indent_format("}")
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_empty() {
        assert_snapshot!(
            render_node(Tst::object(vec![])),
            @"
        {
        }
        "
        );
    }

    #[test]
    fn test_render_properties() {
        assert_snapshot!(
            render_node(
                Tst::object(vec![
                    Tst::property("name", Tst::primitive_string()),
                    Tst::property_optional("age", Tst::primitive_number()),
                ]),
            ),
            @"
        {
          name: string,
          age?: number
        }
        "
        );
    }

    #[test]
    fn test_render_indent() {
        assert_snapshot!(
            Tst::object(vec![
                Tst::property("name", Tst::primitive_string()),
                Tst::property_optional("age", Tst::primitive_number()),
            ])
            .render(
                TsRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            @"
        {
            name: string,
            age?: number
          }
        "
        );
    }

    #[test]
    fn test_render_zod_mode() {
        let mut context = Tst::render_context_zod();

        assert_snapshot!(
            render_node_with(
                Tst::object(vec![Tst::property("name", Tst::primitive_string())]),
                &mut context,
            ),
            @"
        z.object({
          name: z.string()
        })
        "
        );
    }

    #[test]
    fn test_render_zod_mode_multiple_fields() {
        let mut context = Tst::render_context_zod();

        assert_snapshot!(
            render_node_with(
                Tst::object(vec![
                    Tst::property("name", Tst::primitive_string()),
                    Tst::property_optional(
                        "address",
                        Tst::object(vec![Tst::property("name", Tst::primitive_string())]),
                    ),
                ]),
                &mut context,
            ),
            @"
        z.object({
          name: z.string(),
          address: z.object({
            name: z.string()
          }).optional()
        })
        "
        );
    }

    #[test]
    fn test_render_zod_mode_no_fields() {
        let mut context = Tst::render_context_zod();

        assert_snapshot!(
            render_node_with(Tst::object(vec![]), &mut context),
            @"
        z.object({
        })
        "
        );
    }
}
