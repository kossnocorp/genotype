use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for TSInlineImport {
    type RenderState = TSRenderState;

    type RenderContext = TSRenderContext;

    fn render(&self, state: Self::RenderState, context: &mut Self::RenderContext) -> Result<String> {
        let path = self.path.render(state, context)?;
        let name = self.name.render(state, context)?;

        Ok(format!(r#"import("{path}").{name}"#))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render() {
        assert_eq!(
            TSInlineImport {
                path: "./path/to/module.ts".into(),
                name: "Name".into(),
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            r#"import("./path/to/module.ts").Name"#
        );
    }
}
