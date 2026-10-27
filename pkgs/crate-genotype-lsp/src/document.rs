use async_lsp::lsp_types::{
    Diagnostic, DiagnosticSeverity, NumberOrString, Position, Range, SemanticToken,
    SemanticTokenType,
};
use genotype_parser::{GtParseError, GtToken, GtTokenKind as K, parse_tokens};
use std::collections::HashMap;
use std::ops::Range as ByteRange;

pub const TOKEN_TYPES: &[SemanticTokenType] = &[
    SemanticTokenType::TYPE,
    SemanticTokenType::TYPE_PARAMETER,
    SemanticTokenType::PROPERTY,
    SemanticTokenType::DECORATOR,
    SemanticTokenType::STRING,
    SemanticTokenType::NUMBER,
    SemanticTokenType::KEYWORD,
    SemanticTokenType::COMMENT,
    SemanticTokenType::NAMESPACE,
];

#[derive(Clone, Debug)]
pub struct Name {
    pub text: String,
    pub span: ByteRange<usize>,
}

impl From<&GtToken<'_>> for Name {
    fn from(token: &GtToken<'_>) -> Self {
        Self {
            text: token.text.into(),
            span: token.span.0..token.span.1,
        }
    }
}

#[derive(Debug)]
pub struct Import {
    pub path: String,
    pub path_span: ByteRange<usize>,
    pub name: Option<Name>,
    pub alias: Option<Name>,
}

#[derive(Debug)]
pub struct Reference {
    pub name: Name,
    pub path: Option<String>,
    pub generic: Option<usize>,
}

pub struct Entity {
    pub signature: String,
    pub documentation: String,
}

pub struct Document {
    pub text: String,
    pub valid: bool,
    pub diagnostics: Vec<Diagnostic>,
    pub definitions: Vec<Name>,
    pub generics: Vec<Name>,
    pub imports: Vec<Import>,
    pub references: Vec<Reference>,
    pub highlights: Vec<(ByteRange<usize>, u32, u32)>,
    pub entities: HashMap<usize, Entity>,
    pub generic_scopes: Vec<(ByteRange<usize>, usize)>,
    pub import_end: usize,
}

impl Document {
    pub fn new(text: String) -> Self {
        let mut doc = Self {
            text: String::new(),
            valid: false,
            diagnostics: vec![],
            definitions: vec![],
            generics: vec![],
            imports: vec![],
            references: vec![],
            highlights: vec![],
            entities: HashMap::new(),
            generic_scopes: vec![],
            import_end: 0,
        };
        match parse_tokens(&text) {
            Ok(tokens) => {
                doc.valid = true;
                for token in tokens {
                    doc.visit(&token, K::Module, &[], None);
                }
            }
            Err(GtParseError::Syntax { span, message }) => {
                let end = text[span.0..]
                    .chars()
                    .next()
                    .map_or(span.0, |ch| span.0 + ch.len_utf8());
                doc.diagnostics.push(error_diagnostic(
                    Range::new(position(&text, span.0), position(&text, end)),
                    "GT001",
                    format!("Syntax error: {message}"),
                ));
            }
            Err(error) => doc.diagnostics.push(error_diagnostic(
                Range::default(),
                "syntax",
                error.to_string(),
            )),
        }
        doc.text = text;
        doc
    }

    fn highlight(&mut self, token: &GtToken<'_>, kind: u32, declaration: bool) {
        self.highlights
            .push((token.span.0..token.span.1, kind, u32::from(declaration)));
    }

