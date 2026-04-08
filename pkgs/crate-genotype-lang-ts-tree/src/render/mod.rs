use crate::prelude::internal::*;
use std::sync::LazyLock;

#[derive(Debug, Clone, PartialEq)]
pub struct TsRenderContext<'a> {
    pub config: &'a TsConfigLang,
    pub mode_override: Option<TsMode>,
}

impl<'a> GtlRenderContext for TsRenderContext<'_> {}

impl TsRenderContext<'_> {
    pub fn is_zod_mode(&self) -> bool {
        self.mode() == TsMode::Zod
    }

    pub fn mode(&self) -> TsMode {
        self.mode_override
            .clone()
            .unwrap_or(self.config.mode.clone())
    }

    pub fn with_mode<Cb, CbResult>(&mut self, mode: TsMode, callback: Cb) -> Result<CbResult>
    where
        Cb: FnOnce(&mut Self) -> Result<CbResult>,
    {
        let prev_mode_override = self.mode_override.clone();
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

impl<'a> GtlRenderState for TsRenderState {
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
    fn test_context_is_zod_mode() {
        let context = TsRenderContext {
            config: &TsConfigLang {
                mode: TsMode::Zod,
                ..Default::default()
            },
            ..Default::default()
        };
        assert!(context.is_zod_mode());

        let context = TsRenderContext {
            config: &TsConfigLang {
                mode: TsMode::Types,
                ..Default::default()
            },
            ..Default::default()
        };
        assert!(!context.is_zod_mode());
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
        assert!(context.is_zod_mode());

        context
            .with_mode(TsMode::Types, |context| {
                let inner_ptr1 = context as *const _;
                assert!(std::ptr::eq(inner_ptr1, outer_ptr));
                assert!(!context.is_zod_mode());

                context
                    .with_mode(TsMode::Zod, |context| {
                        let inner_ptr2 = context as *const _;
                        assert!(std::ptr::eq(inner_ptr2, outer_ptr));
                        assert!(context.is_zod_mode());
                        Ok(())
                    })
                    .unwrap();

                assert!(!context.is_zod_mode());
                Ok(())
            })
            .unwrap();

        assert!(context.is_zod_mode());
    }
}
