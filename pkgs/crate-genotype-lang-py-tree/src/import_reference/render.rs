use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for PyImportReference {
    type RenderState = PyRenderState;

    type RenderContext = PyRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
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
                .collect::<Result<Vec<_>>>()?
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
