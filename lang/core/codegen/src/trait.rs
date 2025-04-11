use crate::*;
use genotype_parser::*;
use miette::Result;

/// Codegen trait for target languages.
pub trait GtlCodegen<'a, RenderContext> {
    /// Generates code for a descriptor node.
    fn gen_descriptor(
        descriptor: &GTDescriptor,
    ) -> Result<GtlCodegenResultDescriptor<'a, RenderContext>>;

    /// Generates code for an alias node.
    fn gen_alias(alias: &GTAlias) -> Result<GtlCodegenResultAlias<'a, RenderContext>>;
}
