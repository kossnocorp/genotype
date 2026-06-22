use crate::prelude::internal::*;

impl<'context> GtlRender<'context, RsRenderTypes> for RsInlineUse {
    fn render(
        &self,
        state: RsRenderState,
        context: &mut RsRenderContext,
    ) -> RsRenderResult<String> {
        let module = self.path.render(state, context)?;
        let name = self.name.render(state, context)?;
        Ok(format!("{module}::{name}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            RsInlineUse {
                path: RsPath("path/to/module".into(), "self::path::to::module".into()),
                name: "Name".into(),
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"self::path::to::module::Name"
        );
    }
}
