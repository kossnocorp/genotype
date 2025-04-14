use crate::*;
use genotype_lang_core_tree::*;
use genotype_parser::*;
use miette::Result;

/// Codegen trait for target languages.
pub trait GtlCodegen<'a, RenderState, RenderContext>
where
    Box<(dyn GtlRenderResolveImport<'a, RenderState, RenderContext>)>: Clone,
{
    /// Generates code for a descriptor node.
    fn gen_descriptor(
        descriptor: &GTDescriptor,
    ) -> Result<GtlCodegenResultDescriptor<'a, RenderState, RenderContext>>;

    /// Generates code for an alias node.
    fn gen_alias(alias: &GTAlias) -> Result<GtlCodegenResultAlias<'a, RenderState, RenderContext>>;
}
