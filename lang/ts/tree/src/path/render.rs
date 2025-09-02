use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TSPath {
    type RenderState = TSRenderState;

    type RenderContext = TSRenderContext<'a>;

    fn render(
        &self,
        _state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        if self.is_external() {
            return Ok(self.0.clone());
        }
        Ok(context.config.format_import_path(&self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn render_basic() {
        assert_eq!(
            TSPath("./path/to/module".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            "./path/to/module.js"
        );
    }

    #[test]
    fn render_ts_ext() {
        let mut ctx = TSRenderContext {
            config: &TsConfigLang {
                tsconfig: TsConfigLangTsconfig {
                    allow_importing_ts_extensions: true,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        assert_eq!(
            TSPath("./path/to/module".into())
                .render(Default::default(), &mut ctx)
                .unwrap(),
            "./path/to/module.ts"
        );
    }

    #[test]
    fn render_external() {
        assert_eq!(
            TSPath("path/to/module".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            "path/to/module"
        );
    }
}
