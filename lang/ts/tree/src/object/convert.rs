use crate::prelude::internal::*;

impl TSConvert<TSObject> for GTObject {
    fn convert(&self, context: &mut TSConvertContext) -> TSObject {
        TSObject {
            properties: self
                .properties
                .iter()
                .map(|property| property.convert(context))
                .collect(),
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
            GTObject {
                span: (0, 0).into(),
                name: GTIdentifier::new((0, 0).into(), "Person".into()).into(),
                extensions: vec![],
                properties: vec![
                    GTProperty {
                        span: (0, 0).into(),
                        doc: None,
                        attributes: vec![],
                        name: GTKey::new((0, 0).into(), "name".into()),
                        descriptor: GtFactory::primitive_string().into(),
                        required: true,
                    },
                    GTProperty {
                        span: (0, 0).into(),
                        doc: None,
                        attributes: vec![],
                        name: GTKey::new((0, 0).into(), "age".into()),
                        descriptor: GtFactory::primitive_i32().into(),
                        required: false,
                    }
                ]
            }
            .convert(&mut Default::default()),
            @r#"
        TSObject(
          properties: [
            TSProperty(
              doc: None,
              name: TSKey("name"),
              descriptor: Primitive(String),
              required: true,
            ),
            TSProperty(
              doc: None,
              name: TSKey("age"),
              descriptor: Union(TSUnion(
                descriptors: [
                  Primitive(Number),
                  Primitive(Undefined),
                ],
              )),
              required: false,
            ),
          ],
        )
        "#
        );
    }
}
