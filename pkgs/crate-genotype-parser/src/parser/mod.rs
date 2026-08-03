use chumsky::{
    error::Rich,
    extra,
    input::{MappedInput, Stream},
    prelude::*,
};
use std::{fmt, ops::Range};

use crate::prelude::internal::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize)]
pub enum GtTokenKind {
    Whitespace,
    Newline,
    LineComment,
    BlockComment,
    ModuleDocComment,
    DocComment,
    Identifier,
    Integer,
    Float,
    String,
    HashBracket,
    Ellipsis,
    OptionalColon,
    Colon,
    Comma,
    Pipe,
    Less,
    Greater,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    Equal,
    At,
    Star,
    Slash,
    Dot,
    Unknown,
}

impl GtTokenKind {
    fn is_trivia(self) -> bool {
        matches!(
            self,
            Self::Whitespace | Self::Newline | Self::LineComment | Self::BlockComment
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize)]
pub struct GtToken {
    pub kind: GtTokenKind,
    pub span: GtSpan,
}

impl GtToken {
    pub fn text<'src>(&self, source: &'src str) -> &'src str {
        &source[self.span.0..self.span.1]
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct GtTokens<'src> {
    source: &'src str,
    tokens: Vec<GtToken>,
}

impl<'src> GtTokens<'src> {
    pub fn source(&self) -> &'src str {
        self.source
    }
    pub fn tokens(&self) -> &[GtToken] {
        &self.tokens
    }
    pub fn text(&self, token: &GtToken) -> &'src str {
        token.text(self.source)
    }
    pub fn round_trip(&self) -> String {
        self.tokens
            .iter()
            .map(|token| token.text(self.source))
            .collect()
    }
}

fn raw_lexer<'src>()
-> impl Parser<'src, &'src str, Vec<(GtTokenKind, Range<usize>)>, extra::Err<Rich<'src, char>>> {
    let block = just("/*")
        .then(any().and_is(just("*/").not()).repeated())
        .then(just("*/"))
        .ignored()
        .to(GtTokenKind::BlockComment);
    let string = just('"')
        .then(choice((just('\\').then(any()).ignored(), none_of("\"\\").ignored())).repeated())
        .then(just('"').or_not())
        .ignored()
        .to(GtTokenKind::String);
    let number = one_of("+-")
        .or_not()
        .then(one_of("0123456789").then(one_of("0123456789_").repeated()))
        .then(
            just('.')
                .then(one_of("0123456789_").repeated().at_least(1))
                .or_not(),
        )
        .then(
            one_of("eE")
                .then(one_of("+-").or_not())
                .then(one_of("0123456789").repeated().at_least(1))
                .or_not(),
        )
        .map_with(|_, e| {
            let text: &str = e.slice();
            if text.contains(['.', 'e', 'E']) {
                GtTokenKind::Float
            } else {
                GtTokenKind::Integer
            }
        });
    let token = choice((
        just("//!")
            .then(any().and_is(one_of("\r\n").not()).repeated())
            .ignored()
            .to(GtTokenKind::ModuleDocComment),
        just("///")
            .then(any().and_is(one_of("\r\n").not()).repeated())
            .ignored()
            .to(GtTokenKind::DocComment),
        just("//")
            .then(any().and_is(one_of("\r\n").not()).repeated())
            .ignored()
            .to(GtTokenKind::LineComment),
        block,
        string,
        number,
        any()
            .filter(char::is_ascii_alphabetic)
            .then(
                any()
                    .filter(|c: &char| c.is_ascii_alphanumeric() || *c == '_')
                    .repeated(),
            )
            .ignored()
            .to(GtTokenKind::Identifier),
        just("#[").to(GtTokenKind::HashBracket),
        just("...").to(GtTokenKind::Ellipsis),
        just("?:").to(GtTokenKind::OptionalColon),
        one_of(" \t")
            .repeated()
            .at_least(1)
            .to(GtTokenKind::Whitespace),
        choice((just("\r\n"), just("\n"), just("\r"))).to(GtTokenKind::Newline),
        one_of(":,|<>{}[]()=@*/.").map(|c| match c {
            ':' => GtTokenKind::Colon,
            ',' => GtTokenKind::Comma,
            '|' => GtTokenKind::Pipe,
            '<' => GtTokenKind::Less,
            '>' => GtTokenKind::Greater,
            '{' => GtTokenKind::LeftBrace,
            '}' => GtTokenKind::RightBrace,
            '[' => GtTokenKind::LeftBracket,
            ']' => GtTokenKind::RightBracket,
            '(' => GtTokenKind::LeftParen,
            ')' => GtTokenKind::RightParen,
            '=' => GtTokenKind::Equal,
            '@' => GtTokenKind::At,
            '*' => GtTokenKind::Star,
            '/' => GtTokenKind::Slash,
            '.' => GtTokenKind::Dot,
            _ => unreachable!(),
        }),
        any().map(|_| GtTokenKind::Unknown),
    ));
    token
        .map_with(|kind, e| {
            let span: chumsky::span::SimpleSpan<usize> = e.span();
            (kind, span.start..span.end)
        })
        .repeated()
        .collect()
}

pub fn lex(source: &str) -> GtTokens<'_> {
    let tokens = raw_lexer()
        .parse(source)
        .into_output()
        .unwrap_or_default()
        .into_iter()
        .map(|(kind, span)| GtToken {
            kind,
            span: span.into(),
        })
        .collect();
    GtTokens { source, tokens }
}

#[derive(Clone, Debug)]
struct Spanned<T> {
    span: GtSpan,
    value: T,
}
type Name = Spanned<String>;
type Annotation = Spanned<AnnotationKind>;

