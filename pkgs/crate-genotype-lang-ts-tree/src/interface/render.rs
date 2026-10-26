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

        let resolution = Arc::clone(&context.resolution);
        if let Some(effect) = resolution.effect.interfaces.get(&self.name) {
            let (native, annotate_fields) = match effect {
                TsEffectInterface::Generic {
                    definition,
                    properties,
                } => {
                    return self
                        .render_effect_recursive_generic(definition, properties, state, context);
                }
                TsEffectInterface::Recursive {
                    native,
                    annotate_fields,
                } => (native, annotate_fields),
            };
            let type_code =
                context.with_mode(TsMode::Types, |context| native.render_type(state, context))?;
            // Preserve the Codec inference boundary and expose the actual Struct fields
            // so extensions do not have to inspect a suspended schema.
            let annotation = if *annotate_fields {
                let mut fields = Vec::new();
                for property in &self.properties {
                    let key = property.name.render(state, context)?;
                    let descriptor = context.with_mode(TsMode::Types, |context| {
                        property.descriptor.render(state, context)
                    })?;
                    let codec = format!("Schema.Codec<{descriptor}>");
                    let field = if property.required {
                        codec
                    } else {
                        format!("Schema.optionalKey<{codec}>")
                    };
                    fields.push(
                        state
                            .indent_inc()
                            .indent_inc()
                            .indent_format(&format!("{key}: {field}")),
                    );
                }
                let own_fields = format!(
                    "{{\n{}\n{}}}",
                    fields.join(",\n"),
                    state.indent_inc().indent_str()
                );
                let fields = self.render_effect_fields_type(own_fields, state, context)?;
                format!(
                    ": Schema.Codec<{name}> & {{\n{}fields: {fields}\n{}}}",
                    state.indent_inc().indent_str(),
                    state.indent_str()
                )
            } else {
                String::new()
            };
            let schema = TsDoc::with_doc(
                &self.doc,
                state,
                context,
                format!("export const {name}{annotation} = {schema};"),
                false,
            )?;
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

    fn render_effect_recursive_generic(
        &self,
        generic: &TsEffectGeneric,
        resolved_properties: &[TsEffectGeneric],
        state: TsRenderState,
        context: &mut TsRenderContext,
    ) -> TsRenderResult<String> {
        let name = self.name.render(state, context)?;
        let generics = self.generic_names();
        let types = generic.render_types(
            &name,
            &generics,
            &self.extensions,
            &self.doc,
            state,
            context,
        )?;
        let mut fields = Vec::new();
        let mut properties = Vec::new();
        for extension in &self.extensions {
            let reference = &extension.reference;
            let name = reference.identifier.render(state, context)?;
            let reference = if reference.arguments.is_empty() {
                name
            } else {
                // Match the Codec boundary used by the inherited field types:
                // arbitrary schema parameters can carry optionality modifiers.
                let arguments = reference
                    .arguments
                    .iter()
                    .map(|argument| {
                        let codec = TsEffectGeneric::resolve(argument, &self.generics)
                            .codec
                            .render(&generics, state, context)?;
                        let descriptor = argument.render(state.within_suspend(), context)?;
                        Ok(format!("Schema.suspend((): {codec} => {descriptor})"))
                    })
                    .collect::<TsRenderResult<Vec<_>>>()?;
                format!("{name}({})", arguments.join(", "))
            };
            properties.push(
                state
                    .indent_inc()
                    .indent_format(&format!("...{reference}.fields")),
            );
        }
        for (property, resolved) in self.properties.iter().zip(resolved_properties) {
            let key = property.name.render(state, context)?;
            let codec = resolved
                .codec
                .render(&generics, state.indent_inc(), context)?;
            let descriptor = resolved
                .deferred
                .render(state.indent_inc().within_suspend(), context)?;
            let descriptor = format!("Schema.suspend((): {codec} => {descriptor})");
            let (field, descriptor) = if property.required {
                (codec, descriptor)
            } else {
                (
                    format!("Schema.optionalKey<{codec}>"),
                    format!("Schema.optionalKey({descriptor})"),
                )
            };
            fields.push(state.indent_inc().indent_format(&format!("{key}: {field}")));
            properties.push(TsDoc::with_doc(
                &property.doc,
                state.indent_inc(),
                context,
                state
                    .indent_inc()
                    .indent_format(&format!("{key}: {descriptor}")),
                false,
            )?);
        }
        let params = render_effect_generic_params(&generics);
        let values = render_effect_value_params(&generics);
        let fields = self.render_effect_fields_type(
            format!("{{\n{}\n{}}}", fields.join(",\n"), state.indent_str()),
            state,
            context,
        )?;
        let schema = TsDoc::with_doc(
            &self.doc,
            state,
            context,
            format!(
                "export const {name} = {params}({values}): Schema.Struct<{fields}> => Schema.Struct({{\n{}\n{}}});",
                properties.join(",\n"),
                state.indent_str()
            ),
            false,
        )?;
        Ok(format!("{types}\n\n{schema}"))
    }

    fn render_effect_fields_type(
        &self,
        own: String,
        state: TsRenderState,
        context: &mut TsRenderContext,
    ) -> TsRenderResult<String> {
        let mut inherited: Option<String> = None;
        for extension in &self.extensions {
            let reference = &extension.reference;
            let name = reference.identifier.render(state, context)?;
            let next = if reference.arguments.is_empty() {
                format!("typeof {name}.fields")
            } else {
                let arguments = reference
                    .arguments
                    .iter()
                    .map(|argument| {
                        TsEffectGeneric::resolve(argument, &self.generics)
                            .codec
                            .render(&self.generic_names(), state, context)
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                format!(
                    "ReturnType<typeof {name}<{}>>[\"fields\"]",
                    arguments.join(", ")
                )
            };
            inherited = Some(match inherited {
                Some(previous) => format!("Omit<{previous}, keyof ({next})> & {next}"),
                None => next,
            });
        }
        Ok(match inherited {
            Some(inherited) => format!("Omit<{inherited}, keyof ({own})> & {own}"),
            None => own,
        })
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
            @"
        export const Name = Schema.Struct({
          ...Hello.fields,
          ...World.fields,
          ...{
            name: Schema.String
          }
        });

        export type Name = Schema.Schema.Type<typeof Name>;
        "
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
    fn test_render_effect_forward_preserves_codec_boundary_and_fields() {
        assert_snapshot!(
            render_node_with(Tst::interface("Node", vec![Tst::property("next", Tst::reference_forward("Later"))]), &mut Tst::render_context_effect()),
            @"
        export interface Node {
          next: Later;
        }

        export const Node: Schema.Codec<Node> & {
          fields: {
            next: Schema.Codec<Later>
          }
        } = Schema.Struct({
          next: Schema.suspend((): Schema.Codec<Later> => Later)
        });
        "
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
            @"
        export const Name = Schema.Struct({
          ...Base.fields,
          ...{
            details: Schema.Struct({
              name: Schema.String
            })
          }
        });

        export type Name = Schema.Schema.Type<typeof Name>;
        "
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
            @"
        export const Name = Schema.Struct({
          ...Base.fields,
          ...{
          }
        });

        export type Name = Schema.Schema.Type<typeof Name>;
        "
        );
    }
    #[test]
    fn test_render_effect_recursive_generic_preserves_fields_and_encoded_types() {
        let reference = TsReference::new(
            "Node".into(),
            vec![Tst::reference("T").into()],
            TsReferenceRel::SelfRecursive,
        );
        assert_snapshot!(render_node_with(
            Tst::interface_with_generics("Node", vec!["T"], vec![
                Tst::property("value", Tst::reference("T")),
                Tst::property_optional("next", Tst::union(vec_into![reference, Tst::primitive_undefined()])),
            ]), &mut Tst::render_context_effect()), @r#"
        export namespace Node {
          export type Type<T> = {
            value: T,
            next?: Node.Type<T> | undefined
          };
        }

        export type Node<T extends Schema.Top> = Node.Type<T["Type"]>;

        export const Node = <T extends Schema.Top>(T: T): Schema.Struct<{
          value: Schema.Codec<T["Type"], T["Encoded"], T["DecodingServices"], T["EncodingServices"]>,
          next: Schema.optionalKey<Schema.Codec<Node.Type<T["Type"]> | undefined, Node.Type<T["Encoded"]> | undefined, T["DecodingServices"], T["EncodingServices"]>>
        }> => Schema.Struct({
          value: Schema.suspend((): Schema.Codec<T["Type"], T["Encoded"], T["DecodingServices"], T["EncodingServices"]> => T),
          next: Schema.optionalKey(Schema.suspend((): Schema.Codec<Node.Type<T["Type"]> | undefined, Node.Type<T["Encoded"]> | undefined, T["DecodingServices"], T["EncodingServices"]> => Schema.Union([Node(T), Schema.Undefined])))
        });
        "#);
    }

    #[test]
    fn test_render_effect_extension_override_alias_preference() {
        let config = TsConfigLang {
            mode: TsMode::Effect,
            prefer: TsPrefer::Alias,
            ..Default::default()
        };
        let mut context = TsRenderContext::new(&config);

        let node = TsInterface {
            extensions: vec![Tst::extension("Base")],
            ..Tst::interface(
                "Name",
                vec![Tst::property("kind", Tst::literal_string("override"))],
            )
        };
        let mut resolver = TsEffectResolver::new(IndexSet::from([Tst::identifier("Name")]));
        node.traverse(&mut resolver);
        context.resolution = Arc::new(TsModuleResolution {
            effect: resolver.resolution,
        });
        assert_snapshot!(node.render(Default::default(), &mut context).unwrap(), @r#"
        export type Name = Omit<Base, "kind"> & {
          kind: "override";
        };

        export const Name = Schema.Struct({
          ...Base.fields,
          ...{
            kind: Schema.Literal("override")
          }
        });
        "#);
    }

    #[test]
    fn test_render_effect_extension_cycle() {
        assert_snapshot!(render_node_with(Tst::module(vec![], vec_into![
            Tst::interface("B", vec![Tst::property("a", Tst::reference_forward("A"))]),
            TsInterface { extensions: vec![Tst::extension("B")], ..Tst::interface("A", vec![]) },
        ]), &mut Tst::render_context_effect()), @r#"
        import { Schema } from "effect";

        export interface B {
          a: A;
        }

        export const B: Schema.Codec<B> & {
          fields: {
            a: Schema.Codec<A>
          }
        } = Schema.Struct({
          a: Schema.suspend((): Schema.Codec<A> => A)
        });

        export interface A extends B {
        }

        export const A = Schema.Struct({
          ...B.fields,
          ...{
          }
        });
        "#);
    }
}
