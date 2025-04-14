use genotype_lang_core_tree::*;
use std::fmt::Debug;

/// Codegen result enum.
#[derive(Debug, PartialEq)]
pub enum GtlCodegenResult<'a, RenderState, RenderContext>
where
    Box<(dyn GtlRenderResolveImport<'a, RenderState, RenderContext>)>: Clone,
{
    /// Descriptor codegen result.
    Descriptor(GtlCodegenResultDescriptor<'a, RenderState, RenderContext>),
    /// Alias codegen result.
    Alias(GtlCodegenResultAlias<'a, RenderState, RenderContext>),
}

impl<'a, RenderState, RenderContext> GtlCodegenResult<'a, RenderState, RenderContext>
where
    Box<(dyn GtlRenderResolveImport<'a, RenderState, RenderContext>)>: Clone,
{
    pub fn definitions(&self) -> &String {
        match self {
            GtlCodegenResult::Descriptor(result) => &result.definitions,
            GtlCodegenResult::Alias(result) => &result.definitions,
        }
    }

    pub fn resolve(&self) -> &GtlRenderResolve<'a, RenderState, RenderContext> {
        match self {
            GtlCodegenResult::Descriptor(result) => &result.resolve,
            GtlCodegenResult::Alias(result) => &result.resolve,
        }
    }
}

/// Descriptor codegen result.
#[derive(Debug, PartialEq)]
pub struct GtlCodegenResultDescriptor<'a, RenderState, RenderContext>
where
    Box<(dyn GtlRenderResolveImport<'a, RenderState, RenderContext>)>: Clone,
{
    /// Source code representation of a descriptor.
    pub inline: String,
    /// Definitions source code that shall be appended to the root level. It might be empty string.
    pub definitions: String,
    /// Codegen resolve containing the information to embed the generated code.
    pub resolve: GtlRenderResolve<'a, RenderState, RenderContext>,
}

/// Alias codegen result.
#[derive(Debug, PartialEq)]
pub struct GtlCodegenResultAlias<'a, RenderState, RenderContext>
where
    Box<(dyn GtlRenderResolveImport<'a, RenderState, RenderContext>)>: Clone,
{
    /// Definitions source code that shall be appended to the root level.
    pub definitions: String,
    /// Codegen resolve containing the information to embed the generated code.
    pub resolve: GtlRenderResolve<'a, RenderState, RenderContext>,
}