#[derive(Clone, Debug)]
enum AnnotationKind {
    Doc(String),
    Attribute(AttributeSyntax),
}
#[derive(Clone, Debug)]
struct AttributeSyntax {
    name: Name,
    descriptor: Option<AttributeDescriptorSyntax>,
}
#[derive(Clone, Debug)]
enum AttributeDescriptorSyntax {
    Assignment(Spanned<AttributeValueSyntax>),
    Arguments(Vec<Spanned<AttributeValueSyntax>>),
    Properties(Vec<Spanned<(Name, Spanned<AttributeValueSyntax>)>>),
}
#[derive(Clone, Debug)]
enum AttributeValueSyntax {
    Literal(LiteralSyntax),
    Name(Name),
}
#[derive(Clone, Debug)]
enum LiteralSyntax {
    Null,
    String(String),
    Integer(String),
    Float(String),
    Boolean(bool),
}
#[derive(Clone, Debug)]
struct ModuleSyntax {
    docs: Vec<Annotation>,
    items: Vec<ItemSyntax>,
}
#[derive(Clone, Debug)]
enum ItemSyntax {
    Import(Spanned<ImportSyntax>),
    Alias(Spanned<AliasSyntax>),
}
#[derive(Clone, Debug)]
struct ImportSyntax {
    path: Spanned<String>,
    reference: Spanned<ImportReferenceSyntax>,
}
#[derive(Clone, Debug)]
enum ImportReferenceSyntax {
    Glob,
    Name(Name),
    Names(Vec<Spanned<(Name, Option<Name>)>>),
}
#[derive(Clone, Debug)]
struct AliasSyntax {
    annotations: Vec<Annotation>,
    name: Name,
    generics: Vec<Name>,
    descriptor: Spanned<DescriptorsSyntax>,
}
#[derive(Clone, Debug)]
struct DescriptorsSyntax {
    values: Vec<Spanned<DescriptorSyntax>>,
}
#[derive(Clone, Debug)]
struct DescriptorSyntax {
    annotations: Vec<Annotation>,
    kind: DescriptorKindSyntax,
}
#[derive(Clone, Debug)]
enum DescriptorKindSyntax {
    Primitive(String),
    Any,
    Literal(LiteralSyntax),
    Branded(Spanned<String>),
    Object(Vec<Spanned<PropertySyntax>>),
    Array(Box<Spanned<DescriptorsSyntax>>),
    Tuple(Vec<Spanned<DescriptorsSyntax>>),
    Record(Spanned<Option<String>>, Box<Spanned<DescriptorsSyntax>>),
    Reference(ReferenceSyntax),
    InlineImport(InlineImportSyntax),
    Alias(Box<Spanned<AliasSyntax>>),
}
#[derive(Clone, Debug)]
struct ReferenceSyntax {
    name: Name,
    arguments: Vec<Spanned<DescriptorsSyntax>>,
}
#[derive(Clone, Debug)]
struct InlineImportSyntax {
    path: Spanned<String>,
    reference: ReferenceSyntax,
}
#[derive(Clone, Debug)]
enum PropertySyntax {
    Extension(Spanned<DescriptorKindSyntax>),
    Property {
        annotations: Vec<Annotation>,
        name: Name,
        required: bool,
        descriptor: Spanned<DescriptorsSyntax>,
    },
}

type TokenInput<'src> = MappedInput<
    GtTokenKind,
    GtSpan,
    Stream<std::vec::IntoIter<(GtTokenKind, GtSpan)>>,
    fn((GtTokenKind, GtSpan)) -> (GtTokenKind, GtSpan),
>;
type TokExtra<'src> = extra::Err<Rich<'src, GtTokenKind, GtSpan>>;
fn just_token<'src>(
    kind: GtTokenKind,
) -> impl Parser<'src, TokenInput<'src>, GtTokenKind, TokExtra<'src>> + Clone {
    just(kind)
}
fn spanned<'src, O, P>(
    parser: P,
) -> impl Parser<'src, TokenInput<'src>, Spanned<O>, TokExtra<'src>> + Clone
where
    P: Parser<'src, TokenInput<'src>, O, TokExtra<'src>> + Clone,
{
    parser.map_with(|value, e| Spanned {
        span: e.span(),
        value,
    })
}
fn name<'src>(
    source: &'src str,
) -> impl Parser<'src, TokenInput<'src>, Name, TokExtra<'src>> + Clone {
    spanned(
        just_token(GtTokenKind::Identifier)
            .map_with(move |_, e| source[e.span().0..e.span().1].to_owned()),
    )
}
fn literal<'src>(
    source: &'src str,
) -> impl Parser<'src, TokenInput<'src>, Spanned<LiteralSyntax>, TokExtra<'src>> + Clone {
    spanned(choice((
        just_token(GtTokenKind::String).map_with(move |_, e| {
            LiteralSyntax::String(source[e.span().0 + 1..e.span().1.saturating_sub(1)].to_owned())
        }),
        just_token(GtTokenKind::Integer).map_with(move |_, e| {
            LiteralSyntax::Integer(source[e.span().0..e.span().1].to_owned())
        }),
        just_token(GtTokenKind::Float)
            .map_with(move |_, e| LiteralSyntax::Float(source[e.span().0..e.span().1].to_owned())),
        just_token(GtTokenKind::Identifier).try_map(move |_, span| match &source[span.0..span.1] {
            "true" => Ok(LiteralSyntax::Boolean(true)),
            "false" => Ok(LiteralSyntax::Boolean(false)),
            "null" => Ok(LiteralSyntax::Null),
            _ => Err(Rich::custom(span, "expected literal")),
        }),
    )))
}

