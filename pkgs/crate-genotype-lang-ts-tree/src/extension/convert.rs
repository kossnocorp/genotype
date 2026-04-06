use crate::prelude::internal::*;

impl TsConvert<TsExtension> for GtExtension {
    fn convert(&self, context: &mut TsConvertContext) -> TsExtension {
        TsExtension {
            reference: self.reference.convert(context),
        }
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
            convert_node(GtExtension {
                span: (0, 0).into(),
                reference: Gt::reference("Name"),
            }),
            @r#"
        TsExtension(
          reference: TsReference(
            identifier: TsIdentifier("Name"),
            rel: Regular,
          ),
        )
        "#,
        );
    }
}
