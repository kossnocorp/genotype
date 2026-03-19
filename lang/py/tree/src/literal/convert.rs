use crate::prelude::internal::*;

impl PYConvert<PYLiteral> for GTLiteral {
    fn convert(&self, context: &mut PYConvertContext) -> PYLiteral {
        match &self.value {
            GTLiteralValue::Null => PYLiteral::None,
            GTLiteralValue::Boolean(value) => PYLiteral::Boolean(*value),
            GTLiteralValue::Integer(value) => PYLiteral::Integer(*value),
            GTLiteralValue::Float(value) => PYLiteral::Float(*value),
            GTLiteralValue::String(value) => PYLiteral::String(value.clone()),
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
            GTLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GTLiteralValue::Null,
            }
            .convert(&mut PYConvertContext::default()),
            @"r#None"
        );
        assert_ron_snapshot!(
            GTLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GTLiteralValue::Boolean(true),
            }
            .convert(&mut PYConvertContext::default()),
            @"Boolean(true)"
        );
        assert_ron_snapshot!(
            GTLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GTLiteralValue::Integer(-123),
            }
            .convert(&mut PYConvertContext::default()),
            @"Integer(-123)"
        );
        assert_ron_snapshot!(
            GTLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GTLiteralValue::Float(1.23),
            }
            .convert(&mut PYConvertContext::default()),
            @"Float(1.23)"
        );
        assert_ron_snapshot!(
            GTLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GTLiteralValue::String("Hello, world!".into()),
            }
            .convert(&mut PYConvertContext::default()),
            @r#"String("Hello, world!")"#
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = Default::default();
        assert_ron_snapshot!(
            GTLiteral {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                value: GTLiteralValue::Boolean(false),
            }
            .convert(&mut context),
            @"Boolean(false)"
        );
        assert_ron_snapshot!(
            context.as_dependencies(),
            @r#"
        [
          (Typing, PYIdentifier("Literal")),
        ]
        "#
        );
    }
}