fn slash_chain<'src>(
    source: &'src str,
) -> impl Parser<'src, TokenInput<'src>, (Spanned<String>, Name), TokExtra<'src>> + Clone {
    let component = name(source);
    let named_segment = component
        .clone()
        .then_ignore(just_token(GtTokenKind::Slash))
        .ignored();
    let relative_segment = just_token(GtTokenKind::Dot)
        .then(just_token(GtTokenKind::Dot).or_not())
        .then_ignore(just_token(GtTokenKind::Slash))
        .ignored();
    let first_segment = choice((
        just_token(GtTokenKind::Dot)
            .then(just_token(GtTokenKind::Dot).or_not())
            .then_ignore(just_token(GtTokenKind::Slash))
            .ignored(),
        component
            .clone()
            .then_ignore(just_token(GtTokenKind::Slash))
            .ignored(),
    ));

    first_segment
        .then(
            recursive(|chain| {
                choice((named_segment.clone(), relative_segment.clone()))
                    .then(choice((
                        chain,
                        component
                            .clone()
                            .map(|reference| (Vec::<()>::new(), reference)),
                    )))
                    .map(|(_, (segments, reference))| (segments, reference))
            })
            .or(component
                .clone()
                .map(|reference| (Vec::<()>::new(), reference))),
        )
        .map_with(move |(_, (_, reference)), extra| {
            let path_end = reference.span.0;
            (
                Spanned {
                    span: GtSpan(extra.span().0, path_end),
                    value: source[extra.span().0..path_end - 1].to_owned(),
                },
                reference,
            )
        })
}

fn path_prefix<'src>(
    source: &'src str,
) -> impl Parser<'src, TokenInput<'src>, Spanned<String>, TokExtra<'src>> + Clone {
    let segment = choice((
        name(source)
            .then_ignore(just_token(GtTokenKind::Slash))
            .ignored(),
        just_token(GtTokenKind::Dot)
            .then(just_token(GtTokenKind::Dot).or_not())
            .then_ignore(just_token(GtTokenKind::Slash))
            .ignored(),
    ));

    spanned(
        segment
            .repeated()
            .at_least(1)
            .collect::<Vec<_>>()
            .map_with(move |_, extra| source[extra.span().0..extra.span().1 - 1].to_owned()),
    )
    .map(|mut path| {
        path.span.1 -= 1;
        path
    })
}

