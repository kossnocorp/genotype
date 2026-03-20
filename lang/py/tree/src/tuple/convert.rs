use crate::prelude::internal::*;

impl PYConvert<PYTuple> for GTTuple {
    fn convert(&self, context: &mut PYConvertContext) -> PYTuple {
        PYTuple {
            descriptors: self
                .descriptors
                .iter()
                .map(|descriptor| descriptor.convert(context))
                .collect(),
        }
        .resolve(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;
    use genotype_test::*;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            convert_node(Gt::tuple(vec![
                Gt::primitive_boolean().into(),
                Gt::primitive_string().into(),
            ])),
            @"
        PYTuple(
          descriptors: [
            Primitive(Boolean),
            Primitive(String),
          ],
        )
        "
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = PYConvertContext::new(
            PYConvertResolve::default(),
            PyConfig {
                lang: PyConfigLang::new(PYVersion::Legacy),
                ..Default::default()
            },
        );
        assert_ron_snapshot!(
            convert_node_with(Gt::tuple(vec![
                Gt::primitive_string().into(),
            ]), &mut context),
            @"
        PYTuple(
          descriptors: [
            Primitive(String),
          ],
        )
        "
        );
        assert_ron_snapshot!(
            context.as_dependencies(),
            @r#"
        [
          (Typing, PYIdentifier("Tuple")),
        ]
        "#
        );
    }
}
