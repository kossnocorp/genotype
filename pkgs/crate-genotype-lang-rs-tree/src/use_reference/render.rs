use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for RsUseReference {
    type RenderState = RsRenderState;

    type RenderContext = RsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        Ok(match self {
            RsUseReference::Module => "".into(),

            RsUseReference::Glob => "*".into(),

            RsUseReference::Named(names) => {
                let names_str = names
                    .iter()
                    .map(|name| name.render(state, context))
                    .collect::<Result<Vec<String>>>()?
                    .join(", ");
                if names.len() == 1 {
                    if let Some(RsUseName::Name(_)) = names.first() {
                        return Ok(names_str);
                    }
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
