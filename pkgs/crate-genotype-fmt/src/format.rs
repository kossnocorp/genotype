use crate::prelude::internal::*;
use genotype_parser::GtTokenKind as K;

pub struct GtfmtFormatter<'a> {
    source: &'a str,
}

impl<'a> GtfmtFormatter<'a> {
    /// Format a complete module. Invalid syntax returns a diagnostic without producing output.
    pub fn format(source: &'a str) -> Result<String, GtfmtError> {
        Ok(Self::format_doc(source)?.print())
    }

    /// Build a document borrowing the source, without filesystem access or an external allocator.
    pub fn format_doc(source: &'a str) -> Result<GtfmtDoc<'a>, GtParseError> {
        let root = parse_tokens(source)?
            .next()
            .ok_or(GtParseError::InvalidGrammar)?;
        let elements = Self::new(source).module(&root);
        Ok(GtfmtDoc::new(elements, source.len()))
    }

    pub(crate) fn new(source: &'a str) -> Self {
        Self { source }
    }

    pub(crate) fn module(&self, token: &GtToken<'a>) -> GtfmtElements<'a> {
        let mut doc = vec![];
        let mut previous: Option<&GtToken<'_>> = None;
        let mut previous_kind = None;
        for child in Self::significant(&token.children) {
            if child.kind == K::Eoi {
                continue;
            }
            let trailing_comment = previous
                .is_some_and(|prev| Self::is_comment(child) && !self.newline_between(prev, child));
            if let Some(prev) = previous {
                if trailing_comment {
                    doc.push(GtfmtDocEl::space());
                } else if previous_kind == Some(K::Alias)
                    || (previous_kind == Some(K::Import) && child.kind != K::Import)
                    || (previous_kind == Some(K::ModuleDoc) && child.kind != K::ModuleDoc)
                    || self.blank_between(prev, child)
                {
                    doc.push(GtfmtDocEl::empty_line());
                } else {
                    doc.push(GtfmtDocEl::hard_line());
                }
            }
            doc.extend(self.node(child));
            // Trailing comments keep the preceding entity's spacing rules.
            if !trailing_comment {
                previous_kind = Some(child.kind);
            }
            previous = Some(child);
        }
        if !doc.is_empty() {
            doc.push(GtfmtDocEl::hard_line());
        }
        doc
    }

    fn node(&self, token: &GtToken<'a>) -> GtfmtElements<'a> {
        match token.kind {
            // Opaque spelling is significant: never reinterpret strings, numbers, or paths.
            K::Literal
            | K::Primitive
            | K::Branded
            | K::Any
            | K::Name
            | K::Path
            | K::PathSegment
            | K::ModuleDoc
            | K::LineDoc
            | K::LineComment
            | K::BlockComment
            | K::Doc => vec![GtfmtDocEl::text(token.text)],
            K::Object => {
                let mut children = vec![];
                for child in &token.children {
                    if child.kind == K::Properties {
                        children.extend(child.children.iter());
                    } else {
                        children.push(child);
                    }
                }
                self.list(children, "{", "}", true, true)
            }
            K::Tuple => self.list(token.children.iter().collect(), "(", ")", false, false),
            K::GenericParameters | K::GenericArguments => {
                self.list(token.children.iter().collect(), "<", ">", false, false)
            }
            K::AttributeArguments | K::AttributeProperties => {
                self.list(token.children.iter().collect(), "(", ")", false, false)
            }
            K::ImportNames => self.list(token.children.iter().collect(), "{", "}", false, false),
            K::Array => self.enclosed(token, "[", "]", false),
            K::Record => self.enclosed(token, "{", "}", true),
            K::Descriptors => self.descriptors(token, false),
            K::Import => self.sequence(token, Some("use")),
            K::ImportName => self.sequence(token, Some("as")),
            _ if token.children.is_empty() => vec![GtfmtDocEl::text(token.text)],
            _ => self.sequence(token, None),
        }
    }

    fn sequence(&self, token: &GtToken<'a>, keyword: Option<&'static str>) -> GtfmtElements<'a> {
        let mut doc = vec![];
        let mut previous: Option<&GtToken<'_>> = None;
        let mut names = 0;
        if keyword == Some("use") {
            doc.extend([GtfmtDocEl::text("use"), GtfmtDocEl::space()]);
        }
        for child in Self::significant(&token.children) {
            if keyword.is_some() && child.kind == K::Punctuation {
                continue;
            }
            if Self::is_comment(child) {
                self.comment(&mut doc, child, previous);
                previous = Some(child);
                continue;
            }
            if let Some(prev) = previous {
                if self.blank_between(prev, child) {
                    doc.push(GtfmtDocEl::empty_line());
                } else if Self::is_line(prev) || matches!(prev.kind, K::LineDoc | K::Attribute) {
                    doc.push(GtfmtDocEl::hard_line());
                } else if Self::is_comment(prev) {
                    doc.push(GtfmtDocEl::space());
                }
            }
            if keyword == Some("as") && child.kind == K::Name {
                if names > 0 {
                    doc.extend([
                        GtfmtDocEl::space(),
                        GtfmtDocEl::text("as"),
                        GtfmtDocEl::space(),
                    ]);
                }
                names += 1;
            }
            if child.kind == K::Descriptors
                && matches!(
                    token.kind,
                    K::Alias | K::RequiredProperty | K::OptionalProperty | K::Record
                )
            {
                doc.extend(self.descriptors(child, true));
            } else if child.kind == K::Punctuation && child.text == "=" {
                doc.extend([
                    GtfmtDocEl::space(),
                    GtfmtDocEl::text("="),
                    GtfmtDocEl::space(),
                ]);
            } else {
                doc.extend(self.node(child));
            }
            previous = Some(child);
        }
        doc
    }

    fn enclosed(
        &self,
        token: &GtToken<'a>,
        open: &'static str,
        close: &'static str,
        spaced: bool,
    ) -> GtfmtElements<'a> {
        let mut inner = token.clone();
        inner.children.retain(|child| {
            !(child.kind == K::Punctuation && (child.text == open || child.text == close))
        });
        let mut body = vec![if spaced {
            GtfmtDocEl::line()
        } else {
            GtfmtDocEl::soft_line()
        }];
        body.extend(self.sequence(&inner, None));
        let mut doc = vec![GtfmtDocEl::text(open)];
        doc.extend(GtfmtDocEl::indent(body));
        doc.push(if spaced {
            GtfmtDocEl::line()
        } else {
            GtfmtDocEl::soft_line()
        });
        doc.push(GtfmtDocEl::text(close));
        GtfmtDocEl::group(doc, false)
    }

    fn descriptors(&self, token: &GtToken<'a>, after_colon: bool) -> GtfmtElements<'a> {
        let children: Vec<_> = Self::significant(&token.children)
            .filter(|child| child.kind != K::Punctuation)
            .collect();
        let union = children
            .iter()
            .filter(|child| child.kind == K::Descriptor)
            .count()
            > 1;
        if !union {
            let mut doc = if after_colon {
                vec![GtfmtDocEl::space()]
            } else {
                vec![]
            };
            // Keep the optional pipe when it owns leading comments. Otherwise
            // reparsing attaches those comments outside the descriptors node.
            if children
                .first()
                .is_some_and(|child| Self::is_comment(child))
            {
                doc.extend([GtfmtDocEl::text("|"), GtfmtDocEl::space()]);
            }
            let mut previous = None;
            for child in children {
                if Self::is_comment(child) {
                    self.comment(&mut doc, child, previous);
                } else {
                    if previous.is_some_and(Self::is_line) {
                        doc.push(GtfmtDocEl::hard_line());
                    } else if previous.is_some_and(Self::is_comment) {
                        doc.push(GtfmtDocEl::space());
                    }
                    doc.extend(self.node(child));
                }
                previous = Some(child);
            }
            return doc;
        }

        let mut doc = vec![];
        if after_colon {
            doc.push(GtfmtDocEl::line());
        }
        let leading_comment = children
            .first()
            .is_some_and(|child| Self::is_comment(child));
        if leading_comment {
            doc.extend([GtfmtDocEl::text("|"), GtfmtDocEl::space()]);
        }
        let mut previous = None;
        let mut count = 0;
        for child in children {
            if Self::is_comment(child) {
                self.comment(&mut doc, child, previous);
            } else {
                if count > 0 {
                    doc.extend([
                        GtfmtDocEl::line(),
                        GtfmtDocEl::text("|"),
                        GtfmtDocEl::space(),
                    ]);
                } else {
                    if previous.is_some_and(Self::is_line) {
                        doc.push(GtfmtDocEl::hard_line());
                    } else if previous.is_some_and(Self::is_comment) {
                        doc.push(GtfmtDocEl::space());
                    }
                    if !leading_comment {
                        doc.extend(GtfmtDocEl::if_break(vec![
                            GtfmtDocEl::text("|"),
                            GtfmtDocEl::space(),
                        ]));
                    }
                }
                doc.extend(self.node(child));
                count += 1;
            }
            previous = Some(child);
        }
        GtfmtDocEl::group(
            if after_colon {
                GtfmtDocEl::indent(doc)
            } else {
                doc
            },
            false,
        )
    }

    fn list(
        &self,
        children: Vec<&GtToken<'a>>,
        open: &'static str,
        close: &'static str,
        trailing: bool,
        object: bool,
    ) -> GtfmtElements<'a> {
        let multiline_object = object
            && children
                .iter()
                .any(|child| child.text.contains(['\r', '\n']));
        let children: Vec<_> = children
            .into_iter()
            .filter(|child| !matches!(child.kind, K::Whitespace | K::Punctuation))
            .collect();
        if children.is_empty() {
            return vec![GtfmtDocEl::text(open), GtfmtDocEl::text(close)];
        }
        let count = children
            .iter()
            .filter(|child| !Self::is_comment(child))
            .count();
        let commented_imports =
            open == "{" && !object && children.iter().any(|child| Self::contains_comment(child));
        let mut body = vec![if object {
            GtfmtDocEl::line()
        } else {
            GtfmtDocEl::soft_line()
        }];
        let mut previous = None;
        let mut index = 0;
        for child in children {
            if Self::is_comment(child) {
                self.comment(&mut body, child, previous);
            } else {
                if let Some(prev) = previous {
                    body.push(if self.blank_between(prev, child) {
                        GtfmtDocEl::empty_line()
                    } else {
                        GtfmtDocEl::line()
                    });
                }
                body.extend(self.node(child));
                index += 1;
                if index < count || trailing {
                    body.push(GtfmtDocEl::text(","));
                }
            }
            previous = Some(child);
        }
        let mut doc = vec![GtfmtDocEl::text(open)];
        doc.extend(GtfmtDocEl::indent(body));
        doc.extend([
            if object {
                GtfmtDocEl::line()
            } else {
                GtfmtDocEl::soft_line()
            },
            GtfmtDocEl::text(close),
        ]);
        GtfmtDocEl::group(doc, multiline_object || commented_imports)
    }

    fn comment(
        &self,
        doc: &mut GtfmtElements<'a>,
        token: &GtToken<'a>,
        previous: Option<&GtToken<'_>>,
    ) {
        if let Some(previous) = previous {
            doc.push(if self.blank_between(previous, token) {
                GtfmtDocEl::empty_line()
            } else if self.newline_between(previous, token) || Self::is_line(previous) {
                GtfmtDocEl::hard_line()
            } else {
                GtfmtDocEl::space()
            });
        }
        doc.push(GtfmtDocEl::text(token.text));
        // A line comment must force the containing group to break, even when it
        // is the final item before a delimiter.
        if Self::is_line(token) {
            doc.push(GtfmtDocEl::hard_line());
        }
    }

    fn newline_between(&self, left: &GtToken<'_>, right: &GtToken<'_>) -> bool {
        self.source[left.span.1..right.span.0].contains(['\r', '\n'])
    }

    fn blank_between(&self, left: &GtToken<'_>, right: &GtToken<'_>) -> bool {
        self.source[left.span.1..right.span.0]
            .bytes()
            .filter(|&ch| ch == b'\n')
            .count()
            > 1
    }

    fn significant<'t, 'b>(children: &'t [GtToken<'b>]) -> impl Iterator<Item = &'t GtToken<'b>> {
        children
            .iter()
            .filter(|child| !matches!(child.kind, K::Whitespace | K::Space | K::Eoi))
    }

    fn is_comment(token: &GtToken<'_>) -> bool {
        matches!(token.kind, K::LineComment | K::BlockComment)
    }

    fn is_line(token: &GtToken<'_>) -> bool {
        token.kind == K::LineComment
    }

    fn contains_comment(token: &GtToken<'_>) -> bool {
        Self::is_comment(token) || token.children.iter().any(Self::contains_comment)
    }
}