fn syntax_parser<'src>(
    source: &'src str,
) -> impl Parser<'src, TokenInput<'src>, ModuleSyntax, TokExtra<'src>> {
    let name = name(source);
    let literal = literal(source);
    let value = choice((
        literal
            .clone()
            .map(|v| v.map(AttributeValueSyntax::Literal)),
        name.clone().map(|n| Spanned {
            span: n.span,
            value: AttributeValueSyntax::Name(n),
        }),
    ));
    let attribute = spanned(
        just_token(GtTokenKind::HashBracket)
            .ignore_then(name.clone())
            .then(
                choice((
                    spanned(just_token(GtTokenKind::Equal).ignore_then(value.clone())).map(|v| {
                        AttributeDescriptorSyntax::Assignment(Spanned {
                            span: v.span,
                            value: v.value.value,
                        })
                    }),
                    just_token(GtTokenKind::LeftParen)
                        .ignore_then(choice((
                            spanned(
                                name.clone()
                                    .then_ignore(just_token(GtTokenKind::Equal))
                                    .then(value.clone()),
                            )
                            .separated_by(just_token(GtTokenKind::Comma))
                            .at_least(1)
                            .collect()
                            .map(AttributeDescriptorSyntax::Properties),
                            value
                                .clone()
                                .separated_by(just_token(GtTokenKind::Comma))
                                .collect()
                                .map(AttributeDescriptorSyntax::Arguments),
                        )))
                        .then_ignore(just_token(GtTokenKind::RightParen)),
                ))
                .or_not(),
            )
            .then_ignore(just_token(GtTokenKind::RightBracket))
            .map(|(name, descriptor)| AttributeSyntax { name, descriptor }),
    );
    let doc = spanned(just_token(GtTokenKind::DocComment).map_with(move |_, e| {
        source[e.span().0 + 3 + usize::from(source[e.span().0 + 3..].starts_with(' '))..e.span().1]
            .to_owned()
    }))
    .map(|d| d.map(AnnotationKind::Doc));
    let annotation = choice((doc, attribute.map(|a| a.map(AnnotationKind::Attribute))));
    let annotations = annotation.repeated().collect::<Vec<_>>();

    let descriptor = recursive(|descriptor| {
        let descriptors = descriptor
            .clone()
            .separated_by(just_token(GtTokenKind::Pipe))
            .allow_leading()
            .allow_trailing()
            .at_least(1)
            .collect::<Vec<_>>()
            .map_with(|values, e| Spanned {
                span: e.span(),
                value: DescriptorsSyntax { values },
            });
        let reference = name
            .clone()
            .then(
                just_token(GtTokenKind::Less)
                    .ignore_then(
                        descriptors
                            .clone()
                            .separated_by(just_token(GtTokenKind::Comma))
                            .at_least(1)
                            .collect(),
                    )
                    .then_ignore(just_token(GtTokenKind::Greater))
                    .or_not(),
            )
            .map(|(name, arguments)| ReferenceSyntax {
                name,
                arguments: arguments.unwrap_or_default(),
            });
        let inline = slash_chain(source)
            .then(
                just_token(GtTokenKind::Less)
                    .ignore_then(
                        descriptors
                            .clone()
                            .separated_by(just_token(GtTokenKind::Comma))
                            .at_least(1)
                            .collect(),
                    )
                    .then_ignore(just_token(GtTokenKind::Greater))
                    .or_not(),
            )
            .map(|((path, name), arguments)| InlineImportSyntax {
                path,
                reference: ReferenceSyntax {
                    name,
                    arguments: arguments.unwrap_or_default(),
                },
            });
        let primitive = name.clone().try_map(move |n, span| {
            if primitive_kind(&n.value).is_some() {
                Ok(DescriptorKindSyntax::Primitive(n.value))
            } else if n.value == "any" {
                Ok(DescriptorKindSyntax::Any)
            } else {
                Err(Rich::custom(span, "expected primitive"))
            }
        });
        let property = choice((
            spanned(
                just_token(GtTokenKind::Ellipsis).ignore_then(spanned(choice((
                    inline.clone().map(DescriptorKindSyntax::InlineImport),
                    reference.clone().map(DescriptorKindSyntax::Reference),
                )))),
            )
            .map(|v| Spanned {
                span: v.span,
                value: PropertySyntax::Extension(v.value),
            }),
            spanned(
                annotations
                    .clone()
                    .then(name.clone())
                    .then(choice((
                        just_token(GtTokenKind::Colon).to(true),
                        just_token(GtTokenKind::OptionalColon).to(false),
                    )))
                    .then(descriptors.clone())
                    .map(|(((annotations, name), required), descriptor)| {
                        PropertySyntax::Property {
                            annotations,
                            name,
                            required,
                            descriptor,
                        }
                    }),
            ),
        ));
        let object = just_token(GtTokenKind::LeftBrace)
            .ignore_then(
                property
                    .separated_by(just_token(GtTokenKind::Comma))
                    .allow_trailing()
                    .collect(),
            )
            .then_ignore(just_token(GtTokenKind::RightBrace))
            .map(DescriptorKindSyntax::Object);
        let record = just_token(GtTokenKind::LeftBrace)
            .ignore_then(spanned(
                just_token(GtTokenKind::LeftBracket)
                    .ignore_then(name.clone().map(|n| n.value).or_not())
                    .then_ignore(just_token(GtTokenKind::RightBracket)),
            ))
            .then_ignore(just_token(GtTokenKind::Colon))
            .then(descriptors.clone())
            .then_ignore(just_token(GtTokenKind::RightBrace))
            .map(|(key, value)| DescriptorKindSyntax::Record(key, Box::new(value)));
        let alias = annotations
            .clone()
            .then(name.clone())
            .then(
                just_token(GtTokenKind::Less)
                    .ignore_then(
                        name.clone()
                            .separated_by(just_token(GtTokenKind::Comma))
                            .at_least(1)
                            .collect(),
                    )
                    .then_ignore(just_token(GtTokenKind::Greater))
                    .or_not(),
            )
            .then_ignore(just_token(GtTokenKind::Colon))
            .then(descriptors.clone())
            .map_with(|(((annotations, name), generics), descriptor), e| {
                DescriptorKindSyntax::Alias(Box::new(Spanned {
                    span: e.span(),
                    value: AliasSyntax {
                        annotations,
                        name,
                        generics: generics.unwrap_or_default(),
                        descriptor,
                    },
                }))
            });
        annotations
            .clone()
            .then(spanned(choice((
                literal
                    .clone()
                    .map(|l| DescriptorKindSyntax::Literal(l.value)),
                just_token(GtTokenKind::At)
                    .ignore_then(name.clone())
                    .map(|n| DescriptorKindSyntax::Branded(n.map(|s| s))),
                record,
                object,
                just_token(GtTokenKind::LeftBracket)
                    .ignore_then(descriptors.clone())
                    .then_ignore(just_token(GtTokenKind::RightBracket))
                    .map(|d| DescriptorKindSyntax::Array(Box::new(d))),
                just_token(GtTokenKind::LeftParen)
                    .ignore_then(
                        descriptors
                            .clone()
                            .separated_by(just_token(GtTokenKind::Comma))
                            .collect(),
                    )
                    .then_ignore(just_token(GtTokenKind::RightParen))
                    .map(DescriptorKindSyntax::Tuple),
                alias,
                primitive,
                inline.map(DescriptorKindSyntax::InlineImport),
                reference.map(DescriptorKindSyntax::Reference),
            ))))
            .map(|(annotations, kind)| Spanned {
                span: kind.span,
                value: DescriptorSyntax {
                    annotations,
                    kind: kind.value,
                },
            })
    });
    let descriptors = descriptor
        .separated_by(just_token(GtTokenKind::Pipe))
        .allow_leading()
        .allow_trailing()
        .at_least(1)
        .collect::<Vec<_>>()
        .map_with(|values, e| Spanned {
            span: e.span(),
            value: DescriptorsSyntax { values },
        });
    let alias = spanned(
        annotations
            .then(name.clone())
            .then(
                just_token(GtTokenKind::Less)
                    .ignore_then(
                        name.clone()
                            .separated_by(just_token(GtTokenKind::Comma))
                            .at_least(1)
                            .collect(),
                    )
                    .then_ignore(just_token(GtTokenKind::Greater))
                    .or_not(),
            )
            .then_ignore(just_token(GtTokenKind::Colon))
            .then(descriptors)
            .map(
                |(((annotations, name), generics), descriptor)| AliasSyntax {
                    annotations,
                    name,
                    generics: generics.unwrap_or_default(),
                    descriptor,
                },
            ),
    )
    .map(ItemSyntax::Alias);
    let import_name = spanned(
        name.clone().then(
            just_token(GtTokenKind::Identifier)
                .try_map(move |_, span| {
                    if &source[span.0..span.1] == "as" {
                        Ok(())
                    } else {
                        Err(Rich::custom(span, "expected `as`"))
                    }
                })
                .ignore_then(name.clone())
                .or_not(),
        ),
    );
    let grouped_import_ref = spanned(choice((
        just_token(GtTokenKind::Star).to(ImportReferenceSyntax::Glob),
        just_token(GtTokenKind::LeftBrace)
            .ignore_then(
                import_name
                    .separated_by(just_token(GtTokenKind::Comma))
                    .collect(),
            )
            .then_ignore(just_token(GtTokenKind::RightBrace))
            .map(ImportReferenceSyntax::Names),
    )));
    let import = spanned(
        just_token(GtTokenKind::Identifier)
            .try_map(move |_, span| {
                if &source[span.0..span.1] == "use" {
                    Ok(())
                } else {
                    Err(Rich::custom(span, "expected `use`"))
                }
            })
            .ignore_then(choice((
                path_prefix(source).then(grouped_import_ref),
                slash_chain(source).map(|(path, name)| {
                    let path = Spanned {
                        span: GtSpan(path.span.0, path.span.1 - 1),
                        value: path.value,
                    };
                    let reference = Spanned {
                        span: name.span,
                        value: ImportReferenceSyntax::Name(name),
                    };
                    (path, reference)
                }),
            )))
            .map(|(path, reference)| ImportSyntax { path, reference }),
    )
    .map(ItemSyntax::Import);
    let module_doc = spanned(
        just_token(GtTokenKind::ModuleDocComment).map_with(move |_, e| {
            source[e.span().0 + 3 + usize::from(source[e.span().0 + 3..].starts_with(' '))
                ..e.span().1]
                .to_owned()
        }),
    )
    .map(|d| d.map(AnnotationKind::Doc));
    module_doc
        .repeated()
        .collect()
        .then(choice((import, alias)).repeated().collect())
        .then_ignore(end())
        .map(|(docs, items)| ModuleSyntax { docs, items })
}

