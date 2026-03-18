use crate::prelude::internal::*;

impl PYConvert<PYLiteral> for GTLiteral {
    fn convert(&self, context: &mut PYConvertContext) -> PYLiteral {
        match self {
            GTLiteral::Null(_) => PYLiteral::None,
            GTLiteral::Boolean(_, value) => PYLiteral::Boolean(*value),
            GTLiteral::Integer(_, value) => PYLiteral::Integer(*value),
            GTLiteral::Float(_, value) => PYLiteral::Float(*value),
            GTLiteral::String(_, value) => PYLiteral::String(value.clone()),
        }
        .resolve(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            GTLiteral::Null((0, 0).into()).convert(&mut PYConvertContext::default()),
            @"r#None"
        );
        assert_ron_snapshot!(
            GTLiteral::Boolean((0, 0).into(), true).convert(&mut PYConvertContext::default()),
            @"Boolean(true)"
        );
        assert_ron_snapshot!(
            GTLiteral::Integer((0, 0).into(), -123).convert(&mut PYConvertContext::default()),
            @"Integer(-123)"
        );
        assert_ron_snapshot!(
            GTLiteral::Float((0, 0).into(), 1.23).convert(&mut PYConvertContext::default()),
            @"Float(1.23)"
        );
        assert_ron_snapshot!(
            GTLiteral::String((0, 0).into(), "Hello, world!".into())
                .convert(&mut PYConvertContext::default()),
            @r#"String("Hello, world!")"#
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = Default::default();
        assert_ron_snapshot!(
            GTLiteral::Boolean((0, 0).into(), false).convert(&mut context),
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
