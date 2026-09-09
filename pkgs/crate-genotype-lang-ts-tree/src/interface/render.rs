use crate::prelude::internal::*;

impl<'context> GtlRender<'context, TsRenderTypes> for TsInterface {
    fn render(
        &self,
        state: TsRenderState,
        context: &mut TsRenderContext,
    ) -> TsRenderResult<String> {
        match context.mode() {
            TsMode::Effect => self.render_effect(state, context),
            TsMode::Zod => self.render_zod(state, context),
            TsMode::Types => self.render_type(state, context),
        }
    }
}

impl TsInterface {
    fn render_type(
        &self,
        state: TsRenderState,
        context: &mut TsRenderContext,
    ) -> TsRenderResult<String> {
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

    fn render_effect(
        &self,
        state: TsRenderState,
        context: &mut TsRenderContext,
    ) -> TsRenderResult<String> {
        let name = self.name.render(state, context)?;
        let generic_names = self.generic_names();

        let shape_state = if self.extensions.is_empty() {
            state
        } else {
            state.indent_inc()
        };
        let properties = self
            .properties
            .iter()
            .map(|property| property.render(shape_state.indent_inc(), context))
            .collect::<Result<Vec<_>, _>>()?
            .join(",\n");

        let object_shape = format!(
            "{{\n{properties}{}{}",
            if !properties.is_empty() { "\n" } else { "" },
            shape_state.indent_format("}")
        );

        let mut fields = Vec::new();
        for extension in &self.extensions {
            let reference = extension.reference.render(state, context)?;
            fields.push(shape_state.indent_format(&format!("...{reference}.fields")));
        }
        fields.push(shape_state.indent_format(&format!("...{object_shape}")));
        let schema = if self.extensions.is_empty() {
            format!("Schema.Struct({object_shape})")
        } else {
            format!(
                "Schema.Struct({{\n{}\n{}",
                fields.join(",\n"),
                state.indent_format("})")
            )
        };

        let recursive = self.properties.iter().any(|property| {
            let refs = property.descriptor.scan_references();
            refs.has_forward || refs.has_self_recursive
        });
        if recursive && generic_names.is_empty() {
            let type_code =
                context.with_mode(TsMode::Types, |context| self.render_type(state, context))?;
            // Forward references can form cycles through inferred aliases. An outer
            // annotation breaks those cycles; direct recursion keeps Struct fields.
            let declaration = if self
                .properties
                .iter()
                .any(|property| property.descriptor.scan_references().has_forward)
            {
                format!(
                    "export const {name}: Schema.Codec<{name}> = Schema.suspend(() => {schema});"
                )
            } else {
                format!("export const {name} = {schema};")
            };
            let schema = TsDoc::with_doc(&self.doc, state, context, declaration, false)?;
            return Ok(format!("{type_code}\n\n{schema}"));
        }

        let schema = if generic_names.is_empty() {
            format!("export const {name} = {schema};")
        } else {
            let generic_params = render_effect_generic_params(&generic_names);
            let params = render_effect_value_params(&generic_names);
            format!("export const {name} = {generic_params}({params}) => {schema};")
        };
        let schema = TsDoc::with_doc(&self.doc, state, context, schema, false)?;

        let r#type = if generic_names.is_empty() {
            format!("export type {name} = Schema.Schema.Type<typeof {name}>;")
        } else {
            let generic_params = render_effect_generic_params(&generic_names);
            let return_type_args = generic_names.join(", ");
            format!(
                "export type {name}{generic_params} = Schema.Schema.Type<ReturnType<typeof {name}<{return_type_args}>>>;"
            )
        };
        let r#type = TsDoc::with_doc(&self.doc, state, context, r#type, false)?;

        Ok(format!("{schema}\n\n{type}"))
    }

    fn render_zod(
        &self,
        state: TsRenderState,
        context: &mut TsRenderContext,
    ) -> TsRenderResult<String> {
        let name = self.name.render(state, context)?;
        let generic_names = self.generic_names();

        let properties = self
            .properties
            .iter()
            .map(|property| property.render(state.indent_inc(), context))
            .collect::<Result<Vec<_>, _>>()?
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

        Ok(format!("{schema}\n\n{type}"))
    }

    fn render_extension_names(
        &self,
        state: TsRenderState,
        context: &mut TsRenderContext,
    ) -> Result<Vec<String>, TsRenderError> {
        let names = self
            .extensions
            .iter()
            .map(|extension| extension.render(state, context))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(names)
    }

