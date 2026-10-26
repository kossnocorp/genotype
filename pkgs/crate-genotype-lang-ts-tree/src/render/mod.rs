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
    pub resolution: Arc<TsModuleResolution>,
}

impl GtlRenderContext for TsRenderContext<'_> {}

impl TsRenderContext<'_> {
    pub fn new<'config>(config: &'config TsConfigLang) -> TsRenderContext<'config> {
        TsRenderContext {
            config,
            mode_override: None,
            resolution: Arc::new(TsModuleResolution::default()),
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
            resolution: Arc::new(TsModuleResolution::default()),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct TsRenderState {
    indent: usize,
    deferred: bool,
}

impl TsRenderState {
    pub(crate) fn within_suspend(self) -> Self {
        Self {
            deferred: true,
            ..self
        }
    }

    pub(crate) fn is_deferred(self) -> bool {
        self.deferred
    }
}

impl GtlRenderState for TsRenderState {
    fn indent_inc(&self) -> Self {
        Self {
            indent: self.indent + 1,
            ..*self
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
    #[test]
    fn test_deferred_scope_survives_indentation_without_changing_parent() {
        let state = TsRenderState::default();
        let nested = state.within_suspend().indent_inc();
        assert!(!state.is_deferred());
        assert!(nested.is_deferred());
        assert_eq!(nested.indent_level(), 1);
    }

    #[test]
    fn test_mode_override_preserves_shared_resolution() {
        let mut context = Tst::render_context_effect();
        let resolution = Arc::clone(&context.resolution);
        context
            .with_mode(TsMode::Types, |context| {
                assert!(Arc::ptr_eq(&context.resolution, &resolution));
                Ok(())
            })
            .unwrap();
        assert!(Arc::ptr_eq(&context.resolution, &resolution));
    }
}
