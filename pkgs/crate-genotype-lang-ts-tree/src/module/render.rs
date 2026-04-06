use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsModule {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let mut imports = self
            .imports
            .iter()
            .map(|import| import.render(state, context))
            .collect::<Result<Vec<_>>>()?;

        if context.is_zod_mode() {
            imports.insert(0, r#"import { z } from "zod";"#.into());
        }

        let imports = Self::join_imports(&imports);
        let has_imports = !imports.is_empty();

        let definitions = Self::join_definitions(
            &self
                .definitions
                .iter()
                .map(|definition| definition.render(state, context))
                .collect::<Result<Vec<_>>>()?,
        );
        let has_definitions = !definitions.is_empty();

        let mut str = imports;

        if has_imports && has_definitions {
            str.push_str("\n\n");
        }

        str.push_str(&definitions);

        if has_imports || has_definitions {
            str.push_str("\n");
        }

        TsDoc::with_doc(&self.doc, state, context, str, true)
    }
}

impl GtlRenderModule for TsModule {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            render_node(
                Tst::module(
                    vec![
                        Tst::import_default("../path/to/module", "Name"),
                        Tst::import_named(
                            "../path/to/module",
                            vec![
                                Tst::import_name("Name"),
                                Tst::import_alias("Name", "Alias"),
                            ],
                        ),
                    ],
                    vec_into![
                        Tst::alias("Name", Tst::primitive_string()),
                        Tst::interface(
                            "Name",
                            vec![
                                Tst::property("name", Tst::primitive_string()),
                                Tst::property_optional("age", Tst::primitive_number()),
                            ],
                        ),
                    ],
                ),
            ),
            @r#"
        import Name from "../path/to/module.js";
        import { Name, Name as Alias } from "../path/to/module.js";

        export type Name = string;

        export interface Name {
          name: string;
          age?: number;
        }
        "#
        );
    }

    #[test]
    fn test_render_doc() {
        assert_snapshot!(
            render_node(
                TsModule {
                    doc: Tst::some_doc("Hello, world!"),
                    imports: vec![Tst::import_default("../path/to/module", "Name")],
                    definitions: vec_into![Tst::alias("Name", Tst::primitive_string())]
                },
            ),
            @r#"
        /** Hello, world! */

        import Name from "../path/to/module.js";

        export type Name = string;
        "#
        );
    }

    #[test]
    fn test_render_zod_mode() {
        let mut context = Tst::render_context_zod();

        assert_snapshot!(
            render_node_with(
                Tst::module(
                    vec![],
                    vec_into![
                        Tst::alias("Primitive", Tst::primitive_string()),
                        assign!(
                            Tst::alias("Literal", Tst::literal_string(r#"Hello, "world"!"#)),
                            doc = Tst::some_doc("It's a literal string")
                        )
                    ],
                ),
                &mut context,
            ),
            @r#"
        import { z } from "zod";

        export const Primitive = z.string();

        export type Primitive = z.infer<typeof Primitive>;

        /** It's a literal string */
        export const Literal = z.literal("Hello, \"world\"!");

        /** It's a literal string */
        export type Literal = z.infer<typeof Literal>;
        "#
        );
    }
}
