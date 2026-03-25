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
    use genotype_test::*;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            GtExtension {
                span: (0, 0).into(),
                reference: Gt::reference("Name"),
            }
            .convert(&mut Default::default()),
            @r#"
        TsExtension(
          reference: TsReference(TsIdentifier("Name")),
        )
        "#,
        );
    }
}