    fn render_object_body(
        &self,
        state: &TsRenderState,
        context: &mut TsRenderContext,
    ) -> TsRenderResult<String> {
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
    ) -> TsRenderResult<String> {
        let properties = self
            .properties
            .iter()
            .map(|property| property.render(state.indent_inc(), context))
            .collect::<Result<Vec<_>, _>>()?
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

fn render_effect_generic_params(generic_names: &[String]) -> String {
    format!(
        "<{}>",
        generic_names
            .iter()
            .map(|generic| format!("{generic} extends Schema.Top"))
            .collect::<Vec<_>>()
            .join(", ")
    )
}

fn render_effect_value_params(generic_names: &[String]) -> String {
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

    #[test]
    fn test_render_effect_with_arguments() {
        assert_snapshot!(
            render_node_with(
                Tst::interface_with_generics(
                    "Response",
                    vec!["Payload"],
                    vec![Tst::property("value", Tst::reference("Payload"))],
                ),
                &mut Tst::render_context_effect(),
            ),
            @"
        export const Response = <Payload extends Schema.Top>(Payload: Payload) => Schema.Struct({
          value: Payload
        });

        export type Response<Payload extends Schema.Top> = Schema.Schema.Type<ReturnType<typeof Response<Payload>>>;
        "
        );
    }

    #[test]
    fn test_render_effect_doc() {
        let mut context = Tst::render_context_effect();

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
        export const Name = Schema.Struct({
        });

        /** Hello, world! */
        export type Name = Schema.Schema.Type<typeof Name>;
        "
        );
    }

    #[test]
    fn test_render_effect_optional() {
        assert_snapshot!(
            render_node_with(Tst::interface("Name", vec![Tst::property_optional("age", Tst::primitive_number())]), &mut Tst::render_context_effect()),
            @r#"
        export const Name = Schema.Struct({
          age: Schema.optionalKey(Schema.Number)
        });

        export type Name = Schema.Schema.Type<typeof Name>;
        "#
        );
    }

    #[test]
    fn test_render_effect_extensions() {
        assert_snapshot!(
            render_node_with(TsInterface { extensions: vec![Tst::extension("Hello"), Tst::extension("World")], ..Tst::interface("Name", vec![Tst::property("name", Tst::primitive_string())]) }, &mut Tst::render_context_effect()),
            @r#"
        export const Name = Schema.Struct({
          ...Hello.fields,
          ...World.fields,
          ...{
            name: Schema.String
          }
        });

        export type Name = Schema.Schema.Type<typeof Name>;
        "#
        );
    }

    #[test]
    fn test_render_effect_self_recursive_preserves_fields() {
        assert_snapshot!(
            render_node_with(Tst::interface("Node", vec![Tst::property_optional("next", Tst::reference_self_recursive("Node"))]), &mut Tst::render_context_effect()),
            @r#"
        export interface Node {
          next?: Node;
        }

        export const Node = Schema.Struct({
          next: Schema.optionalKey(Schema.suspend((): Schema.Codec<Node> => Node))
        });
        "#
        );
    }

    #[test]
    fn test_render_effect_forward_has_codec_boundary() {
        assert_snapshot!(
            render_node_with(Tst::interface("Node", vec![Tst::property("next", Tst::reference_forward("Later"))]), &mut Tst::render_context_effect()),
            @r#"
        export interface Node {
          next: Later;
        }

        export const Node: Schema.Codec<Node> = Schema.suspend(() => Schema.Struct({
          next: Schema.suspend((): Schema.Codec<Later> => Later)
        }));
        "#
        );
    }

    #[test]
    fn test_render_effect_extension_nested_fields() {
        assert_snapshot!(
            render_node_with(
                TsInterface {
                    extensions: vec![Tst::extension("Base")],
                    ..Tst::interface("Name", vec![Tst::property(
                        "details",
                        Tst::object(vec![Tst::property("name", Tst::primitive_string())]),
                    )])
                },
                &mut Tst::render_context_effect(),
            ),
            @r#"
        export const Name = Schema.Struct({
          ...Base.fields,
          ...{
            details: Schema.Struct({
              name: Schema.String
            })
          }
        });

        export type Name = Schema.Schema.Type<typeof Name>;
        "#
        );
    }

    #[test]
    fn test_render_effect_extension_empty_fields() {
        assert_snapshot!(
            render_node_with(
                TsInterface {
                    extensions: vec![Tst::extension("Base")],
                    ..Tst::interface("Name", vec![])
                },
                &mut Tst::render_context_effect(),
            ),
            @r#"
        export const Name = Schema.Struct({
          ...Base.fields,
          ...{
          }
        });

        export type Name = Schema.Schema.Type<typeof Name>;
        "#
        );
    }
}
