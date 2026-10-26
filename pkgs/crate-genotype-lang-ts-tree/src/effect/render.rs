use crate::prelude::internal::*;

impl TsEffectGeneric {
    pub(crate) fn render_types(
        &self,
        name: &str,
        generics: &[String],
        extensions: &[TsExtension],
        doc: &Option<TsDoc>,
        state: TsRenderState,
        context: &mut TsRenderContext,
    ) -> TsRenderResult<String> {
        let descriptor = context.with_mode(TsMode::Types, |context| {
            let mut inherited: Option<String> = None;
            for extension in extensions {
                let mut reference = extension.reference.clone();
                // Structural namespace parameters are value types, while the
                // public aliases of generic bases accept schema types.
                for argument in &mut reference.arguments {
                    *argument = TsReference::new(
                        "Schema.Codec".into(),
                        vec![argument.clone()],
                        TsReferenceRel::Regular,
                    )
                    .into();
                }
                let next = reference.render(state, context)?;
                inherited = Some(match inherited {
                    Some(previous) => format!("Omit<{previous}, keyof ({next})> & {next}"),
                    None => next,
                });
            }
            let own = self.structural.render(state.indent_inc(), context)?;
            Ok(match inherited {
                Some(inherited) => format!("Omit<{inherited}, keyof ({own})> & {own}"),
                None => own,
            })
        })?;
        let params = generics.join(", ");
        let projected = generics
            .iter()
            .map(|generic| format!("{generic}[\"Type\"]"))
            .collect::<Vec<_>>()
            .join(", ");
        let schema_params = generics
            .iter()
            .map(|generic| format!("{generic} extends Schema.Top"))
            .collect::<Vec<_>>()
            .join(", ");
        TsDoc::with_doc(
            doc,
            state,
            context,
            format!(
                "export namespace {name} {{\n{}export type Type<{params}> = {descriptor};\n{}}}\n\nexport type {name}<{schema_params}> = {name}.Type<{projected}>;",
                state.indent_inc().indent_str(),
                state.indent_str()
            ),
            false,
        )
    }
}

impl TsEffectCodec {
    pub(crate) fn render(
        &self,
        generics: &[String],
        state: TsRenderState,
        context: &mut TsRenderContext,
    ) -> TsRenderResult<String> {
        let mut sides = context.with_mode(TsMode::Types, |context| {
            Ok(vec![
                self.decoded.render(state, context)?,
                self.encoded.render(state, context)?,
            ])
        })?;
        for projection in ["DecodingServices", "EncodingServices"] {
            if generics.is_empty() {
                sides.push("never".into());
                continue;
            }
            sides.push(
                generics
                    .iter()
                    .map(|generic| format!("{generic}[\"{projection}\"]"))
                    .collect::<Vec<_>>()
                    .join(" | "),
            );
        }
        Ok(format!("Schema.Codec<{}>", sides.join(", ")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_prepared_codec_and_service_requirements() {
        let codec = TsEffectCodec {
            decoded: Tst::primitive_number().into(),
            encoded: Tst::primitive_string().into(),
        };
        assert_eq!(
            codec
                .render(
                    &["T".into(), "U".into()],
                    Default::default(),
                    &mut Tst::render_context_effect()
                )
                .unwrap(),
            "Schema.Codec<number, string, T[\"DecodingServices\"] | U[\"DecodingServices\"], T[\"EncodingServices\"] | U[\"EncodingServices\"]>"
        );
    }

    #[test]
    fn test_render_prepared_recursive_types() {
        let generic = TsEffectGeneric {
            structural: Tst::array(Tst::reference("T")).into(),
            codec: TsEffectCodec {
                decoded: Tst::any().into(),
                encoded: Tst::any().into(),
            },
            deferred: Tst::any().into(),
        };
        assert_snapshot!(generic.render_types("Node", &["T".into()], &[], &Tst::some_doc("Recursive values"), Default::default(), &mut Tst::render_context_effect()).unwrap(), @r#"
        /** Recursive values */
        export namespace Node {
          export type Type<T> = Array<T>;
        }

        export type Node<T extends Schema.Top> = Node.Type<T["Type"]>;
        "#);
    }
}
