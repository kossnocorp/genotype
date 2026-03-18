use crate::prelude::internal::*;

impl RSConvert<RSTuple> for GTTuple {
    fn convert(&self, context: &mut RSConvertContext) -> Result<RSTuple> {
        context.drop_definition_id();
        context.enter_parent(RSContextParent::Anonymous);

        let descriptors = self
            .descriptors
            .iter()
            .map(|descriptor| descriptor.convert(context))
            .collect::<Result<Vec<_>>>()?;
        let tuple = RSTuple { descriptors };

        context.exit_parent();
        Ok(tuple)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            GTTuple {
                span: (0, 0).into(),
                descriptors: vec![
                    GTPrimitive::Boolean((0, 0).into()).into(),
                    GTPrimitive::String((0, 0).into()).into(),
                ]
            }
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
            @"
        RSTuple(
          descriptors: [
            Primitive(Boolean),
            Primitive(String),
          ],
        )
        "
        );
    }
}
