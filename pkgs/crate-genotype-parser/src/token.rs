use crate::prelude::internal::*;

/// Syntax kinds with stable names for diagnostics and serialization.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GtTokenKind {
    Module,
    Name,
    Reference,
    Alias,
    Descriptors,
    Descriptor,
    DescriptorInner,
    Primitive,
    PrimitiveNumeric,
    Any,
    Literal,
    LiteralString,
    Quote,
    StringInner,
    StringEscape,
    StringUnicode,
    LiteralInteger,
    LiteralFloat,
    LiteralNull,
    Integer,
    Digits,
    Exponent,
    LiteralBoolean,
    Branded,
    Object,
    Properties,
    Property,
    ExtensionProperty,
    RequiredProperty,
    OptionalProperty,
    GenericParameters,
    GenericArgument,
    GenericArguments,
    Attribute,
    AttributeDescriptor,
    AttributeAssignment,
    AttributeArguments,
    AttributeProperties,
    AttributeProperty,
    AttributeValue,
    Annotation,
    Array,
    Tuple,
    Record,
    RecordKey,
    Import,
    ImportReference,
    ImportNames,
    ImportName,
    ImportGlob,
    InlineImport,
    Path,
    PathSegment,
    LineComment,
    BlockComment,
    ModuleDoc,
    LineDoc,
    Doc,
    #[serde(rename = "SPACE")]
    Space,
    #[serde(rename = "WHITESPACE")]
    Whitespace,
    #[serde(rename = "COMMENT")]
    Comment,
    #[serde(rename = "EOI")]
    Eoi,
    /// A punctuation or keyword leaf retained for exact source reconstruction.
    #[serde(rename = "Punctuation")]
    Punctuation,
}

impl GtTokenKind {
    /// Stable diagnostic spelling, independent of the Rust variant name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Module => "module",
            Self::Name => "name",
            Self::Reference => "reference",
            Self::Alias => "alias",
            Self::Descriptors => "descriptors",
            Self::Descriptor => "descriptor",
            Self::DescriptorInner => "descriptor_inner",
            Self::Primitive => "primitive",
            Self::PrimitiveNumeric => "primitive_numeric",
            Self::Any => "any",
            Self::Literal => "literal",
            Self::LiteralString => "literal_string",
            Self::Quote => "quote",
            Self::StringInner => "string_inner",
            Self::StringEscape => "string_escape",
            Self::StringUnicode => "string_unicode",
            Self::LiteralInteger => "literal_integer",
            Self::LiteralFloat => "literal_float",
            Self::LiteralNull => "literal_null",
            Self::Integer => "integer",
            Self::Digits => "digits",
            Self::Exponent => "exponent",
            Self::LiteralBoolean => "literal_boolean",
            Self::Branded => "branded",
            Self::Object => "object",
            Self::Properties => "properties",
            Self::Property => "property",
            Self::ExtensionProperty => "extension_property",
            Self::RequiredProperty => "required_property",
            Self::OptionalProperty => "optional_property",
            Self::GenericParameters => "generic_parameters",
            Self::GenericArgument => "generic_argument",
            Self::GenericArguments => "generic_arguments",
            Self::Attribute => "attribute",
            Self::AttributeDescriptor => "attribute_descriptor",
            Self::AttributeAssignment => "attribute_assignment",
            Self::AttributeArguments => "attribute_arguments",
            Self::AttributeProperties => "attribute_properties",
            Self::AttributeProperty => "attribute_property",
            Self::AttributeValue => "attribute_value",
            Self::Annotation => "annotation",
            Self::Array => "array",
            Self::Tuple => "tuple",
            Self::Record => "record",
            Self::RecordKey => "record_key",
            Self::Import => "import",
            Self::ImportReference => "import_reference",
            Self::ImportNames => "import_names",
            Self::ImportName => "import_name",
            Self::ImportGlob => "import_glob",
            Self::InlineImport => "inline_import",
            Self::Path => "path",
            Self::PathSegment => "path_segment",
            Self::LineComment => "line_comment",
            Self::BlockComment => "block_comment",
            Self::ModuleDoc => "module_doc",
            Self::LineDoc => "line_doc",
            Self::Doc => "doc",
            Self::Space => "SPACE",
            Self::Whitespace => "WHITESPACE",
            Self::Comment => "COMMENT",
            Self::Eoi => "EOI",
            Self::Punctuation => "Punctuation",
        }
    }
}

impl std::fmt::Display for GtTokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

// Keep token names consistent in diagnostic debug output.
impl std::fmt::Debug for GtTokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A source-preserving syntax token. Structural tokens own ordered children;
/// leaves retain the exact spelling, including whitespace and comments.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GtToken<'a> {
    pub kind: GtTokenKind,
    pub span: GtSpan,
    pub text: &'a str,
    pub children: Vec<GtToken<'a>>,
}

impl<'a> GtToken<'a> {
    pub(crate) fn new(
        kind: GtTokenKind,
        source: &'a str,
        start: usize,
        end: usize,
        children: Vec<Self>,
    ) -> Self {
        Self {
            kind,
            span: (start, end).into(),
            text: &source[start..end],
            children,
        }
    }

    pub fn as_str(&self) -> &'a str {
        self.text
    }
    /// Semantic children for AST conversion. The public `children` collection
    /// retains punctuation and trivia for formatters.
    pub fn into_semantic_children(self) -> GtTokens<'a> {
        GtTokens::new(
            self.children
                .into_iter()
                .filter(|token| !token.kind.is_trivia())
                .collect(),
        )
    }
}

impl GtTokenKind {
    pub fn is_trivia(self) -> bool {
        matches!(
            self,
            Self::Whitespace
                | Self::Space
                | Self::Comment
                | Self::LineComment
                | Self::BlockComment
                | Self::Punctuation
        )
    }
}

#[derive(Debug, Clone)]
pub struct GtTokens<'a> {
    inner: std::vec::IntoIter<GtToken<'a>>,
}

impl<'a> GtTokens<'a> {
    pub(crate) fn new(tokens: Vec<GtToken<'a>>) -> Self {
        Self {
            inner: tokens.into_iter(),
        }
    }
}

impl<'a> Iterator for GtTokens<'a> {
    type Item = GtToken<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl ExactSizeIterator for GtTokens<'_> {}
