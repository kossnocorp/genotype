use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for RSUseName {
    type RenderState = RSRenderState;

    type RenderContext = RSRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        Ok(match self {
            RSUseName::Name(name) => name.render(state, context)?,
            RSUseName::Alias(name, alias) => {
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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render_name() {
        assert_eq!(
            RSUseName::Name("Name".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            "Name"
        );
    }

    #[test]
    fn test_render_alias() {
        assert_eq!(
            RSUseName::Alias("Name".into(), "Alias".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            "Name as Alias"
        );
    }
}
