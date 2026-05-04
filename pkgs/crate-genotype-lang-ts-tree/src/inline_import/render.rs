use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsInlineImport {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let name = self.name.render(state, context)?;
        let arguments = self
            .arguments
            .iter()
            .map(|argument| argument.render(state, context))
            .collect::<Result<Vec<_>>>()?
            .join(", ");

        let call = if self.arguments.is_empty() {
            name.clone()
        } else if context.is_zod_mode() {
            format!("{name}({arguments})")
        } else {
            format!("{name}<{arguments}>")
        };

        if context.is_zod_mode() {
            return Ok(call);
        }

        let path = self.path.render(state, context)?;

        Ok(format!(r#"import("{path}").{call}"#))
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
}
