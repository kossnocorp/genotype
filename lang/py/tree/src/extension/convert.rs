use crate::prelude::internal::*;

impl PYConvert<PYExtension> for GTExtension {
    fn convert(&self, context: &mut PYConvertContext) -> PYExtension {
        PYExtension {
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
            GTExtension {
                span: (0, 0).into(),
                reference: GtFactory::reference("Name").into()
            }
            .convert(&mut PYConvertContext::default()),
            @r#"
        PYExtension(
          reference: PYReference(
            identifier: PYIdentifier("Name"),
            forward: true,
          ),
        )
        "#,
        );
    }
}