trait MapSpanned<T> {
    fn map<U>(self, f: impl FnOnce(T) -> U) -> Spanned<U>;
}
impl<T> MapSpanned<T> for Spanned<T> {
    fn map<U>(self, f: impl FnOnce(T) -> U) -> Spanned<U> {
        Spanned {
            span: self.span,
            value: f(self.value),
        }
    }
}

fn parse_syntax(source: &str) -> Result<ModuleSyntax, GtParseError> {
    let tokens = lex(source);
    let significant: Vec<_> = tokens
        .tokens
        .iter()
        .filter(|t| !t.kind.is_trivia())
        .cloned()
        .collect();
    let end = GtSpan(source.len(), source.len());
    let pairs = significant
        .into_iter()
        .map(|t| (t.kind, t.span))
        .collect::<Vec<_>>();
    let input = Stream::from_iter(pairs).map(
        end,
        std::convert::identity as fn((GtTokenKind, GtSpan)) -> (GtTokenKind, GtSpan),
    );
    let (output, errors) = syntax_parser(source).parse(input).into_output_errors();
    let errors: Vec<_> = errors.into_iter().collect();
    if let Some(output) = output {
        Ok(output)
    } else if errors.is_empty() {
        Err(syntax_error(end, "parser produced no module"))
    } else {
        Err(GtParseError::Syntax {
            errors: errors
                .into_iter()
                .map(|e| GtSyntaxError {
                    span: *e.span(),
                    message: format!("{e:?}"),
                })
                .collect(),
        })
    }
}

fn syntax_error(span: GtSpan, message: impl Into<String>) -> GtParseError {
    GtParseError::Syntax {
        errors: vec![GtSyntaxError {
            span,
            message: message.into(),
        }],
    }
}
fn primitive_kind(value: &str) -> Option<GtPrimitiveKind> {
    Some(match value {
        "boolean" => GtPrimitiveKind::Boolean,
        "string" => GtPrimitiveKind::String,
        "number" => GtPrimitiveKind::Number,
        "int" | "i64" => GtPrimitiveKind::Int64,
        "i8" => GtPrimitiveKind::Int8,
        "i16" => GtPrimitiveKind::Int16,
        "i32" => GtPrimitiveKind::Int32,
        "i128" => GtPrimitiveKind::Int128,
        "isize" => GtPrimitiveKind::IntSize,
        "u8" => GtPrimitiveKind::IntU8,
        "u16" => GtPrimitiveKind::IntU16,
        "u32" => GtPrimitiveKind::IntU32,
        "u64" => GtPrimitiveKind::IntU64,
        "u128" => GtPrimitiveKind::IntU128,
        "usize" => GtPrimitiveKind::IntUSize,
        "float" | "f64" => GtPrimitiveKind::Float64,
        "f32" => GtPrimitiveKind::Float32,
        _ => return None,
    })
}

