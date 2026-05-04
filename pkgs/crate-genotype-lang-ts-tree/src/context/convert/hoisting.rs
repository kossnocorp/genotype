use crate::prelude::internal::*;

impl TsConvertContext {
    pub fn hoist<HoistFn, Definition>(&mut self, mut hoist_fn: HoistFn) -> TsReference
    where
        Definition: Into<TsDefinition>,
        HoistFn: FnMut(&mut TsConvertContext) -> Definition,
    {
        let definition = hoist_fn(self).into();
        let arguments = vec![];

        if let Some(generics) = definition.generics()
            && !generics.is_empty()
        {
            panic!("Hoisting definitions with generics is not supported")
        }

        let reference = TsReference::new(definition.name(), arguments, TsReferenceRel::Regular);
        self.hoisted.push(definition);

        reference
    }

    pub fn drain_hoisted(&mut self) -> Vec<TsDefinition> {
        self.hoisted.drain(..).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hoist() {
        let mut context = TsConvertContext::default();
        let reference =
            context.hoist(|context| Gt::alias("Hello", Gt::primitive_string()).convert(context));
        assert_ron_snapshot!(
            reference,
            @r#"
        TsReference(
          identifier: TsIdentifier("Hello"),
          arguments: [],
          rel: Regular,
        )
        "#
        );
    }

    #[test]
    #[ignore = r#"
        This should not be possible and return error, but convert methods currently never return
        error. We need to test it after the issue is resolved. See: https://github.com/kossnocorp/genotype/issues/113
    "#]
    fn test_hoist_with_arguments() {
        let mut context = TsConvertContext::default();
        let reference = context.hoist(|context| {
            Gt::alias_with_generics(
                "Hello",
                vec![Gt::generic_parameter("Type")],
                Gt::reference("Type", (0, 0)),
            )
            .convert(context)
        });
        assert_ron_snapshot!(
            reference,
            @""
        );
    }
}
