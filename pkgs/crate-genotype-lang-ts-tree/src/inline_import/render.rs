use crate::prelude::internal::*;

impl<'context> GtlRender<'context, TsRenderTypes> for TsInlineImport {
    fn render(
        &self,
        state: TsRenderState,
        context: &mut TsRenderContext,
    ) -> TsRenderResult<String> {
        let name = self.name.render(state, context)?;
        let arguments = self
            .arguments
            .iter()
            .map(|argument| argument.render(state, context))
            .collect::<Result<Vec<_>, _>>()?
            .join(", ");

        let call = if self.arguments.is_empty() {
            name.clone()
        } else {
            match context.mode() {
                TsMode::Zod | TsMode::Effect => format!("{name}({arguments})"),
                TsMode::Types => format!("{name}<{arguments}>"),
            }
        };

        match context.mode() {
            TsMode::Zod | TsMode::Effect => Ok(call),
            TsMode::Types => {
                let path = self.path.render(state, context)?;
                Ok(format!(r#"import("{path}").{call}"#))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render() {
        assert_snapshot!(
            render_node(Tst::inline_import("./path/to/module", "Name")),
            @r#"import("./path/to/module.js").Name"#
        );
    }

    #[test]
    fn test_render_with_arguments() {
        assert_snapshot!(
            render_node(Tst::inline_import_with_arguments(
                "./path/to/module",
                "Name",
                vec![Tst::primitive_string().into()]
            )),
            @r#"import("./path/to/module.js").Name<string>"#
        );

        assert_snapshot!(
            render_node(Tst::inline_import_with_arguments(
                "./path/to/module",
                "Name",
                vec![Tst::primitive_string().into(), Tst::primitive_number().into()]
            )),
            @r#"import("./path/to/module.js").Name<string, number>"#
        );
    }

    #[test]
    fn test_render_zod() {
        assert_snapshot!(
            render_node_with(Tst::inline_import("./path/to/module", "Name"), &mut Tst::render_context_zod()),
            @"Name"
        );
    }

    #[test]
    fn test_render_zod_with_arguments() {
        assert_snapshot!(
            render_node_with(
                Tst::inline_import_with_arguments(
                    "./path/to/module",
                    "Name",
                    vec![Tst::primitive_string().into()]
                ),
                &mut Tst::render_context_zod(),
            ),
            @"Name(z.string())"
        );
    }

    #[test]
    fn test_render_effect() {
        assert_snapshot!(
            render_node_with(Tst::inline_import("./path/to/module", "Name"), &mut Tst::render_context_effect()),
            @"Name"
        );
    }

    #[test]
    fn test_render_effect_with_arguments() {
        assert_snapshot!(
            render_node_with(
                Tst::inline_import_with_arguments(
                    "./path/to/module",
                    "Name",
                    vec![Tst::primitive_string().into()]
                ),
                &mut Tst::render_context_effect(),
            ),
            @"Name(Schema.String)"
        );
    }
}
