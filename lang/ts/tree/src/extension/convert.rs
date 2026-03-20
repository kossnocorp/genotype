use crate::prelude::internal::*;

impl TSConvert<TSExtension> for GTExtension {
    fn convert(&self, context: &mut TSConvertContext) -> TSExtension {
        TSExtension {
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
                reference: GtFactory::reference("Name"),
            }
            .convert(&mut Default::default()),
            @r#"
        TSExtension(
          reference: TSReference(TSIdentifier("Name")),
        )
        "#,
        );
    }
}
