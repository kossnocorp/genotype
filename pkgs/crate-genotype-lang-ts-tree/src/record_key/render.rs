use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsRecordKey {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        _state: Self::RenderState,
        _context: &mut Self::RenderContext,
    ) -> Result<String> {
        Ok(match self {
            TsRecordKey::String => "string".into(),
            TsRecordKey::Number => "number".into(),
            TsRecordKey::Boolean => "boolean".into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            TsRecordKey::String
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"string"
        );
        assert_snapshot!(
            TsRecordKey::Number
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"number"
        );
        assert_snapshot!(
            TsRecordKey::Boolean
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"boolean"
        );
    }
}
