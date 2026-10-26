use crate::prelude::internal::*;

impl<'context> GtlRender<'context, TsRenderTypes> for TsAlias {
    fn render(
        &self,
        state: TsRenderState,
        context: &mut TsRenderContext,
    ) -> TsRenderResult<String> {
        let name = self.name.render(state, context)?;
        let generic_names = self
            .generics
            .iter()
            .map(|generic| generic.render(state, context))
            .collect::<Result<Vec<_>, _>>()?;
        let generic_params = if generic_names.is_empty() {
            String::new()
        } else {
            format!("<{}>", generic_names.join(", "))
        };

        match context.mode() {
            TsMode::Effect => self.render_effect(&name, &generic_names, state, context),
            TsMode::Zod => self.render_zod(&name, &generic_names, state, context),
            TsMode::Types => {
                let descriptor = self.descriptor.render(state, context)?;

                TsDoc::with_doc(
                    &self.doc,
                    state,
                    context,
                    format!("export type {name}{generic_params} = {descriptor};"),
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
        generic_names: &[String],
        state: TsRenderState,
        context: &mut TsRenderContext<'a>,
    ) -> TsRenderResult<String> {
        if !generic_names.is_empty() {
            return self.render_zod_generic(name, generic_names, state, context);
        }

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
    ) -> TsRenderResult<String> {
        let zod_descriptor = self.descriptor.render(state, context)?;

        let schema = TsDoc::with_doc(
            &self.doc,
            state,
            context,
            format!("export const {name}: z.ZodType<{name}> = z.lazy(() => {zod_descriptor});"),
            false,
        )?;

        let type_code = context.with_mode(TsMode::Types, |context| {
            let descriptor = self.descriptor.render(state, context)?;
            let type_code = TsDoc::with_doc(
                &self.doc,
                state,
                context,
                format!("export type {name} = {descriptor};"),
                false,
            )?;
            Ok(type_code)
        })?;

        Ok(format!("{type_code}\n\n{schema}"))
    }

    fn render_zod_generic<'a>(
        &self,
        name: &String,
        generic_names: &[String],
        state: TsRenderState,
        context: &mut TsRenderContext<'a>,
    ) -> TsRenderResult<String> {
        let zod_generic_params = render_zod_generic_params(generic_names);
        let params = render_zod_value_params(generic_names);
        let zod_descriptor = self.descriptor.render(state, context)?;
        let schema = TsDoc::with_doc(
            &self.doc,
            state,
            context,
            format!("export const {name} = {zod_generic_params}({params}) => {zod_descriptor};"),
            false,
        )?;

        let generic_params = render_zod_generic_params(generic_names);
        let return_type_args = generic_names.join(", ");
        let r#type = TsDoc::with_doc(
            &self.doc,
            state,
            context,
            format!(
                "export type {name}{generic_params} = z.infer<ReturnType<typeof {name}<{return_type_args}>>>;"
            ),
            false,
        )?;

        Ok(format!("{schema}\n\n{type}"))
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

impl TsAlias {
    fn render_effect<'a>(
        &self,
        name: &String,
        generic_names: &[String],
        state: TsRenderState,
        context: &mut TsRenderContext<'a>,
    ) -> TsRenderResult<String> {
        if !generic_names.is_empty() {
            return self.render_effect_generic(name, generic_names, state, context);
        }

        match context.resolution.effect.aliases.contains_key(&self.name) {
            true => self.render_effect_recursive(name, state, context),

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
                    format!("export type {name} = Schema.Schema.Type<typeof {name}>;"),
                    false,
                )?;

                Ok(format!("{schema}\n\n{type}"))
            }
        }
    }

