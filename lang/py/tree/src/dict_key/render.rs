use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for PYDictKey {
    type RenderState = PYRenderState;

    type RenderContext = PYRenderContext<'a>;

    fn render(
        &self,
        _state: Self::RenderState,
        _context: &mut Self::RenderContext,
    ) -> Result<String> {
        Ok(match self {
            PYDictKey::String => "str".into(),
            PYDictKey::Int => "int".into(),
            PYDictKey::Float => "float".into(),
            PYDictKey::Boolean => "bool".into(),
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
            PYDictKey::Boolean
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"bool"
        );
        assert_snapshot!(
            PYDictKey::String
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"str"
        );
        assert_snapshot!(
            PYDictKey::Int
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"int"
        );
        assert_snapshot!(
            PYDictKey::Float
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"float"
        );
    }
}
