use crate::prelude::internal::*;

pub struct TsRenderTypes;

impl<'context> GtlRenderTypes<'context> for TsRenderTypes {
    type State = TsRenderState;
    type Context = TsRenderContext<'context>;
    type Error = TsRenderError;
}
