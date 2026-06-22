use crate::prelude::internal::*;

pub struct RsRenderTypes;

impl<'context> GtlRenderTypes<'context> for RsRenderTypes {
    type State = RsRenderState;
    type Context = RsRenderContext<'context>;
    type Error = RsRenderError;
}
