use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsInterface {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let name = self.name.render(state, context)?;

        if context.is_zod_mode() {
            let properties = self
                .properties
                .iter()
                .map(|property| property.render(state.indent_inc(), context))
                .collect::<Result<Vec<_>>>()?
                .join(",\n");

            let object_shape = format!(
                "{{\n{properties}{}{}",
                if properties.len() > 0 { "\n" } else { "" },
                state.indent_format("}")
            );

            let mut schema = if self.extensions.is_empty() {
                format!("z.object({object_shape})")
            } else {
                let first_extension = self.extensions.first().unwrap();
                first_extension
                    .reference
                    .identifier
                    .render(state, context)?
            };

            for extension in self.extensions.iter().skip(1) {
                let extension_name = extension.reference.identifier.render(state, context)?;
                schema.push_str(&format!(".extend({extension_name}.shape)"));
            }

            if !self.extensions.is_empty() {
                schema.push_str(&format!(".extend({object_shape})"));
            }

            let schema = TsDoc::with_doc(
                &self.doc,
                state,
                context,
                format!("export const {name} = {schema};"),
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

        let properties = self
            .properties
            .iter()
            .map(|property| property.render(state.indent_inc(), context))
            .collect::<Result<Vec<_>>>()?
            .iter()
            .map(|property| format!("{property};"))
            .collect::<Vec<_>>()
            .join("\n");

        let object = format!(
            "{{\n{properties}{}{}",
            if properties.len() > 0 { "\n" } else { "" },
            state.indent_format("}")
        );

        match context.config.prefer {
            TsPrefer::Interface => {
                let extensions = self
                    .extensions
                    .iter()
                    .map(|extension| extension.render(state, context))
                    .collect::<Result<Vec<_>>>()?
                    .join(", ");

                let extends = if extensions.len() > 0 {
                    format!(" extends {extensions}")
                } else {
                    "".into()
                };

                TsDoc::with_doc(
                    &self.doc,
                    state,
                    context,
                    format!(
                        "{}export interface {name}{extends} {object}",
                        state.indent_str(),
                    ),
                    false,
                )
            }

            TsPrefer::Alias => {
                let extensions = self
                    .extensions
                    .iter()
                    .map(|extension| extension.render(state, context))
                    .collect::<Result<Vec<_>>>()?;

                let descriptor = if extensions.is_empty() {
                    object
                } else {
                    format!("{} & {object}", extensions.join(" & "))
                };

                TsDoc::with_doc(
                    &self.doc,
                    state,
                    context,
                    format!("{}export type {name} = {descriptor};", state.indent_str()),
                    false,
                )
            }
        }
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
            render_node(Tst::interface("Name", vec![])),
            @"
        export interface Name {
        }
        "
        );
    }

    #[test]
    fn test_render_properties() {
        assert_snapshot!(
            render_node(
                Tst::interface(
                    "Name",
                    vec![
                        Tst::property("name", Tst::primitive_string()),
                        Tst::property_optional("age", Tst::primitive_number()),
                    ],
                ),
            ),
            @"
        export interface Name {
          name: string;
          age?: number;
        }
        "
        );
    }

    #[test]
    fn test_render_indent() {
        assert_snapshot!(
            Tst::interface(
                "Name",
                vec![
                    Tst::property("name", Tst::primitive_string()),
                    Tst::property_optional("age", Tst::primitive_number()),
                ],
            )
            .render(
                TsRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            @"
        export interface Name {
          name: string;
          age?: number;
        }
        "
        );
    }

    #[test]
    fn test_render_extensions() {
        assert_snapshot!(
            render_node(
                TsInterface {
                    extensions: vec![Tst::extension("Hello"), Tst::extension("World")],
                    properties: vec![Tst::property("name", Tst::primitive_string())],
                    ..Tst::interface("Name", vec![])
                },
            ),
            @"
        export interface Name extends Hello, World {
          name: string;
        }
        "
        );
    }

    #[test]
    fn test_render_doc() {
        assert_snapshot!(
            render_node(
                TsInterface {
                    doc: Tst::some_doc("Hello, world!"),
                    ..Tst::interface("Name", vec![])
                },
            ),
            @"
        /** Hello, world! */
        export interface Name {
        }
        "
        );
    }

    #[test]
    fn test_render_alias_preference() {
        let mut context = TsRenderContext {
            config: &TsConfigLang {
                prefer: TsPrefer::Alias,
                ..Default::default()
            },
            ..Default::default()
        };

        assert_snapshot!(
            render_node_with(
                Tst::interface(
                    "Name",
                    vec![
                        Tst::property("name", Tst::primitive_string()),
                        Tst::property_optional("age", Tst::primitive_number()),
                    ],
                ),
                &mut context,
            ),
            @"
        export type Name = {
          name: string;
          age?: number;
        };
        "
        );
    }

    #[test]
    fn test_render_alias_preference_extensions() {
        let mut context = TsRenderContext {
            config: &TsConfigLang {
                prefer: TsPrefer::Alias,
                ..Default::default()
            },
            ..Default::default()
        };

        assert_snapshot!(
            render_node_with(
                TsInterface {
                    extensions: vec![Tst::extension("Hello"), Tst::extension("World")],
                    properties: vec![Tst::property("name", Tst::primitive_string())],
                    ..Tst::interface("Name", vec![])
                },
                &mut context,
            ),
            @"
        export type Name = Hello & World & {
          name: string;
        };
        "
        );
    }

    #[test]
    fn test_render_zod_mode() {
        let mut context = Tst::render_context_zod();

        assert_snapshot!(
            render_node_with(
                Tst::interface(
                    "Name",
                    vec![Tst::property_optional("age", Tst::primitive_number())],
                ),
                &mut context,
            ),
            @"
        export const Name = z.object({
          age: z.number().optional()
        });

        export type Name = z.infer<typeof Name>;
        "
        );
    }

    #[test]
    fn test_render_zod_mode_single_extension() {
        let mut context = Tst::render_context_zod();

        assert_snapshot!(
            render_node_with(
                TsInterface {
                    extensions: vec![Tst::extension("Base")],
                    properties: vec![Tst::property_optional("age", Tst::primitive_number())],
                    ..Tst::interface("Name", vec![])
                },
                &mut context,
            ),
            @"
        export const Name = Base.extend({
          age: z.number().optional()
        });

        export type Name = z.infer<typeof Name>;
        "
        );
    }

    #[test]
    fn test_render_zod_mode_multiple_extensions() {
        let mut context = Tst::render_context_zod();

        assert_snapshot!(
            render_node_with(
                TsInterface {
                    extensions: vec![Tst::extension("Base"), Tst::extension("Extra")],
                    properties: vec![Tst::property_optional("age", Tst::primitive_number())],
                    ..Tst::interface("Name", vec![])
                },
                &mut context,
            ),
            @"
        export const Name = Base.extend(Extra.shape).extend({
          age: z.number().optional()
        });

        export type Name = z.infer<typeof Name>;
        "
        );
    }

    #[test]
    fn test_render_zod_mode_doc() {
        let mut context = Tst::render_context_zod();

        assert_snapshot!(
            render_node_with(
                assign!(
                    Tst::interface("Name", vec![]),
                    doc = Tst::some_doc("Hello, world!")
                ),
                &mut context,
            ),
            @"
        /** Hello, world! */
        export const Name = z.object({
        });

        /** Hello, world! */
        export type Name = z.infer<typeof Name>;
        "
        );
    }
}
