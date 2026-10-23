use crate::prelude::internal::*;
use std::sync::LazyLock;

mod error;
pub use error::*;

mod result;
pub use result::*;

mod types;
pub use types::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TsRenderContext<'a> {
    pub config: &'a TsConfigLang,
    pub mode_override: Option<TsMode>,
}

impl GtlRenderContext for TsRenderContext<'_> {}

impl TsRenderContext<'_> {
    pub fn new<'config>(config: &'config TsConfigLang) -> TsRenderContext<'config> {
        TsRenderContext {
            config,
            mode_override: None,
        }
    }

    pub fn mode(&self) -> TsMode {
        self.mode_override.unwrap_or(self.config.mode)
    }

    pub fn with_mode<Cb, CbResult>(
        &mut self,
        mode: TsMode,
        callback: Cb,
    ) -> Result<CbResult, TsRenderError>
    where
        Cb: FnOnce(&mut Self) -> Result<CbResult, TsRenderError>,
    {
        let prev_mode_override = self.mode_override;
        self.mode_override = Some(mode);

        let result = callback(self);

        self.mode_override = prev_mode_override;
        result
    }
}

static TS_DEFAULT_CONFIG: LazyLock<TsConfigLang> = LazyLock::new(TsConfigLang::default);

impl Default for TsRenderContext<'_> {
    fn default() -> Self {
        Self {
            config: &TS_DEFAULT_CONFIG,
            mode_override: None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct TsRenderState {
    indent: usize,
}

impl GtlRenderState for TsRenderState {
    fn indent_inc(&self) -> Self {
        Self {
            indent: self.indent + 1,
        }
    }

    fn indent_level(&self) -> usize {
        self.indent
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_mode() {
        let context = TsRenderContext {
            config: &TsConfigLang {
                mode: TsMode::Zod,
                ..Default::default()
            },
            ..Default::default()
        };
        assert_eq!(context.mode(), TsMode::Zod);

        let context = TsRenderContext {
            config: &TsConfigLang {
                mode: TsMode::Types,
                ..Default::default()
            },
            ..Default::default()
        };
        assert_eq!(context.mode(), TsMode::Types);
    }

    #[test]
    fn test_context_with_mode_override() {
        let mut context = TsRenderContext {
            config: &TsConfigLang {
                mode: TsMode::Zod,
                ..Default::default()
            },
            ..Default::default()
        };

        let outer_ptr = &context as *const _;
        assert_eq!(context.mode(), TsMode::Zod);

        context
            .with_mode(TsMode::Types, |context| {
                let inner_ptr1 = context as *const _;
                assert!(std::ptr::eq(inner_ptr1, outer_ptr));
                assert_eq!(context.mode(), TsMode::Types);

                context
                    .with_mode(TsMode::Zod, |context| {
                        let inner_ptr2 = context as *const _;
                        assert!(std::ptr::eq(inner_ptr2, outer_ptr));
                        assert_eq!(context.mode(), TsMode::Zod);
                        Ok(())
                    })
                    .unwrap();

                assert_eq!(context.mode(), TsMode::Types);
                Ok(())
            })
            .unwrap();

        assert_eq!(context.mode(), TsMode::Zod);
    }

    #[test]
    fn test_effect_mode_override() {
        let mut context = Tst::render_context_effect();
        assert_eq!(context.mode(), TsMode::Effect);
        context
            .with_mode(TsMode::Types, |context| {
                assert_eq!(context.mode(), TsMode::Types);
                context.with_mode(TsMode::Zod, |context| {
                    assert_eq!(context.mode(), TsMode::Zod);
                    Ok(())
                })?;
                assert_eq!(context.mode(), TsMode::Types);
                Ok(())
            })
            .unwrap();
        assert_eq!(context.mode(), TsMode::Effect);
    }
}
