use crate::prelude::internal::*;

/// Parses an AST node from a grammar token and its module context.
pub trait GtParse: Sized {
    fn parse(pair: Pair<'_, Rule>, context: &mut GtContext) -> GtNodeParseResult<Self>;
}

/// Converts a single grammar token into an AST leaf.
pub trait GtParseToken: Sized {
    fn parse(pair: Pair<'_, Rule>) -> Self;
}

/// Parses source code into a module tree and its resolution metadata.
pub trait GtParseModule {
    fn parse(module_id: GtModuleId, source_code: &str) -> Result<GtModuleParse, GtParseError>;
}

/// Parses and normalizes a module path.
pub trait GtParsePath: Sized {
    fn parse(span: GtSpan, module_id: &GtModuleId, path: &str) -> GtNodeParseResult<Self>;
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
