use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TSRecordKey {
    type RenderState = TSRenderState;

    type RenderContext = TSRenderContext<'a>;

    fn render(
        &self,
        _state: Self::RenderState,
        _context: &mut Self::RenderContext,
    ) -> Result<String> {
        Ok(match self {
            TSRecordKey::String => "string".into(),
            TSRecordKey::Number => "number".into(),
            TSRecordKey::Boolean => "boolean".into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render() {
        assert_eq!(
            TSRecordKey::String
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            "string"
        );
        assert_eq!(
            TSRecordKey::Number
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            "number"
        );
        assert_eq!(
            TSRecordKey::Boolean
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            "boolean"
        );
    }
}
