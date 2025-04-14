use genotype_lang_core_tree::*;
use std::fmt::Debug;

pub trait GtlCodegenImport<'a>: GtlImport + GtlRender<'a> {}

impl<'a, RenderState, RenderContext> Debug
    for dyn GtlCodegenImport<'a, RenderState = RenderState, RenderContext = RenderContext>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("GtlCodegenImport")
            // [TODO]
            // .field("imports", &"<omitted>")
            .finish()
    }
}

impl<'a, RenderState, RenderContext> PartialEq
    for dyn GtlCodegenImport<'a, RenderState = RenderState, RenderContext = RenderContext>
{
    fn eq(&self, other: &Self) -> bool {
        // [TODO]
        true
    }
}
