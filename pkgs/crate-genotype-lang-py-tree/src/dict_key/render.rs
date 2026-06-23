use crate::prelude::internal::*;

impl<'context> GtlRender<'context, PyRenderTypes> for PyDictKey {
    fn render(
        &self,
        _state: PyRenderState,
        _context: &mut PyRenderContext,
    ) -> PyRenderResult<String> {
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
