use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for RSInlineUse {
    type RenderState = RSRenderState;

    type RenderContext = RSRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let module = self.path.render(state, context)?;
        let name = self.name.render(state, context)?;
        Ok(format!("{module}::{name}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render() {
        assert_eq!(
            RSInlineUse {
                path: RSPath("path/to/module".into(), "self::path::to::module".into()),
                name: "Name".into(),
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            "self::path::to::module::Name"
        );
    }
}
