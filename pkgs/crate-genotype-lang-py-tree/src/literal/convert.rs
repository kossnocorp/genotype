use crate::prelude::internal::*;

impl PyConvert<PyLiteral> for GtLiteral {
    fn convert(&self, context: &mut PyConvertContext) -> PyLiteral {
        match &self.value {
            GtLiteralValue::Null => PyLiteral::None,
            GtLiteralValue::Boolean(value) => PyLiteral::Boolean(*value),
            GtLiteralValue::Integer(value) => PyLiteral::Integer(*value),
            GtLiteralValue::Float(value) => PyLiteral::Float(*value),
            GtLiteralValue::String(value) => PyLiteral::String(value.clone()),
        }
        .resolve(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            GtLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GtLiteralValue::Null,
            }
            .convert(&mut PyConvertContext::default()),
            @"r#None"
        );
        assert_ron_snapshot!(
            GtLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GtLiteralValue::Boolean(true),
            }
            .convert(&mut PyConvertContext::default()),
            @"Boolean(true)"
        );
        assert_ron_snapshot!(
            GtLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GtLiteralValue::Integer(-123),
            }
            .convert(&mut PyConvertContext::default()),
            @"Integer(-123)"
        );
        assert_ron_snapshot!(
            GtLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GtLiteralValue::Float(1.23),
            }
            .convert(&mut PyConvertContext::default()),
            @"Float(1.23)"
        );
        assert_ron_snapshot!(
            GtLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GtLiteralValue::String("Hello, world!".into()),
            }
            .convert(&mut PyConvertContext::default()),
            @r#"String("Hello, world!")"#
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = Default::default();
        assert_ron_snapshot!(
            GtLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GtLiteralValue::Boolean(false),
            }
            .convert(&mut context),
            @"Boolean(false)"
        );
        assert_ron_snapshot!(
            context.imports(),
            @r#"
        [
          PyImport(
            dependency: Typing,
            reference: Named([
              Name(PyIdentifier("Literal")),
            ]),
          ),
        ]
        "#
        );
    }
}
