use crate::prelude::internal::*;

pub trait GtlRenderTypes<'context> {
    type State: GtlRenderState;
    type Context: GtlRenderContext;
    type Error: GtlError;
}
