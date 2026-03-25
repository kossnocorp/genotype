use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsPath {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        _state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        if self.is_external() {
            return Ok(self.0.to_string());
        }
        Ok(context.config.format_import_path(&self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn render_basic() {
        assert_snapshot!(
            TsPath("./path/to/module".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"./path/to/module.js"
        );
    }

    #[test]
    fn render_ts_ext() {
        let mut ctx = TsRenderContext {
            config: &TsConfigLang {
                tsconfig: TsConfigLangTsconfig {
                    allow_importing_ts_extensions: true,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        assert_snapshot!(
            TsPath("./path/to/module".into())
                .render(Default::default(), &mut ctx)
                .unwrap(),
            @"./path/to/module.ts"
        );
    }

    #[test]
    fn render_external() {
        assert_snapshot!(
            TsPath("path/to/module".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"path/to/module"
        );
    }
}
