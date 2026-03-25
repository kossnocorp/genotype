use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for PyDictKey {
    type RenderState = PyRenderState;

    type RenderContext = PyRenderContext<'a>;

    fn render(
        &self,
        _state: Self::RenderState,
        _context: &mut Self::RenderContext,
    ) -> Result<String> {
        Ok(match self {
            PyDictKey::String => "str".into(),
            PyDictKey::Int => "int".into(),
            PyDictKey::Float => "float".into(),
            PyDictKey::Boolean => "bool".into(),
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
            PyDictKey::Boolean
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"bool"
        );
        assert_snapshot!(
            PyDictKey::String
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"str"
        );
        assert_snapshot!(
            PyDictKey::Int
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"int"
        );
        assert_snapshot!(
            PyDictKey::Float
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"float"
        );
    }
}
