use crate::prelude::internal::*;

impl PyConvert<PyExtension> for GtExtension {
    fn convert(&self, context: &mut PyConvertContext) -> PyExtension {
        PyExtension {
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
                reference: Gt::reference_anon("Name").into()
            }
            .convert(&mut PyConvertContext::default()),
            @r#"
        PyExtension(
          reference: PyReference(
            identifier: PyIdentifier("Name"),
            forward: true,
          ),
        )
        "#,
        );
    }
}