    fn render_effect_recursive<'a>(
        &self,
        name: &String,
        state: TsRenderState,
        context: &mut TsRenderContext<'a>,
    ) -> TsRenderResult<String> {
        let effect_descriptor = self.descriptor.render(state, context)?;

        let schema = TsDoc::with_doc(
            &self.doc,
            state,
            context,
            format!(
                "export const {name}: Schema.Codec<{name}> = Schema.suspend(() => {effect_descriptor});"
            ),
            false,
        )?;

        let type_code = context.with_mode(TsMode::Types, |context| {
            let descriptor = self.descriptor.render(state, context)?;
            let type_code = TsDoc::with_doc(
                &self.doc,
                state,
                context,
                format!("export type {name} = {descriptor};"),
                false,
            )?;
            Ok(type_code)
        })?;

        Ok(format!("{type_code}\n\n{schema}"))
    }

    fn render_effect_generic<'a>(
        &self,
        name: &String,
        generic_names: &[String],
        state: TsRenderState,
        context: &mut TsRenderContext<'a>,
    ) -> TsRenderResult<String> {
        let resolution = Arc::clone(&context.resolution);
        if let Some(generic) = resolution
            .effect
            .aliases
            .get(&self.name)
            .and_then(|effect| effect.generic.as_ref())
        {
            let types =
                generic.render_types(name, generic_names, &[], &self.doc, state, context)?;
            let codec = generic.codec.render(generic_names, state, context)?;
            let descriptor = generic.deferred.render(state.within_suspend(), context)?;
            let generic_params = Self::render_effect_generic_params(generic_names);
            let params = Self::render_effect_value_params(generic_names);
            let schema = TsDoc::with_doc(
                &self.doc,
                state,
                context,
                format!(
                    "export const {name} = {generic_params}({params}): {codec} => Schema.suspend(() => {descriptor});"
                ),
                false,
            )?;
            return Ok(format!("{types}\n\n{schema}"));
        }
        let effect_generic_params = Self::render_effect_generic_params(generic_names);
        let params = Self::render_effect_value_params(generic_names);
        let effect_descriptor = self.descriptor.render(state, context)?;
        let schema = TsDoc::with_doc(
            &self.doc,
            state,
            context,
            format!(
                "export const {name} = {effect_generic_params}({params}) => {effect_descriptor};"
            ),
            false,
        )?;

        let generic_params = Self::render_effect_generic_params(generic_names);
        let return_type_args = generic_names.join(", ");
        let r#type = TsDoc::with_doc(
            &self.doc,
            state,
            context,
            format!(
                "export type {name}{generic_params} = Schema.Schema.Type<ReturnType<typeof {name}<{return_type_args}>>>;"
            ),
            false,
        )?;

        Ok(format!("{schema}\n\n{type}"))
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
    fn test_render_with_generics() {
        assert_snapshot!(
            render_node(Tst::alias_with_generics("Response", vec!["Payload"], Tst::primitive_string())),
            @"export type Response<Payload> = string;"
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
    fn test_render_zod() {
        assert_snapshot!(
            render_node_with(Tst::alias("Name", Tst::primitive_string()), &mut Tst::render_context_zod()),
            @"
        export const Name = z.string();

        export type Name = z.infer<typeof Name>;
        "
        );
    }

    #[test]
    fn test_render_zod_with_generics() {
        assert_snapshot!(
            render_node_with(
                Tst::alias_with_generics("Response", vec!["Payload"], Tst::reference("Payload")),
                &mut Tst::render_context_zod(),
            ),
            @"
        export const Response = <Payload extends z.ZodTypeAny>(Payload: Payload) => Payload;

        export type Response<Payload extends z.ZodTypeAny> = z.infer<ReturnType<typeof Response<Payload>>>;
        "
        );
    }

    #[test]
    fn test_render_zod_doc() {
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
    fn test_render_zod_self_recursive_array() {
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
    fn test_render_zod_self_recursive_tuple() {
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

    #[test]
    fn test_render_effect() {
        assert_snapshot!(
            render_node_with(Tst::alias("Name", Tst::primitive_string()), &mut Tst::render_context_effect()),
            @"
        export const Name = Schema.String;

        export type Name = Schema.Schema.Type<typeof Name>;
        "
        );
    }

    #[test]
    fn test_render_effect_with_generics() {
        assert_snapshot!(
            render_node_with(
                Tst::alias_with_generics("Response", vec!["Payload"], Tst::reference("Payload")),
                &mut Tst::render_context_effect(),
            ),
            @"
        export const Response = <Payload extends Schema.Top>(Payload: Payload) => Payload;

        export type Response<Payload extends Schema.Top> = Schema.Schema.Type<ReturnType<typeof Response<Payload>>>;
        "
        );
    }

    #[test]
    fn test_render_effect_doc() {
        let mut context = Tst::render_context_effect();

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
        export const Name = Schema.String;

        /** Hello, world! */
        export type Name = Schema.Schema.Type<typeof Name>;
        "
        );
    }

    #[test]
    fn test_render_effect_self_recursive_array() {
        assert_eq!(
            render_node_with(
                Tst::alias("Tree", Tst::array(Tst::reference_self_recursive("Tree"))),
                &mut Tst::render_context_effect()
            ),
            "export type Tree = Array<Tree>;\n\nexport const Tree: Schema.Codec<Tree> = Schema.suspend(() => Schema.mutable(Schema.Array(Tree)));"
        );
    }

    #[test]
    fn test_render_effect_forward() {
        assert_snapshot!(
            render_node_with(Tst::alias("Earlier", Tst::reference_forward("Later")), &mut Tst::render_context_effect()),
            @r#"
        export type Earlier = Later;

        export const Earlier: Schema.Codec<Earlier> = Schema.suspend(() => Later);
        "#
        );
    }

    #[test]
    fn test_render_effect_recursive_tuple() {
        assert_snapshot!(
            render_node_with(Tst::alias("Tree", Tst::union(vec_into![Tst::literal_null(), Tst::tuple(vec_into![Tst::primitive_string(), Tst::reference_self_recursive("Tree")])])), &mut Tst::render_context_effect()),
            @r#"
        export type Tree = null | [string, Tree];

        export const Tree: Schema.Codec<Tree> = Schema.suspend(() => Schema.Union([Schema.Null, Schema.mutable(Schema.Tuple([Schema.String, Tree]))]));
        "#
        );
    }

    #[test]
    fn test_render_effect_recursive_generic_tuple() {
        let reference = TsReference::new(
            "Tree".into(),
            vec![Tst::reference("T").into()],
            TsReferenceRel::SelfRecursive,
        );
        assert_snapshot!(render_node_with(
            Tst::alias_with_generics("Tree", vec!["T"], Tst::union(vec_into![
                Tst::literal_null(), Tst::tuple(vec_into![Tst::reference("T"), reference])
            ])), &mut Tst::render_context_effect()), @r#"
        export namespace Tree {
          export type Type<T> = null | [T, Tree.Type<T>];
        }

        export type Tree<T extends Schema.Top> = Tree.Type<T["Type"]>;

        export const Tree = <T extends Schema.Top>(T: T): Schema.Codec<null | [T["Type"], Tree.Type<T["Type"]>], null | [T["Encoded"], Tree.Type<T["Encoded"]>], T["DecodingServices"], T["EncodingServices"]> => Schema.suspend(() => Schema.Union([Schema.Null, Schema.mutable(Schema.Tuple([T, Tree(T)]))]));
        "#);
    }

    #[test]
    fn test_render_effect_recursive_generic_multiple_parameters() {
        let reference = TsReference::new(
            "Tree".into(),
            vec_into![Tst::reference("K"), Tst::reference("V")],
            TsReferenceRel::SelfRecursive,
        );
        assert_snapshot!(render_node_with(
            Tst::alias_with_generics("Tree", vec!["K", "V"], Tst::union(vec_into![
                Tst::literal_null(), Tst::tuple(vec_into![Tst::reference("K"), Tst::reference("V"), reference])
            ])), &mut Tst::render_context_effect()), @r#"
        export namespace Tree {
          export type Type<K, V> = null | [K, V, Tree.Type<K, V>];
        }

        export type Tree<K extends Schema.Top, V extends Schema.Top> = Tree.Type<K["Type"], V["Type"]>;

        export const Tree = <K extends Schema.Top, V extends Schema.Top>(K: K, V: V): Schema.Codec<null | [K["Type"], V["Type"], Tree.Type<K["Type"], V["Type"]>], null | [K["Encoded"], V["Encoded"], Tree.Type<K["Encoded"], V["Encoded"]>], K["DecodingServices"] | V["DecodingServices"], K["EncodingServices"] | V["EncodingServices"]> => Schema.suspend(() => Schema.Union([Schema.Null, Schema.mutable(Schema.Tuple([K, V, Tree(K, V)]))]));
        "#);
    }
}
