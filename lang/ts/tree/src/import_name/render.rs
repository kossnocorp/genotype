use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TSImportName {
    type RenderState = TSRenderState;

    type RenderContext = TSRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        match self {
            TSImportName::Name(name) => name.render(state, context),

            TSImportName::Alias(name, alias) => {
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
            TSImportName::Name("Name".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Name"
        );
    }

    #[test]
    fn test_render_alias() {
        assert_snapshot!(
            TSImportName::Alias("Name".into(), "Alias".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Name as Alias"
        );
    }
}
