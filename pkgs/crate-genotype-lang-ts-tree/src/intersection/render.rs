use crate::prelude::internal::*;

impl<'context> GtlRender<'context, TsRenderTypes> for TsIntersection {
    fn render(
        &self,
        state: TsRenderState,
        context: &mut TsRenderContext,
    ) -> TsRenderResult<String> {
        let descriptor_state = match context.mode() {
            TsMode::Effect => state.indent_inc(),
            TsMode::Zod | TsMode::Types => state,
        };
        let descriptors = self
            .descriptors
            .iter()
            .map(|d| d.render(descriptor_state, context))
            .collect::<Result<Vec<_>, _>>()?;

        match context.mode() {
            TsMode::Effect => {
                let fields = descriptors
                    .iter()
                    .skip(1)
                    .chain(descriptors.first())
                    .map(|descriptor| {
                        descriptor_state.indent_format(&format!("...{descriptor}.fields"))
                    })
                    .collect::<Vec<_>>()
                    .join(",\n");
                Ok(format!(
                    "Schema.Struct({{\n{fields}{}{}",
                    if fields.is_empty() { "" } else { "\n" },
                    state.indent_format("})"),
                ))
            }

            TsMode::Zod => {
                let mut iterator = descriptors.into_iter();
                let first = iterator.next().unwrap_or_else(|| "z.any()".into());
                Ok(iterator.fold(first, |acc, descriptor| {
                    format!("z.intersection({acc}, {descriptor})")
                }))
            }
            TsMode::Types => Ok(descriptors.join(" & ")),
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

    #[test]
    fn test_render_effect_extension_order() {
        assert_snapshot!(
            render_node_with(
                Tst::intersection(vec_into![
                    Tst::object(vec![Tst::property("name", Tst::primitive_string())]),
                    Tst::reference("First"),
                    Tst::reference("Second"),
                ]),
                &mut Tst::render_context_effect(),
            ),
            @r#"
        Schema.Struct({
          ...First.fields,
          ...Second.fields,
          ...Schema.Struct({
            name: Schema.String
          }).fields
        })
        "#
        );
    }

    #[test]
    fn test_render_effect_generic_extension() {
        assert_snapshot!(
            render_node_with(
                Tst::intersection(vec_into![
                    Tst::object(vec![]),
                    Tst::reference_with_arguments("Base", vec![Tst::primitive_string().into()]),
                ]),
                &mut Tst::render_context_effect(),
            ),
            @r#"
        Schema.Struct({
          ...Base(Schema.String).fields,
          ...Schema.Struct({
          }).fields
        })
        "#
        );
    }

    #[test]
    fn test_render_effect_nested_intersection() {
        assert_snapshot!(
            render_node_with(
                Tst::intersection(vec_into![
                    Tst::object(vec![]),
                    Tst::intersection(vec_into![
                        Tst::object(vec![]),
                        Tst::reference("Base"),
                    ]),
                ]),
                &mut Tst::render_context_effect(),
            ),
            @r#"
        Schema.Struct({
          ...Schema.Struct({
            ...Base.fields,
            ...Schema.Struct({
            }).fields
          }).fields,
          ...Schema.Struct({
          }).fields
        })
        "#
        );
    }

    #[test]
    fn test_render_effect_nested_property() {
        assert_snapshot!(
            render_node_with(
                Tst::object(vec![Tst::property(
                    "details",
                    Tst::intersection(vec_into![
                        Tst::object(vec![Tst::property("name", Tst::primitive_string())]),
                        Tst::reference("Base"),
                    ]),
                )]),
                &mut Tst::render_context_effect(),
            ),
            @r#"
        Schema.Struct({
          details: Schema.Struct({
            ...Base.fields,
            ...Schema.Struct({
              name: Schema.String
            }).fields
          })
        })
        "#
        );
    }

    #[test]
    fn test_render_effect_empty_intersection() {
        assert_snapshot!(
            render_node_with(Tst::intersection(vec![]), &mut Tst::render_context_effect()),
            @r#"
        Schema.Struct({
        })
        "#
        );
    }
}
