use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for RsUseName {
    type RenderState = RsRenderState;

    type RenderContext = RsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        Ok(match self {
            RsUseName::Name(name) => name.render(state, context)?,
            RsUseName::Alias(name, alias) => {
                format!(
                    "{name} as {alias}",
                    name = name.render(state, context)?,
                    alias = alias.render(state, context)?
                )
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_name() {
        assert_snapshot!(
            RsUseName::Name("Name".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Name"
        );
    }

    #[test]
    fn test_render_alias() {
        assert_snapshot!(
            RsUseName::Alias("Name".into(), "Alias".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Name as Alias"
        );
    }
}