    fn visit(
        &mut self,
        token: &GtToken<'_>,
        parent: K,
        generics: &[(String, usize)],
        path: Option<&str>,
    ) {
        let mut scope = generics.to_vec();
        if matches!(token.kind, K::Import | K::ModuleDoc) {
            self.import_end = self.import_end.max(token.span.1);
        }
        if matches!(
            token.kind,
            K::Alias | K::RequiredProperty | K::OptionalProperty
        ) && let Some(name) = token.children.iter().find(|t| t.kind == K::Name)
        {
            let documentation = token
                .children
                .iter()
                .filter(|t| t.kind == K::LineDoc)
                .filter_map(|t| t.children.iter().find(|t| t.kind == K::Doc))
                .map(|t| t.text)
                .collect::<Vec<_>>()
                .join("\n");
            self.entities.insert(
                name.span.0,
                Entity {
                    signature: token.text[name.span.0 - token.span.0..].to_owned(),
                    documentation,
                },
            );
        }
        if token.kind == K::Alias {
            if let Some(name) = token.children.iter().find(|t| t.kind == K::Name) {
                self.definitions.push(name.into());
            }
            if let Some(params) = token
                .children
                .iter()
                .find(|t| t.kind == K::GenericParameters)
            {
                for name in params.children.iter().filter(|t| t.kind == K::Name) {
                    self.generic_scopes
                        .push((token.span.0..token.span.1, self.generics.len()));
                    scope.push((name.text.into(), self.generics.len()));
                    self.generics.push(name.into());
                }
            }
        }
        if token.kind == K::ImportReference {
            let path_span = token
                .children
                .iter()
                .find(|t| t.kind == K::Path)
                .map(|t| t.span.0..t.span.1)
                .unwrap_or(token.span.0..token.span.1);
            let path = token
                .children
                .iter()
                .find(|t| t.kind == K::Path)
                .map(|t| t.text)
                .unwrap_or("")
                .to_string();
            for child in &token.children {
                match child.kind {
                    K::Name => self.imports.push(Import {
                        path: path.clone(),
                        path_span: path_span.clone(),
                        name: Some(child.into()),
                        alias: None,
                    }),
                    K::ImportGlob => self.imports.push(Import {
                        path: path.clone(),
                        path_span: path_span.clone(),
                        name: None,
                        alias: None,
                    }),
                    K::ImportNames => {
                        for entry in child.children.iter().filter(|t| t.kind == K::ImportName) {
                            let mut names = entry.children.iter().filter(|t| t.kind == K::Name);
                            self.imports.push(Import {
                                path: path.clone(),
                                path_span: path_span.clone(),
                                name: names.next().map(Name::from),
                                alias: names.next().map(Name::from),
                            });
                        }
                    }
                    _ => {}
                }
            }
        }
        let inline_path;
        let path = if token.kind == K::InlineImport {
            inline_path = token
                .children
                .iter()
                .filter(|t| t.kind == K::PathSegment)
                .map(|t| t.text)
                .collect::<String>();
            Some(inline_path.as_str())
        } else {
            path
        };

        match token.kind {
            K::Name => {
                let generic = scope
                    .iter()
                    .rev()
                    .find(|(name, _)| name == token.text)
                    .map(|(_, index)| *index);
                let kind = match parent {
                    K::GenericParameters => 1,
                    K::RequiredProperty | K::OptionalProperty | K::AttributeProperty => 2,
                    K::Attribute => 3,
                    K::Reference | K::RecordKey if generic.is_some() && path.is_none() => 1,
                    _ => 0,
                };
                self.highlight(
                    token,
                    kind,
                    matches!(parent, K::Alias | K::GenericParameters),
                );
                if matches!(parent, K::Reference | K::RecordKey) {
                    self.references.push(Reference {
                        name: token.into(),
                        path: path.map(str::to_owned),
                        generic: if path.is_none() { generic } else { None },
                    });
                }
                return;
            }
            K::Primitive | K::Any | K::LiteralBoolean | K::LiteralNull => {
                self.highlight(token, 6, false);
                return;
            }
            K::LiteralString => {
                self.highlight(token, 4, false);
                return;
            }
            K::LiteralInteger | K::LiteralFloat => {
                self.highlight(token, 5, false);
                return;
            }
            K::LineComment | K::BlockComment | K::LineDoc | K::ModuleDoc => {
                self.highlight(token, 7, false);
                return;
            }
            K::Path | K::PathSegment => {
                self.highlight(token, 8, false);
                return;
            }
            _ => {}
        }
        // The parser retains keywords as individual punctuation leaves.
        if token.kind == K::Import {
            self.highlights.push((token.span.0..token.span.0 + 3, 6, 0));
        }
        if token.kind == K::ImportName {
            for pair in token.children.windows(2) {
                if pair[0].kind == K::Punctuation
                    && pair[0].text == "a"
                    && pair[1].kind == K::Punctuation
                    && pair[1].text == "s"
                {
                    self.highlights.push((pair[0].span.0..pair[1].span.1, 6, 0));
                }
            }
        }
        for child in &token.children {
            self.visit(
                child,
                token.kind,
                &scope,
                if child.kind == K::GenericArguments {
                    None
                } else {
                    path
                },
            );
        }
    }

    pub fn range(&self, span: &ByteRange<usize>) -> Range {
        Range::new(
            position(&self.text, span.start),
            position(&self.text, span.end),
        )
    }

    pub fn tokens(&self) -> Vec<SemanticToken> {
        let mut spans = self.highlights.clone();
        spans.sort_by_key(|(span, _, _)| span.start);
        let mut data = vec![];
        let mut previous = Position::default();
        for (span, token_type, token_modifiers_bitset) in spans {
            let mut start = span.start;
            for line in self.text[span.clone()].split_inclusive('\n') {
                let text = line.trim_end_matches(['\r', '\n']);
                let pos = position(&self.text, start);
                if !text.is_empty() {
                    data.push(SemanticToken {
                        delta_line: pos.line - previous.line,
                        delta_start: if pos.line == previous.line {
                            pos.character - previous.character
                        } else {
                            pos.character
                        },
                        length: text.encode_utf16().count() as u32,
                        token_type,
                        token_modifiers_bitset,
                    });
                    previous = pos;
                }
                start += line.len();
            }
        }
        data
    }
}

pub fn error_diagnostic(range: Range, code: &str, message: String) -> Diagnostic {
    Diagnostic {
        range,
        severity: Some(DiagnosticSeverity::ERROR),
        code: Some(NumberOrString::String(code.into())),
        source: Some("genotype".into()),
        message,
        ..Default::default()
    }
}

pub fn position(text: &str, offset: usize) -> Position {
    let prefix = &text[..offset];
    Position::new(
        prefix.bytes().filter(|b| *b == b'\n').count() as u32,
        prefix
            .rsplit('\n')
            .next()
            .unwrap_or("")
            .encode_utf16()
            .count() as u32,
    )
}

pub fn offset(text: &str, position: Position) -> Option<usize> {
    let mut start = 0;
    for _ in 0..position.line {
        start += text.get(start..)?.find('\n')? + 1;
    }
    let mut units = 0;
    for (index, ch) in text[start..].char_indices() {
        if units == position.character {
            return Some(start + index);
        }
        if ch == '\n' {
            return None;
        }
        units += ch.len_utf16() as u32;
    }
    (units == position.character).then_some(text.len())
}