fn lower_annotations(
    values: Vec<Annotation>,
    context: &mut GtContext,
) -> Result<(Option<GtDoc>, Vec<GtAttribute>), GtParseError> {
    let mut doc: Option<GtDoc> = None;
    let mut attributes = vec![];
    for value in values {
        match value.value {
            AnnotationKind::Doc(text) => {
                let content = GtSpan(
                    value.span.0 + 3 + usize::from(text.len() + value.span.0 + 3 < value.span.1),
                    value.span.1,
                );
                doc = Some(match doc {
                    Some(old) => GtDoc(GtSpan(old.0.0, content.1), format!("{}\n{}", old.1, text)),
                    None => GtDoc(content, text),
                })
            }
            AnnotationKind::Attribute(a) => {
                attributes.push(lower_attribute(value.span, a, context)?)
            }
        }
    }
    Ok((doc, attributes))
}
fn lower_attribute(
    span: GtSpan,
    value: AttributeSyntax,
    context: &mut GtContext,
) -> Result<GtAttribute, GtParseError> {
    Ok(GtAttribute {
        span,
        name: GtAttributeName {
            span: value.name.span,
            value: value.name.value.into(),
        },
        descriptor: value
            .descriptor
            .map(|d| lower_attribute_descriptor(d, context))
            .transpose()?,
    })
}
fn lower_attribute_descriptor(
    value: AttributeDescriptorSyntax,
    context: &mut GtContext,
) -> Result<GtAttributeDescriptor, GtParseError> {
    Ok(match value {
        AttributeDescriptorSyntax::Assignment(v) => {
            let span = v.span;
            let value = Spanned {
                span: GtSpan(span.0 + 2, span.1),
                value: v.value,
            };
            GtAttributeDescriptor::Assignment(GtAttributeAssignment {
                span,
                value: lower_attribute_value(value, context)?,
            })
        }
        AttributeDescriptorSyntax::Arguments(v) => GtAttributeDescriptor::Arguments(
            v.into_iter()
                .map(|v| lower_attribute_value(v, context))
                .collect::<Result<_, _>>()?,
        ),
        AttributeDescriptorSyntax::Properties(v) => GtAttributeDescriptor::Properties(
            v.into_iter()
                .map(|v| {
                    Ok(GtAttributeProperty {
                        span: v.span,
                        name: GtAttributeKey {
                            span: v.value.0.span,
                            value: v.value.0.value.into(),
                        },
                        value: lower_attribute_value(v.value.1, context)?,
                    })
                })
                .collect::<Result<_, GtParseError>>()?,
        ),
    })
}
fn lower_attribute_value(
    value: Spanned<AttributeValueSyntax>,
    context: &mut GtContext,
) -> Result<GtAttributeValue, GtParseError> {
    Ok(match value.value {
        AttributeValueSyntax::Literal(v) => {
            GtAttributeValue::Literal(lower_literal(value.span, v, context, None, vec![])?)
        }
        AttributeValueSyntax::Name(n) => {
            GtAttributeValue::Identifier(GtIdentifier::new(n.span, n.value.into()))
        }
    })
}
fn lower_literal(
    span: GtSpan,
    value: LiteralSyntax,
    _context: &mut GtContext,
    doc: Option<GtDoc>,
    attributes: Vec<GtAttribute>,
) -> Result<GtLiteral, GtParseError> {
    let value = match value {
        LiteralSyntax::Null => GtLiteralValue::Null,
        LiteralSyntax::String(v) => GtLiteralValue::String(v),
        LiteralSyntax::Integer(v) => GtLiteralValue::Integer(
            v.replace('_', "")
                .parse()
                .map_err(|_| syntax_error(span, "integer is out of range"))?,
        ),
        LiteralSyntax::Float(v) => GtLiteralValue::Float(
            v.replace('_', "")
                .parse()
                .map_err(|_| syntax_error(span, "invalid float"))?,
        ),
        LiteralSyntax::Boolean(v) => GtLiteralValue::Boolean(v),
    };
    Ok(GtLiteral {
        span,
        doc,
        attributes,
        value,
    })
}

