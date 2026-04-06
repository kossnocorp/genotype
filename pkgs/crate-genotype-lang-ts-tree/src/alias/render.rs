use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsAlias {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let name = self.name.render(state, context)?;

        match context.is_zod_mode() {
            true => self.render_zod(&name, state, context),

            false => {
                let descriptor = self.descriptor.render(state, context)?;

                TsDoc::with_doc(
                    &self.doc,
                    state,
                    context,
                    format!("export type {name} = {descriptor};"),
                    false,
                )
            }
        }
    }
}

impl TsAlias {
    fn render_zod<'a>(
        &self,
        name: &String,
        state: TsRenderState,
        context: &mut TsRenderContext<'a>,
    ) -> Result<String> {
        let refs_scan = self.descriptor.scan_references();
        match refs_scan.has_self_recursive {
            true => self.render_zod_recursive(name, state, context),

            false => {
                let descriptor = self.descriptor.render(state, context)?;
                let schema = TsDoc::with_doc(
                    &self.doc,
                    state,
                    context,
                    format!("export const {name} = {descriptor};"),
                    false,
                )?;
                let r#type = TsDoc::with_doc(
                    &self.doc,
                    state,
                    context,
                    format!("export type {name} = z.infer<typeof {name}>;"),
                    false,
                )?;

                Ok(format!("{schema}\n\n{type}"))
            }
        }
    }

    fn render_zod_recursive<'a>(
        &self,
        name: &String,
        state: TsRenderState,
        context: &mut TsRenderContext<'a>,
    ) -> Result<String> {
        let zod_descriptor = self.descriptor.render(state, context)?;

        let schema = TsDoc::with_doc(
            &self.doc,
            state,
            context,
            format!("export const {name}: z.ZodType<{name}> = z.lazy(() => {zod_descriptor});"),
            false,
        )?;

        let rendered_type = context.with_mode(TsMode::Types, |context| {
            let type_descriptor = self.descriptor.render(state, context)?;

            let rendered_type = TsDoc::with_doc(
                &self.doc,
                state,
                context,
                format!("export type {name} = {type_descriptor};"),
                false,
            )?;
            Ok(rendered_type)
        })?;

        Ok(format!("{rendered_type}\n\n{schema}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            render_node(Tst::alias("Name", Tst::primitive_string())),
            @"export type Name = string;"
        );
    }

    #[test]
    fn test_render_doc() {
        assert_snapshot!(
            render_node(
                TsAlias {
                    doc: Tst::some_doc("Hello, world!"),
                    ..Tst::alias("Name", Tst::primitive_string())
                },
            ),
            @"
        /** Hello, world! */
        export type Name = string;
        "
        );
    }

    #[test]
    fn test_render_zod_mode() {
        let mut context = Tst::render_context_zod();

        assert_snapshot!(
            render_node_with(Tst::alias("Name", Tst::primitive_string()), &mut context),
            @"
        export const Name = z.string();

        export type Name = z.infer<typeof Name>;
        "
        );
    }

    #[test]
    fn test_render_zod_mode_doc() {
        let mut context = Tst::render_context_zod();

        assert_snapshot!(
            render_node_with(
                TsAlias {
                    doc: Tst::some_doc("Hello, world!"),
                    ..Tst::alias("Name", Tst::primitive_string())
                },
                &mut context,
            ),
            @"
        /** Hello, world! */
        export const Name = z.string();

        /** Hello, world! */
        export type Name = z.infer<typeof Name>;
        "
        );
    }

    #[test]
    fn test_render_zod_mode_self_recursive_array() {
        assert_snapshot!(
            render_node_with(
                Tst::alias("SelfRefArray", Tst::array(Tst::reference_self_recursive("SelfRefArray"))),
                &mut Tst::render_context_zod(),
            ),
            @"
        export type SelfRefArray = Array<SelfRefArray>;

        export const SelfRefArray: z.ZodType<SelfRefArray> = z.lazy(() => z.array(SelfRefArray));
        "
        );
    }

    #[test]
    fn test_render_zod_mode_self_recursive_tuple() {
        assert_snapshot!(
            render_node_with(
                Tst::alias(
                    "SelfRefTuple",
                    Tst::union(vec_into![
                        Tst::primitive_null(),
                        Tst::tuple(vec_into![
                            Tst::primitive_string(),
                            Tst::reference_self_recursive("SelfRefTuple"),
                        ]),
                    ]),
                ),
                &mut Tst::render_context_zod(),
            ),
            @"
        export type SelfRefTuple = null | [string, SelfRefTuple];

        export const SelfRefTuple: z.ZodType<SelfRefTuple> = z.lazy(() => z.union([z.null(), z.tuple([z.string(), SelfRefTuple])]));
        "
        );
    }
}
