use crate::prelude::internal::*;

impl<'context> GtlRender<'context, RsRenderTypes> for RsUseReference {
    fn render(
        &self,
        state: RsRenderState,
        context: &mut RsRenderContext,
    ) -> RsRenderResult<String> {
        Ok(match self {
            RsUseReference::Module => "".into(),

            RsUseReference::Glob => "*".into(),

            RsUseReference::Named(names) => {
                let names_str = names
                    .iter()
                    .map(|name| name.render(state, context))
                    .collect::<Result<Vec<_>, _>>()?
                    .join(", ");
                if names.len() == 1
                    && let Some(RsUseName::Name(_)) = names.first()
                {
                    return Ok(names_str);
                }

                format!("{{{}}}", names_str)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_module() {
        assert_snapshot!(
            RsUseReference::Module
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @""
        );
    }

    #[test]
    fn test_render_glob() {
        assert_snapshot!(
            RsUseReference::Glob
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"*"
        );
    }

    #[test]
    fn test_render_named() {
        assert_snapshot!(
            RsUseReference::Named(vec![
                RsUseName::Name("Name".into()),
                RsUseName::Alias("Name".into(), "Alias".into()),
            ])
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"{Name, Name as Alias}"
        );
    }

    #[test]
    fn test_render_named_solo() {
        assert_snapshot!(
            RsUseReference::Named(vec![RsUseName::Name("Name".into()),])
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Name"
        );
    }

    #[test]
    fn test_render_named_solo_alias() {
        assert_snapshot!(
            RsUseReference::Named(vec![RsUseName::Alias("Name".into(), "Alias".into()),])
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"{Name as Alias}"
        );
    }
}