fn lower_descriptor(
    value: Spanned<DescriptorsSyntax>,
    context: &mut GtContext,
) -> Result<GtDescriptor, GtParseError> {
    let union = value.value.values.len() > 1;
    if union {
        context.enter_named_parent(GtContextParent::Anonymous);
    }
    let mut values = value
        .value
        .values
        .into_iter()
        .map(|v| lower_descriptor_one(v, context))
        .collect::<Result<Vec<_>, _>>()?;
    if union {
        context.exit_named_parent(value.span, GtNode::Descriptor)?;
    }
    if values.len() == 1 {
        Ok(values.remove(0))
    } else {
        Ok(GtDescriptor::Union(GtUnion {
            span: value.span,
            doc: None,
            attributes: vec![],
            descriptors: values,
        }))
    }
}
fn lower_descriptor_one(
    value: Spanned<DescriptorSyntax>,
    context: &mut GtContext,
) -> Result<GtDescriptor, GtParseError> {
    let (doc, attributes) = lower_annotations(value.value.annotations, context)?;
    let span = value.span;
    Ok(match value.value.kind {
        DescriptorKindSyntax::Primitive(v) => GtDescriptor::Primitive(GtPrimitive {
            span,
            kind: primitive_kind(&v).unwrap(),
            doc,
            attributes,
        }),
        DescriptorKindSyntax::Any => GtDescriptor::Any(GtAny {
            span,
            doc,
            attributes,
        }),
        DescriptorKindSyntax::Literal(v) => {
            GtDescriptor::Literal(lower_literal(span, v, context, doc, attributes)?)
        }
        DescriptorKindSyntax::Branded(v) => {
            let primitive = GtPrimitive {
                span: v.span,
                kind: primitive_kind(&v.value)
                    .ok_or_else(|| syntax_error(v.span, "expected primitive after `@`"))?,
                doc: None,
                attributes: vec![],
            };
            let name = context.get_name(&span, &primitive.to_string());
            GtDescriptor::Branded(GtBranded {
                span,
                doc,
                attributes,
                id: context.get_definition_id(&name),
                name,
                primitive,
            })
        }
        DescriptorKindSyntax::Reference(v) => {
            GtDescriptor::Reference(lower_reference(span, v, context, doc, attributes)?)
        }
        DescriptorKindSyntax::InlineImport(v) => {
            GtDescriptor::InlineImport(lower_inline(span, v, context, doc, attributes)?)
        }
        DescriptorKindSyntax::Array(v) => GtDescriptor::Array(Box::new(GtArray {
            span,
            doc,
            attributes,
            descriptor: lower_descriptor(*v, context)?,
        })),
        DescriptorKindSyntax::Tuple(v) => GtDescriptor::Tuple(GtTuple {
            span,
            doc,
            attributes,
            descriptors: v
                .into_iter()
                .map(|v| lower_descriptor(v, context))
                .collect::<Result<_, _>>()?,
        }),
        DescriptorKindSyntax::Record(key, descriptor) => GtDescriptor::Record(Box::new(GtRecord {
            span,
            doc,
            attributes,
            key: lower_record_key(key, context)?,
            descriptor: lower_descriptor(*descriptor, context)?,
        })),
        DescriptorKindSyntax::Object(properties) => {
            GtDescriptor::Object(lower_object(span, properties, context, doc, attributes)?)
        }
        DescriptorKindSyntax::Alias(alias) => GtDescriptor::Alias(Box::new(lower_alias(
            *alias,
            context,
            Some((doc, attributes)),
        )?)),
    })
}
fn lower_reference(
    span: GtSpan,
    value: ReferenceSyntax,
    context: &mut GtContext,
    doc: Option<GtDoc>,
    attributes: Vec<GtAttribute>,
) -> Result<GtReference, GtParseError> {
    let identifier = GtIdentifier::new(value.name.span, value.name.value.into());
    let arguments = value
        .arguments
        .into_iter()
        .map(|v| {
            let span = v.span;
            Ok(GtGenericArgument {
                span,
                descriptor: lower_descriptor(v, context)?,
            })
        })
        .collect::<Result<_, GtParseError>>()?;
    context.resolve.references.insert(identifier.clone());
    context.resolve_reference_identifier_as_generic_parameter(&identifier);
    Ok(GtReference {
        span,
        doc,
        attributes,
        id: GtReferenceId(context.module_id.clone(), span),
        identifier,
        arguments,
    })
}
fn lower_inline(
    span: GtSpan,
    value: InlineImportSyntax,
    context: &mut GtContext,
    doc: Option<GtDoc>,
    attributes: Vec<GtAttribute>,
) -> Result<GtInlineImport, GtParseError> {
    let path = GtPath::parse(value.path.span, &context.module_id, &value.path.value)?;
    let (name, arguments) = lower_reference_parts(value.reference, context)?;
    context
        .resolve
        .deps
        .insert(GtModuleSource::new(&span, &path));
    Ok(GtInlineImport {
        span,
        doc,
        attributes,
        name,
        arguments,
        path,
    })
}
fn lower_reference_parts(
    value: ReferenceSyntax,
    context: &mut GtContext,
) -> Result<(GtIdentifier, Vec<GtGenericArgument>), GtParseError> {
    let name = GtIdentifier::new(value.name.span, value.name.value.into());
    let args = value
        .arguments
        .into_iter()
        .map(|v| {
            let span = v.span;
            Ok(GtGenericArgument {
                span,
                descriptor: lower_descriptor(v, context)?,
            })
        })
        .collect::<Result<_, GtParseError>>()?;
    Ok((name, args))
}
fn lower_record_key(
    value: Spanned<Option<String>>,
    context: &mut GtContext,
) -> Result<GtRecordKey, GtParseError> {
    let span = value.span;
    Ok(match value.value.as_deref().unwrap_or("") {
        "" | "string" => GtRecordKey::String(span),
        "number" => GtRecordKey::Number(span),
        "int" | "i64" => GtRecordKey::Int64(span),
        "i8" => GtRecordKey::Int8(span),
        "i16" => GtRecordKey::Int16(span),
        "i32" => GtRecordKey::Int32(span),
        "i128" => GtRecordKey::Int128(span),
        "isize" => GtRecordKey::IntSize(span),
        "u8" => GtRecordKey::IntU8(span),
        "u16" => GtRecordKey::IntU16(span),
        "u32" => GtRecordKey::IntU32(span),
        "u64" => GtRecordKey::IntU64(span),
        "u128" => GtRecordKey::IntU128(span),
        "usize" => GtRecordKey::IntUSize(span),
        "float" | "f64" => GtRecordKey::Float64(span),
        "f32" => GtRecordKey::Float32(span),
        name => {
            let identifier = GtIdentifier::new(GtSpan(span.0 + 1, span.1 - 1), name.into());
            context.resolve.references.insert(identifier.clone());
            context.resolve_reference_identifier_as_generic_parameter(&identifier);
            GtRecordKey::Reference(GtReference {
                span: identifier.0,
                doc: None,
                attributes: vec![],
                id: GtReferenceId(context.module_id.clone(), identifier.0),
                identifier,
                arguments: vec![],
            })
        }
    })
}
fn lower_object(
    span: GtSpan,
    values: Vec<Spanned<PropertySyntax>>,
    context: &mut GtContext,
    doc: Option<GtDoc>,
    attributes: Vec<GtAttribute>,
) -> Result<GtObject, GtParseError> {
    let name = context.name_object(span)?;
    let named = matches!(name, GtObjectName::Named(_));
    if named {
        context.enter_named_parent(GtContextParent::Anonymous);
    }
    let mut object = GtObject {
        span,
        doc,
        attributes,
        name,
        extensions: vec![],
        properties: vec![],
    };
    for value in values {
        match value.value {
            PropertySyntax::Extension(inner) => {
                let reference = match inner.value {
                    DescriptorKindSyntax::Reference(v) => {
                        lower_reference(inner.span, v, context, None, vec![])?
                    }
                    DescriptorKindSyntax::InlineImport(v) => {
                        let i = lower_inline(inner.span, v, context, None, vec![])?;
                        let r = GtReference {
                            span: i.span,
                            doc: i.doc,
                            attributes: i.attributes,
                            id: GtReferenceId(context.module_id.clone(), i.span),
                            identifier: i.name,
                            arguments: i.arguments,
                        };
                        context.resolve.references.insert(r.identifier.clone());
                        r
                    }
                    _ => unreachable!(),
                };
                object.extensions.push(GtExtension {
                    span: value.span,
                    reference,
                });
            }
            PropertySyntax::Property {
                annotations,
                name,
                required,
                descriptor,
            } => {
                let (doc, attributes) = lower_annotations(annotations, context)?;
                let key = GtKey(name.span, name.value.into());
                context.enter_named_parent(GtContextParent::Property(key.clone()));
                let descriptor = lower_descriptor(descriptor, context)?;
                context.exit_named_parent(value.span, GtNode::Property)?;
                object.properties.push(GtProperty {
                    span: value.span,
                    doc,
                    attributes,
                    name: key,
                    descriptor,
                    required,
                });
            }
        }
    }
    if named {
        context.exit_named_parent(span, GtNode::Object)?;
    }
    Ok(object)
}
fn lower_alias(
    value: Spanned<AliasSyntax>,
    context: &mut GtContext,
    outer: Option<(Option<GtDoc>, Vec<GtAttribute>)>,
) -> Result<GtAlias, GtParseError> {
    let (mut doc, mut attributes) = outer.unwrap_or_else(|| context.take_annotation_or_default());
    let (inner_doc, mut inner_attributes) = lower_annotations(value.value.annotations, context)?;
    doc = match (doc, inner_doc) {
        (Some(a), Some(b)) => Some(GtDoc(GtSpan(a.0.0, b.0.1), format!("{}\n{}", a.1, b.1))),
        (a, b) => a.or(b),
    };
    attributes.append(&mut inner_attributes);
    let name = GtIdentifier::new(value.value.name.span, value.value.name.value.into());
    context.resolve.exports.push(name.clone());
    context.enter_named_parent(GtContextParent::Alias(name.clone()));
    let generics = value
        .value
        .generics
        .into_iter()
        .map(|n| GtGenericParameter {
            span: n.span,
            identifier: GtIdentifier::new(n.span, n.value.into()),
        })
        .collect::<Vec<_>>();
    context.enter_generics_scope(&generics);
    let descriptor = lower_descriptor(value.value.descriptor, context)?;
    context.exit_named_parent(value.span, GtNode::Alias)?;
    context.exit_generics_scope(value.span, GtNode::Alias)?;
    Ok(GtAlias {
        id: GtDefinitionId(context.module_id.clone(), name.1.clone()),
        span: value.span,
        doc,
        attributes,
        name,
        generics,
        descriptor,
    })
}

