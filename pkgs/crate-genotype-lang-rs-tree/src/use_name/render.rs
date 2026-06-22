use crate::prelude::internal::*;

impl<'context> GtlRender<'context, RsRenderTypes> for RsUseName {

    fn render(
        &self,
        state: RsRenderState,
        context: &mut RsRenderContext,
    ) -> RsRenderResult<String> {
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
