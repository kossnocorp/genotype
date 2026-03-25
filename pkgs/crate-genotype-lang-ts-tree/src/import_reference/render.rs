use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsImportReference {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        Ok(match self {
            TsImportReference::Default(name) => name.to_string(),

            TsImportReference::Glob(name) => format!("* as {}", name),

            TsImportReference::Named(names) => {
                let names = names
                    .iter()
                    .map(|name| name.render(state, context))
                    .collect::<Result<Vec<_>>>()?
                    .join(", ");
                format!("{{ {} }}", names)
            }
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
            TsImportReference::Default("Name".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Name"
        );
    }

    #[test]
    fn test_render_glob() {
        assert_snapshot!(
            TsImportReference::Glob("name".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"* as name"
        );
    }

    #[test]
    fn test_render_named() {
        assert_snapshot!(
            TsImportReference::Named(vec![
                TsImportName::Name("Name".into()),
                TsImportName::Alias("Name".into(), "Alias".into()),
            ])
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"{ Name, Name as Alias }"
        );
    }
}
