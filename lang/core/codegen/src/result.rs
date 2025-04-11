use crate::*;
use std::fmt::Debug;

/// Codegen result enum.
#[derive(Debug, PartialEq)]
pub enum GtlCodegenResult<'a, RenderContext> {
    /// Descriptor codegen result.
    Descriptor(GtlCodegenResultDescriptor<'a, RenderContext>),
    /// Alias codegen result.
    Alias(GtlCodegenResultAlias<'a, RenderContext>),
}

impl<'a, RenderContext> GtlCodegenResult<'a, RenderContext> {
    pub fn definitions(&self) -> &String {
        match self {
            GtlCodegenResult::Descriptor(result) => &result.definitions,
            GtlCodegenResult::Alias(result) => &result.definitions,
        }
    }

    pub fn resolve(&self) -> &GtlCodegenResolve<'a, RenderContext> {
        match self {
            GtlCodegenResult::Descriptor(result) => &result.resolve,
            GtlCodegenResult::Alias(result) => &result.resolve,
        }
    }
}

/// Descriptor codegen result.
#[derive(Debug, PartialEq)]
pub struct GtlCodegenResultDescriptor<'a, RenderContext> {
    /// Source code representation of a descriptor.
    pub inline: String,
    /// Definitions source code that shall be appended to the root level. It might be empty string.
    pub definitions: String,
    /// Codegen resolve containing the information to embed the generated code.
    pub resolve: GtlCodegenResolve<'a, RenderContext>,
}

/// Alias codegen result.
#[derive(Debug, PartialEq)]
pub struct GtlCodegenResultAlias<'a, RenderContext> {
    /// Definitions source code that shall be appended to the root level.
    pub definitions: String,
    /// Codegen resolve containing the information to embed the generated code.
    pub resolve: GtlCodegenResolve<'a, RenderContext>,
}