pub(crate) fn parse_gt_code(
    module_id: GtModuleId,
    source: &str,
) -> Result<GtModuleParse, GtParseError> {
    let syntax = parse_syntax(source)?;
    let mut context = GtContext::new(module_id);
    let (doc, _) = lower_annotations(syntax.docs, &mut context)?;
    let mut imports = vec![];
    let mut aliases = vec![];
    for item in syntax.items {
        match item {
            ItemSyntax::Alias(a) => aliases.push(lower_alias(a, &mut context, None)?),
            ItemSyntax::Import(i) => {
                let path =
                    GtPath::parse(i.value.path.span, &context.module_id, &i.value.path.value)?;
                context
                    .resolve
                    .deps
                    .insert(GtModuleSource::new(&i.span, &path));
                let reference = match i.value.reference.value {
                    ImportReferenceSyntax::Glob => GtImportReference::Glob(i.value.reference.span),
                    ImportReferenceSyntax::Name(n) => GtImportReference::Name(
                        i.value.reference.span,
                        GtIdentifier::new(n.span, n.value.into()),
                    ),
                    ImportReferenceSyntax::Names(v) => GtImportReference::Names(
                        i.value.reference.span,
                        v.into_iter()
                            .map(|v| {
                                let n = GtIdentifier::new(v.value.0.span, v.value.0.value.into());
                                match v.value.1 {
                                    Some(a) => GtImportName::Alias(
                                        v.span,
                                        n,
                                        GtIdentifier::new(a.span, a.value.into()),
                                    ),
                                    None => GtImportName::Name(v.span, n),
                                }
                            })
                            .collect(),
                    ),
                };
                imports.push(GtImport {
                    span: i.span,
                    path,
                    reference,
                });
            }
        }
    }
    Ok(GtModuleParse {
        module: GtModule {
            id: context.module_id.clone(),
            doc,
            imports,
            aliases,
        },
        resolve: context.resolve,
    })
}

impl fmt::Display for GtSyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at {}..{}", self.message, self.span.0, self.span.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lossless_tokens_cover_source() {
        let source = "//! docs\nA: { /// p\n x?: \"v\", /* c */ ...foo/Bar } $";
        let tokens = lex(source);
        assert_eq!(tokens.round_trip(), source);
        let mut end = 0;
        for token in tokens.tokens() {
            assert_eq!(token.span.0, end);
            assert!(token.span.1 > token.span.0);
            assert_eq!(token.text(source), &source[token.span.0..token.span.1]);
            end = token.span.1;
        }
        assert_eq!(end, source.len());
        assert!(
            tokens
                .tokens()
                .iter()
                .any(|t| t.kind == GtTokenKind::Unknown)
        );
    }
    #[test]
    fn malformed_input_has_errors() {
        let error = parse_syntax("One: [ @ ]\nTwo: { x: @ }").unwrap_err();
        let GtParseError::Syntax { errors } = error else {
            panic!("expected syntax error")
        };
        assert!(!errors.is_empty());
    }

    #[test]
    fn lexes_paths_losslessly() {
        let source = "use ../../author/{Author, Genre as Kind}\nBook: ./models/Book<string>";
        let tokens = lex(source);
        assert_eq!(tokens.round_trip(), source);
        assert_eq!(
            tokens
                .tokens()
                .iter()
                .filter(|token| token.kind == GtTokenKind::Slash)
                .map(|token| token.span)
                .collect::<Vec<_>>(),
            vec![
                GtSpan(6, 7),
                GtSpan(9, 10),
                GtSpan(16, 17),
                GtSpan(48, 49),
                GtSpan(55, 56)
            ]
        );
    }

    #[test]
    fn parses_import_and_inline_slash_chains() {
        let syntax = parse_syntax("Books: ../../models/Book<string>").unwrap();
        assert_eq!(syntax.items.len(), 1);
        let ItemSyntax::Alias(alias) = &syntax.items[0] else {
            panic!("expected alias")
        };
        let DescriptorKindSyntax::InlineImport(inline) =
            &alias.value.descriptor.value.values[0].value.kind
        else {
            panic!("expected inline import")
        };
        assert_eq!(inline.path.value, "../../models");
        assert_eq!(inline.reference.name.value, "Book");
        assert_eq!(inline.reference.arguments.len(), 1);
    }
}
