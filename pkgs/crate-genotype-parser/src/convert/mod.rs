use crate::prelude::internal::*;

/// Converts a syntax token into an AST node using its module context.
pub trait GtParse: Sized {
    fn parse(token: GtToken<'_>, context: &mut GtContext) -> GtNodeParseResult<Self>;
}

/// Converts a syntax token into an AST leaf.
pub trait GtParseToken: Sized {
    fn parse(token: GtToken<'_>) -> Self;
}

/// Parses source code into a module tree and its resolution metadata.
pub trait GtParseModule {
    fn parse(module_id: GtModuleId, source_code: &str) -> Result<GtModuleParse, GtParseError>;
}

/// Parses and normalizes a module path.
pub trait GtParsePath: Sized {
    fn parse(span: GtSpan, module_id: &GtModuleId, path: &str) -> GtNodeParseResult<Self>;
}

/// Convert a syntax node without re-parsing source text.
pub fn convert_node<T: GtParse>(
    token: GtToken<'_>,
    context: &mut GtContext,
) -> GtNodeParseResult<T> {
    T::parse(token, context)
}

mod path_normalize;

mod alias;
mod any;
mod array;
mod attribute;
mod attribute_assignment;
mod attribute_descriptor;
mod attribute_key;
mod attribute_name;
mod attribute_property;
mod attribute_value;
mod branded;
mod descriptor;
mod doc;
mod extension;
mod generic_argument;
mod generic_parameter;
mod identifier;
mod import;
mod inline_import;
mod key;
mod literal;
mod literal_value;
mod module;
pub use module::convert_module;
mod object;
mod path;
mod primitive;
mod primitive_kind;
mod property;
mod record;
mod record_key;
mod reference;
mod tuple;

pub(crate) use doc::concat_doc;
pub(crate) use path::*;
pub(crate) use reference::parse_name_with_arguments;
