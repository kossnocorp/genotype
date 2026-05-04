use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsInterface {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        if context.is_zod_mode() {
            return self.render_zod(state, context);
        }
        self.render_type(state, context)
    }
}

impl TsInterface {
    fn render_type(&self, state: TsRenderState, context: &mut TsRenderContext) -> Result<String> {
        let name = self.name.render(state, context)?;
        let generic_names = self.generic_names();
        let generic_params = self.render_generic_params(&generic_names);
        let object_body = self.render_object_body(&state, context)?;
        let extension_names = self.render_extension_names(state, context)?;

        let code = match context.config.prefer {
            TsPrefer::Interface => {
                let extensions = extension_names.join(", ");
                let extends = if !extensions.is_empty() {
                    format!(" extends {extensions}")
                } else {
                    "".into()
                };

                format!(
                    "{}export interface {name}{generic_params}{extends} {object_body}",
                    state.indent_str(),
                )
            }

            TsPrefer::Alias => {
                let descriptor = if extension_names.is_empty() {
                    object_body
                } else {
                    format!("{} & {object_body}", extension_names.join(" & "))
                };

                format!(
                    "{}export type {name}{generic_params} = {descriptor};",
                    state.indent_str()
                )
            }
        };

        TsDoc::with_doc(&self.doc, state, context, code, false)
    }

    fn render_zod(&self, state: TsRenderState, context: &mut TsRenderContext) -> Result<String> {
        let name = self.name.render(state, context)?;
        let generic_names = self.generic_names();

        let properties = self
            .properties
            .iter()
            .map(|property| property.render(state.indent_inc(), context))
            .collect::<Result<Vec<_>>>()?
            .join(",\n");

        let object_shape = format!(
            "{{\n{properties}{}{}",
            if !properties.is_empty() { "\n" } else { "" },
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

        let schema = if generic_names.is_empty() {
            format!("export const {name} = {schema};")
        } else {
            let generic_params = render_zod_generic_params(&generic_names);
            let params = render_zod_value_params(&generic_names);
            format!("export const {name} = {generic_params}({params}) => {schema};")
        };
        let schema = TsDoc::with_doc(&self.doc, state, context, schema, false)?;

        let r#type = if generic_names.is_empty() {
            format!("export type {name} = z.infer<typeof {name}>;")
        } else {
            let generic_params = render_zod_generic_params(&generic_names);
            let return_type_args = generic_names.join(", ");
            format!(
                "export type {name}{generic_params} = z.infer<ReturnType<typeof {name}<{return_type_args}>>>;"
            )
        };
        let r#type = TsDoc::with_doc(&self.doc, state, context, r#type, false)?;

        return Ok(format!("{schema}\n\n{type}"));
    }

    fn render_extension_names(
        &self,
        state: TsRenderState,
        context: &mut TsRenderContext,
    ) -> Result<Vec<String>> {
        let names = self
            .extensions
            .iter()
            .map(|extension| extension.render(state, context))
            .collect::<Result<Vec<_>>>()?;
        Ok(names)
    }

    fn render_object_body(
        &self,
        state: &TsRenderState,
        context: &mut TsRenderContext,
    ) -> Result<String> {
        let properties = self.render_properties(state, context)?;
        let body = format!(
            "{{\n{properties}{}{}",
            if !properties.is_empty() { "\n" } else { "" },
            state.indent_format("}")
        );
        Ok(body)
    }

    fn render_properties(
        &self,
        state: &TsRenderState,
        context: &mut TsRenderContext,
    ) -> Result<String> {
        let properties = self
            .properties
            .iter()
            .map(|property| property.render(state.indent_inc(), context))
            .collect::<Result<Vec<_>>>()?
            .iter()
            .map(|property| format!("{property};"))
            .collect::<Vec<_>>()
            .join("\n");
        Ok(properties)
    }

    fn render_generic_params(&self, names: &[String]) -> String {
        if names.is_empty() {
            String::new()
        } else {
            format!("<{}>", names.join(", "))
        }
    }

    fn generic_names(&self) -> Vec<String> {
        self.generics
            .iter()
            .map(|generic| {
                generic
                    .render(TsRenderState::default(), &mut Default::default())
                    .unwrap()
            })
            .collect()
    }
}

fn render_zod_generic_params(generic_names: &[String]) -> String {
    format!(
        "<{}>",
        generic_names
            .iter()
            .map(|generic| format!("{generic} extends z.ZodTypeAny"))
            .collect::<Vec<_>>()
            .join(", ")
    )
}

fn render_zod_value_params(generic_names: &[String]) -> String {
    generic_names
        .iter()
        .map(|generic| format!("{generic}: {generic}"))
        .collect::<Vec<_>>()
        .join(", ")
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

        assert_snapshot!(
            render_node(
                Tst::interface_with_generics(
                    "Response",
                    vec!["Payload"],
                    vec![Tst::property("value", Tst::reference("Payload"))],
                ),
            ),
            @"
        export interface Response<Payload> {
          value: Payload;
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
        assert_snapshot!(
            render_node_with(
                Tst::interface(
                    "Name",
                    vec![
                        Tst::property("name", Tst::primitive_string()),
                        Tst::property_optional("age", Tst::primitive_number()),
                    ],
                ),
                &mut Tst::render_context_alias(),
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
    fn test_render_zod() {
        assert_snapshot!(
            render_node_with(
                Tst::interface(
                    "Name",
                    vec![Tst::property_optional("age", Tst::primitive_number())],
                ),
                &mut Tst::render_context_zod(),
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
    fn test_render_zod_with_arguments() {
        assert_snapshot!(
            render_node_with(
                Tst::interface_with_generics(
                    "Response",
                    vec!["Payload"],
                    vec![Tst::property("value", Tst::reference("Payload"))],
                ),
                &mut Tst::render_context_zod(),
            ),
            @"
        export const Response = <Payload extends z.ZodTypeAny>(Payload: Payload) => z.object({
          value: Payload
        });

        export type Response<Payload extends z.ZodTypeAny> = z.infer<ReturnType<typeof Response<Payload>>>;
        "
        );
    }

    #[test]
    fn test_render_zod_single_extension() {
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
    fn test_render_zod_multiple_extensions() {
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
    fn test_render_zod_doc() {
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
