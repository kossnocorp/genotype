use genotype_lang_core_tree::*;
use std::fmt::Debug;

pub trait GtlCodegenExport: GtlExport {}

impl Debug for dyn GtlCodegenExport {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("GtlCodegenExport")
            // [TODO]
            // .field("imports", &"<omitted>")
            .finish()
    }
}

impl PartialEq for dyn GtlCodegenExport {
    fn eq(&self, other: &Self) -> bool {
        // [TODO]
        true
    }
}
