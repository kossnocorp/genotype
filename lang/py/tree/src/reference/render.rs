use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for PYReference {
    type RenderState = PYRenderState;

    type RenderContext = PYRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let str = self.identifier.render(state, context)?;
        if let PYVersion::Legacy = context.config.version {
            if self.forward {
                return Ok(format!("\"{str}\""));
            }
        }
        Ok(str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            PYReference::new("Foo".into(), false)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Foo"
        );
    }

    #[test]
    fn test_render_forward() {
        assert_snapshot!(
            PYReference::new("Foo".into(), true)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Foo"
        );
        assert_snapshot!(
            PYReference::new("Foo".into(), true)
                .render(
                    Default::default(),
                    &mut PYRenderContext {
                        config: &PyConfigLang::new(PYVersion::Legacy),
                        ..Default::default()
                    }
                )
                .unwrap(),
            @r#""Foo""#
        );
    }
}
