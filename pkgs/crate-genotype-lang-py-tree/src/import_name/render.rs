use crate::prelude::internal::*;

impl<'context> GtlRender<'context, PyRenderTypes> for PyImportName {
    fn render(
        &self,
        state: PyRenderState,
        context: &mut PyRenderContext,
    ) -> PyRenderResult<String> {
        match self {
            PyImportName::Name(name) => name.render(state, context),

            PyImportName::Alias(name, alias) => {
                let name = name.render(state, context)?;
                let alias = alias.render(state, context)?;
                Ok(format!("{name} as {alias}"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_name() {
        assert_snapshot!(
            PyImportName::Name("Name".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Name"
        );
    }

    #[test]
    fn test_render_alias() {
        assert_snapshot!(
            PyImportName::Alias("Name".into(), "Alias".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Name as Alias"
        );
    }
}
