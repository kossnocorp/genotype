use crate::prelude::internal::*;

impl<'context> GtlRender<'context, PyRenderTypes> for PyImportReference {

    fn render(
        &self,
        state: PyRenderState,
        context: &mut PyRenderContext,
    ) -> PyRenderResult<String> {
        Ok(match self {
            PyImportReference::Default(name) => {
                if let Some(name) = name {
                    name.render(state, context)?
                } else {
                    "".into()
                }
            }

            PyImportReference::Glob => "*".into(),

            PyImportReference::Named(names) => names
                .iter()
                .map(|name| name.render(state, context))
                .collect::<Result<Vec<_>, _>>()?
                .join(", "),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_default() {
        assert_snapshot!(
            PyImportReference::Default(Some("Name".into()))
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Name"
        );
        assert_snapshot!(
            PyImportReference::Default(None)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @""
        );
    }

    #[test]
    fn test_render_glob() {
        assert_snapshot!(
            PyImportReference::Glob
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"*"
        );
    }

    #[test]
    fn test_render_named() {
        assert_snapshot!(
            PyImportReference::Named(vec![
                PyImportName::Name("Name".into()),
                PyImportName::Alias("Name".into(), "Alias".into()),
            ])
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"Name, Name as Alias"
        );
    }
}
