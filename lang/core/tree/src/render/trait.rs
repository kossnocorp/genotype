use crate::*;
use miette::Result;

pub trait GtlRender<'a> {
    type RenderState: GtlRenderState;

    type RenderContext: GtlRenderContext;

    fn render(&self, state: Self::RenderState, context: &mut Self::RenderContext)
        -> Result<String>;
}
