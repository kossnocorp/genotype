use crate::prelude::internal::*;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod syntax_tests;

/// Parse a complete module without performing AST conversion or resolution.
pub fn parse_tokens(source: &str) -> Result<GtTokens<'_>, GtParseError> {
    GtParser::parse(GtTokenKind::Module, source)
}

/// Parse a complete module into source-preserving tokens.
pub fn parse_gt_code(source: &str) -> Result<GtTokens<'_>, GtParseError> {
    parse_tokens(source)
}

/// Hand-written recursive-descent parser producing Genotype syntax tokens.
pub struct GtParser;
impl GtParser {
    pub fn parse(kind: GtTokenKind, source: &str) -> Result<GtTokens<'_>, GtParseError> {
        let mut parser = Parser {
            source,
            pos: 0,
            farthest: 0,
            expected: Vec::new(),
            depth: 0,
        };
        match parser.node(kind) {
            Ok(token) => Ok(GtTokens::new(vec![token])),

            Err(()) => {
                let expected = parser
                    .expected
                    .iter()
                    .map(|kind| kind.as_str())
                    .collect::<Vec<_>>();
                let message = match expected.as_slice() {
                    [] => "expected valid syntax".to_owned(),
                    [kind] => format!("expected {kind}"),
                    kinds => format!(
                        "expected {} or {}",
                        kinds[..kinds.len() - 1].join(", "),
                        kinds[kinds.len() - 1]
                    ),
                };
                Err(GtParseError::Syntax {
                    span: (parser.farthest, parser.farthest).into(),
                    message,
                })
            }
        }
    }
}

type Parsed<'a> = Result<GtToken<'a>, ()>;
struct Parser<'a> {
    source: &'a str,
    pos: usize,
    farthest: usize,
    expected: Vec<GtTokenKind>,
    depth: usize,
}

