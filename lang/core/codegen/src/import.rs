use genotype_lang_core_tree::*;
use std::fmt::Debug;

pub trait GtlCodegenImport<'a>: GtlImport + GtlRender<'a> {}

impl<'a, RenderContext> Debug for dyn GtlCodegenImport<'a, RenderContext = RenderContext> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("GtlImport")
            // [TODO]
            // .field("imports", &"<omitted>")
            .finish()
    }
}

impl<'a, RenderContext> PartialEq for dyn GtlCodegenImport<'a, RenderContext = RenderContext> {
    fn eq(&self, other: &Self) -> bool {
        // [TODO]
        true
    }
}
