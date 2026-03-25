use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsInlineImport {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let path = self.path.render(state, context)?;
        let name = self.name.render(state, context)?;

        Ok(format!(r#"import("{path}").{name}"#))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            TsInlineImport {
                path: "./path/to/module".into(),
                name: "Name".into(),
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @r#"import("./path/to/module.js").Name"#
        );
    }
}
