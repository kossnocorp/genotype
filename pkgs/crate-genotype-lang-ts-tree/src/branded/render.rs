use crate::prelude::internal::*;
use heck::ToLowerCamelCase;

impl<'a> GtlRender<'a> for TsBranded {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        if context.is_zod_mode() {
            let name = self.name.render(state, context)?;
            let primitive = self.primitive.render(state, context)?;
            let schema = TsDoc::with_doc(
                &self.doc,
                state,
                context,
                format!("export const {name} = {primitive}.brand<\"{name}\">();"),
                false,
            )?;
            let r#type = TsDoc::with_doc(
                &self.doc,
                state,
                context,
                format!("export type {name} = z.infer<typeof {name}>;"),
                false,
            )?;

            return Ok(format!("{schema}\n\n{type}"));
        }

        let brand_name = format!("{brand}Brand", brand = self.name.0).to_lower_camel_case();

        let mut blocks = vec![];

        if let Some(doc) = &self.doc {
            blocks.push(doc.render(state, context)?);
        }

        blocks.push(format!(
            "{indent}export type {name} = {primitive} & {{ [{brand_name}]: true }};",
            indent = state.indent_str(),
            name = self.name.render(state, context)?,
            primitive = self.primitive.render(state, context)?
        ));

        blocks.push(format!(
            "{indent}declare const {brand_name}: unique symbol;",
            indent = state.indent_str(),
        ));

        Ok(blocks.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_primitive() {
        assert_snapshot!(
            render_node(Tst::branded("Version", Tst::primitive_number())),
            @"
        export type Version = number & { [versionBrand]: true };
        declare const versionBrand: unique symbol;
        "
        );
    }

    #[test]
    fn test_render_doc() {
        assert_snapshot!(
            render_node(
                TsBranded {
                    doc: Tst::some_doc("Object version."),
                    ..Tst::branded("Version", Tst::primitive_number())
                },
            ),
            @"
        /** Object version. */
        export type Version = number & { [versionBrand]: true };
        declare const versionBrand: unique symbol;
        "
        );
    }

    #[test]
    fn test_render_indent() {
        assert_snapshot!(
            TsBranded {
                doc: Tst::some_doc("Object version."),
                ..Tst::branded("Version", Tst::primitive_number())
            }
            .render(
                TsRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            @"
        /** Object version. */
        export type Version = number & { [versionBrand]: true };
        declare const versionBrand: unique symbol;
        "
        );
    }

    #[test]
    fn test_render_zod_mode() {
        let mut context = Tst::render_context_zod();

        assert_snapshot!(
            render_node_with(Tst::branded("Version", Tst::primitive_number()), &mut context),
            @r#"
        export const Version = z.number().brand<"Version">();

        export type Version = z.infer<typeof Version>;
        "#
        );
    }

    #[test]
    fn test_render_zod_mode_doc() {
        let mut context = Tst::render_context_zod();

        assert_snapshot!(
            render_node_with(
                TsBranded {
                    doc: Tst::some_doc("Object version."),
                    ..Tst::branded("Version", Tst::primitive_number())
                },
                &mut context,
            ),
            @r#"
        /** Object version. */
        export const Version = z.number().brand<"Version">();

        /** Object version. */
        export type Version = z.infer<typeof Version>;
        "#
        );
    }
}
