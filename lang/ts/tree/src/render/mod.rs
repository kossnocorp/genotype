use crate::prelude::internal::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TSRenderContext {}

impl<'a> GtlRenderContext for TSRenderContext {}

impl Default for TSRenderContext {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct TSRenderState {
    indent: usize,
}

impl<'a> GtlRenderState for TSRenderState {
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

impl Default for TSRenderState {
    fn default() -> Self {
        Self { indent: 0 }
    }
}