impl<'a> Parser<'a> {
    fn rest(&self) -> &'a str {
        &self.source[self.pos..]
    }

    fn eat(&mut self, text: &str) -> Result<(), ()> {
        if self.rest().starts_with(text) {
            self.pos += text.len();
            Ok(())
        } else {
            Err(())
        }
    }

    fn skip(&mut self) -> Result<(), ()> {
        loop {
            let before = self.pos;
            while self.rest().starts_with([' ', '\t', '\r', '\n']) {
                self.pos += 1;
            }
            if self.rest().starts_with("/*") {
                self.comment()?;
            } else if self.rest().starts_with("//")
                && !self.rest().starts_with("///")
                && !self.rest().starts_with("//!")
            {
                self.line();
            }
            if before == self.pos {
                return Ok(());
            }
        }
    }

    fn line(&mut self) {
        self.pos += self.rest().find(['\r', '\n']).unwrap_or(self.rest().len());
    }

    fn comment(&mut self) -> Result<(), ()> {
        self.eat("/*")?;
        let mut depth = 1;
        while depth > 0 {
            if self.eat("/*").is_ok() {
                depth += 1;
            } else if self.eat("*/").is_ok() {
                depth -= 1;
            } else if let Some(c) = self.rest().chars().next() {
                self.pos += c.len_utf8();
            } else {
                return Err(());
            }
        }
        Ok(())
    }

    fn child(&mut self, kind: GtTokenKind, children: &mut Vec<GtToken<'a>>) -> Result<(), ()> {
        self.skip()?;
        children.push(self.node(kind)?);
        Ok(())
    }

    fn punct(&mut self, text: &str) -> Result<(), ()> {
        self.skip()?;
        self.eat(text)
    }

    fn optional(&mut self, kind: GtTokenKind, children: &mut Vec<GtToken<'a>>) -> bool {
        let start = self.pos;
        if self.child(kind, children).is_ok() {
            true
        } else {
            self.pos = start;
            false
        }
    }

    fn choice(&mut self, kinds: &[GtTokenKind], children: &mut Vec<GtToken<'a>>) -> Result<(), ()> {
        for &kind in kinds {
            if self.optional(kind, children) {
                return Ok(());
            }
        }
        Err(())
    }

    fn adjacent_choice(
        &mut self,
        kinds: &[GtTokenKind],
        children: &mut Vec<GtToken<'a>>,
    ) -> Result<(), ()> {
        for &kind in kinds {
            if let Ok(token) = self.node(kind) {
                children.push(token);
                return Ok(());
            }
        }
        Err(())
    }

    fn annotations(&mut self, children: &mut Vec<GtToken<'a>>) {
        loop {
            if !self.optional(GtTokenKind::Attribute, children)
                && !self.optional(GtTokenKind::LineDoc, children)
            {
                break;
            }
        }
    }

    fn list(
        &mut self,
        kind: GtTokenKind,
        open: &str,
        close: &str,
        empty: bool,
        trailing: bool,
        children: &mut Vec<GtToken<'a>>,
    ) -> Result<(), ()> {
        self.eat(open)?;

        if empty {
            let save = self.pos;
            if self.punct(close).is_ok() {
                return Ok(());
            }
            self.pos = save;
        }

        self.child(kind, children)?;

        loop {
            let save = self.pos;
            if self.punct(",").is_err() {
                self.pos = save;
                break;
            }
            if self.child(kind, children).is_err() {
                if trailing {
                    break;
                }
                return Err(());
            }
        }

        self.punct(close)
    }

    fn node(&mut self, kind: GtTokenKind) -> Parsed<'a> {
        let start = self.pos;

        if self.depth >= 256 {
            return Err(());
        }

        self.depth += 1;

        let mut children = Vec::new();
        let result = self.body(kind, &mut children);

        self.depth -= 1;

        if result.is_err() {
            if self.pos > self.farthest {
                self.farthest = self.pos;
                self.expected.clear();
            }
            if self.pos == self.farthest && !self.expected.contains(&kind) {
                self.expected.push(kind);
            }
            self.pos = start;
            return Err(());
        }

        let end = self.pos;

        // Fill structural gaps with exact punctuation/trivia leaves. Leaves
        // such as strings and paths remain opaque, so comment markers in them
        // are never mistaken for comments.
        if !children.is_empty()
            || matches!(
                kind,
                GtTokenKind::Module
                    | GtTokenKind::Object
                    | GtTokenKind::Tuple
                    | GtTokenKind::ImportNames
                    | GtTokenKind::AttributeArguments
                    | GtTokenKind::RecordKey
            )
        {
            children = self.fill(start, end, children);
        }

        Ok(GtToken::new(kind, self.source, start, end, children))
    }

    fn fill(&self, start: usize, end: usize, children: Vec<GtToken<'a>>) -> Vec<GtToken<'a>> {
        let mut out = Vec::new();
        let mut cursor = start;
        for child in children {
            self.gap(cursor, child.span.0, &mut out);
            cursor = child.span.1;
            out.push(child);
        }
        self.gap(cursor, end, &mut out);
        out
    }

    fn gap(&self, mut pos: usize, end: usize, out: &mut Vec<GtToken<'a>>) {
        while pos < end {
            let start = pos;

            let mut scan = Parser {
                source: &self.source[..end],
                pos,
                farthest: pos,
                expected: vec![],
                depth: 0,
            };

            let kind = if scan.rest().starts_with("///") || scan.rest().starts_with("//!") {
                scan.pos += 3;
                GtTokenKind::Punctuation
            } else if scan.rest().starts_with("/*") {
                let _ = scan.comment();
                GtTokenKind::BlockComment
            } else if scan.rest().starts_with("//")
                && !scan.rest().starts_with("///")
                && !scan.rest().starts_with("//!")
            {
                scan.line();
                GtTokenKind::LineComment
            } else if scan.rest().starts_with([' ', '\t', '\r', '\n']) {
                while scan.rest().starts_with([' ', '\t', '\r', '\n']) {
                    scan.pos += 1;
                }
                GtTokenKind::Whitespace
            } else {
                scan.pos += scan.rest().chars().next().unwrap().len_utf8();
                GtTokenKind::Punctuation
            };

            pos = scan.pos;
            out.push(GtToken::new(kind, self.source, start, pos, vec![]));
        }
    }

    fn body(&mut self, kind: GtTokenKind, c: &mut Vec<GtToken<'a>>) -> Result<(), ()> {
        use GtTokenKind::*;

        match kind {
            Module => {
                self.skip()?;
                while self.optional(ModuleDoc, c) {}
                loop {
                    self.skip()?;
                    if self.pos == self.source.len() {
                        break;
                    }
                    self.choice(&[Import, Alias], c)?;
                }
                c.push(self.node(Eoi)?);
                Ok(())
            }

            Eoi => {
                if self.pos == self.source.len() {
                    Ok(())
                } else {
                    Err(())
                }
            }

            Name => {
                if !self.rest().starts_with(|ch: char| ch.is_ascii_alphabetic()) {
                    return Err(());
                }
                self.pos += 1;
                while self
                    .rest()
                    .starts_with(|ch: char| ch.is_ascii_alphanumeric() || ch == '_')
                {
                    self.pos += 1;
                }
                Ok(())
            }

            Alias => {
                self.annotations(c);
                self.child(Name, c)?;
                self.optional(GenericParameters, c);
                self.punct(":")?;
                self.child(Descriptors, c)
            }

            Descriptors => {
                let _ = self.eat("|");
                self.child(Descriptor, c)?;
                loop {
                    let save = self.pos;
                    if self.punct("|").is_err() {
                        self.pos = save;
                        break;
                    }
                    let after_pipe = self.pos;
                    if self.child(Descriptor, c).is_err() {
                        self.pos = after_pipe;
                        break;
                    }
                }
                Ok(())
            }

            Descriptor => {
                self.annotations(c);
                self.choice(
                    &[
                        Literal,
                        Primitive,
                        Branded,
                        Any,
                        Object,
                        Array,
                        Tuple,
                        Record,
                        InlineImport,
                        Alias,
                        Reference,
                    ],
                    c,
                )
            }

            DescriptorInner => self.choice(
                &[
                    Literal,
                    Primitive,
                    Branded,
                    Any,
                    Object,
                    Array,
                    Tuple,
                    Record,
                    InlineImport,
                    Alias,
                    Reference,
                ],
                c,
            ),

            Primitive => {
                if self.eat("string").is_ok() || self.eat("boolean").is_ok() {
                    Ok(())
                } else {
                    self.child(PrimitiveNumeric, c)
                }
            }

            PrimitiveNumeric => {
                for text in [
                    "number", "int", "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16",
                    "u32", "u64", "u128", "usize", "float", "f32", "f64",
                ] {
                    if self.eat(text).is_ok() {
                        return Ok(());
                    }
                }
                Err(())
            }
            Any => self.eat("any"),

            Branded => {
                self.eat("@")?;
                c.push(self.node(Primitive)?);
                Ok(())
            }

            Literal => self.choice(
                &[
                    LiteralString,
                    LiteralFloat,
                    LiteralInteger,
                    LiteralBoolean,
                    LiteralNull,
                ],
                c,
            ),

            LiteralString => {
                self.eat("\"")?;
                c.push(self.node(StringInner)?);
                self.eat("\"")
            }

            StringInner => {
                while let Some(ch) = self.rest().chars().next() {
                    if ch == '"' {
                        break;
                    }
                    if ch == '\\' {
                        self.node(StringEscape)?;
                    } else {
                        self.pos += ch.len_utf8();
                    }
                }
                Ok(())
            }

            StringEscape => {
                self.eat("\\")?;
                if self.eat("u").is_ok() {
                    for _ in 0..4 {
                        if !self.rest().starts_with(|ch: char| ch.is_ascii_hexdigit()) {
                            return Err(());
                        }
                        self.pos += 1;
                    }
                    Ok(())
                } else if self
                    .rest()
                    .starts_with(['"', '\\', '/', 'b', 'f', 'n', 'r', 't'])
                {
                    self.pos += 1;
                    Ok(())
                } else {
                    Err(())
                }
            }

            LiteralInteger | LiteralFloat => {
                if self.rest().starts_with(['+', '-']) {
                    self.pos += 1;
                }
                self.node(Integer)?;
                if kind == LiteralInteger {
                    return Ok(());
                }
                if self.eat(".").is_ok() {
                    self.node(Digits)?;
                    let save = self.pos;
                    if self.node(Exponent).is_err() {
                        self.pos = save;
                    }
                    Ok(())
                } else {
                    self.node(Exponent).map(|_| ())
                }
            }

            Integer => {
                if self.eat("0").is_ok() {
                    return Ok(());
                }
                if !self
                    .rest()
                    .starts_with(|ch: char| ('1'..='9').contains(&ch))
                {
                    return Err(());
                }
                self.pos += 1;
                let _ = self.node(Digits);
                Ok(())
            }

            Digits => {
                let start = self.pos;
                loop {
                    if self.rest().starts_with(|ch: char| ch.is_ascii_digit()) {
                        self.pos += 1;
                    } else if self.rest().starts_with('_')
                        && self
                            .rest()
                            .as_bytes()
                            .get(1)
                            .is_some_and(u8::is_ascii_digit)
                    {
                        self.pos += 2;
                    } else {
                        break;
                    }
                }
                if self.pos > start { Ok(()) } else { Err(()) }
            }

            Exponent => {
                if !self.rest().starts_with(['e', 'E']) {
                    return Err(());
                }
                self.pos += 1;
                if self.rest().starts_with(['+', '-']) {
                    self.pos += 1;
                }
                let start = self.pos;
                while self.rest().starts_with(|ch: char| ch.is_ascii_digit()) {
                    self.pos += 1;
                }
                if self.pos > start { Ok(()) } else { Err(()) }
            }

            LiteralBoolean => self.eat("true").or_else(|_| self.eat("false")),

            LiteralNull => self.eat("null"),

            Object => {
                self.eat("{")?;
                self.optional(Properties, c);
                self.punct("}")
            }

            Properties => {
                self.child(Property, c)?;
                loop {
                    let save = self.pos;
                    if self.punct(",").is_err() {
                        self.pos = save;
                        break;
                    }
                    if !self.optional(Property, c) {
                        break;
                    }
                }
                Ok(())
            }

            Property => self.choice(&[ExtensionProperty, OptionalProperty, RequiredProperty], c),

            ExtensionProperty => {
                self.eat("...")?;
                self.adjacent_choice(&[InlineImport, Reference], c)
            }

            RequiredProperty | OptionalProperty => {
                self.annotations(c);
                self.child(Name, c)?;
                self.punct(if kind == OptionalProperty { "?:" } else { ":" })?;
                self.child(Descriptors, c)
            }

            Array => {
                self.eat("[")?;
                self.child(Descriptors, c)?;
                self.punct("]")
            }

            Tuple => self.list(Descriptors, "(", ")", true, false, c),

            Record => {
                self.eat("{")?;
                self.child(RecordKey, c)?;
                self.punct(":")?;
                self.child(Descriptors, c)?;
                self.punct("}")
            }

            RecordKey => {
                self.eat("[")?;
                let _ = self.choice(&[Primitive, Name], c);
                self.punct("]")
            }

            Reference => {
                c.push(self.node(Name)?);
                self.optional(GenericArguments, c);
                Ok(())
            }

            GenericParameters => self.list(Name, "<", ">", false, false, c),

            GenericArguments => self.list(GenericArgument, "<", ">", false, false, c),

            GenericArgument => self.child(Descriptors, c),

            Attribute => {
                self.eat("#[")?;
                self.child(Name, c)?;
                self.optional(AttributeDescriptor, c);
                self.punct("]")
            }

            AttributeDescriptor => self.choice(
                &[AttributeAssignment, AttributeArguments, AttributeProperties],
                c,
            ),

            AttributeAssignment => {
                self.eat("=")?;
                self.child(AttributeValue, c)
            }

            AttributeArguments => self.list(AttributeValue, "(", ")", true, false, c),

            AttributeProperties => self.list(AttributeProperty, "(", ")", false, false, c),

            AttributeProperty => {
                self.child(Name, c)?;
                self.punct("=")?;
                self.child(AttributeValue, c)
            }

            AttributeValue => self.choice(&[Literal, Name], c),

            Import => {
                self.eat("use")?;
                self.child(ImportReference, c)
            }

            ImportReference => {
                c.push(self.node(Path)?);
                self.adjacent_choice(&[ImportGlob, ImportNames, Name], c)
            }

            ImportGlob => self.eat("*"),

            ImportNames => self.list(ImportName, "{", "}", true, false, c),

            ImportName => {
                self.child(Name, c)?;
                self.skip()?;
                let save = self.pos;
                if self.punct("as").is_ok() {
                    self.child(Name, c)?;
                } else {
                    self.pos = save;
                }
                Ok(())
            }

            InlineImport => {
                // Inline imports expose path segments directly, matching the
                // historical syntax shape used by the AST converter.
                c.push(self.node(PathSegment)?);
                while let Ok(segment) = self.node(PathSegment) {
                    c.push(segment);
                }
                c.push(self.node(Reference)?);
                Ok(())
            }

            Path => {
                c.push(self.node(PathSegment)?);
                while let Ok(segment) = self.node(PathSegment) {
                    c.push(segment);
                }
                Ok(())
            }

            PathSegment => {
                if self.eat("../").is_ok() || self.eat("./").is_ok() {
                    return Ok(());
                }
                let start = self.pos;
                while let Some(ch) = self.rest().chars().next() {
                    if matches!(ch, '/' | '\r' | '\n' | '<') {
                        break;
                    }
                    self.pos += ch.len_utf8();
                }
                if self.pos == start {
                    return Err(());
                }
                self.eat("/")
            }

            ModuleDoc | LineDoc => {
                self.eat(if kind == ModuleDoc { "//!" } else { "///" })?;
                if self.rest().starts_with([' ', '\t']) {
                    self.pos += 1;
                }
                c.push(self.node(Doc)?);
                Ok(())
            }

            Doc => {
                self.line();
                Ok(())
            }

            LineComment => {
                self.eat("//")?;
                if self.rest().starts_with(['/', '!']) {
                    return Err(());
                }
                self.line();
                Ok(())
            }

            BlockComment => self.comment(),

            Comment => self.adjacent_choice(&[LineComment, BlockComment], c),

            Space => {
                if self.rest().starts_with([' ', '\t']) {
                    self.pos += 1;
                    Ok(())
                } else {
                    Err(())
                }
            }

            Whitespace => {
                if self.rest().starts_with([' ', '\t', '\r', '\n']) {
                    self.pos += 1;
                    Ok(())
                } else {
                    Err(())
                }
            }

            Annotation => self.choice(&[Attribute, LineDoc], c),

            Quote => self.eat("\""),

            StringUnicode => {
                self.eat("u")?;
                for _ in 0..4 {
                    if !self.rest().starts_with(|ch: char| ch.is_ascii_hexdigit()) {
                        return Err(());
                    }
                    self.pos += 1;
                }
                Ok(())
            }

            Punctuation => Err(()),
        }
    }
}
