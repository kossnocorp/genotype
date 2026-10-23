# `genotype_parser`

This is an internal crate of the Genotype Programming Language. It is not intended to be used by external users.

See the [Genotype website](https://genotype-lang.org) for more information about the project.

## Parsing pipeline

`parse/` implements a manual recursive-descent parser. `parse_tokens(source)`
returns a stream containing a module token; `token.rs` defines the token kinds,
byte spans, exact source slices, and ordered child tokens. Children include
whitespace, comments, and punctuation so traversing the leaves reconstructs the
original source. Documentation and attributes remain in their original order.

`convert/` converts these tokens into `genotype_tree` AST nodes and resolution
metadata. Use `convert_module(module_id, token)` for an already parsed module,
or the existing `GtModule::parse` API for both phases. AST conversion deliberately
discards formatting trivia; formatters should consume the tokens instead.

`GtParser::parse(kind, source)` parses an individual syntax kind;
`parse_gt_code(source)` and `parse_tokens(source)` parse complete modules.
All entry points return native `GtTokens` and `GtParseError` values. Syntax
errors carry source spans and implement miette's `Diagnostic` trait directly.
There are no parser compatibility adapters or generic token-kind parameters.

AST converters use `GtToken::into_semantic_children()` to omit punctuation and
ordinary comments. Formatting tools should traverse the full `children` vector
instead. Tests cover AST output, source reconstruction, invalid input, and
diagnostic spans.

Token kinds use Rust's PascalCase convention; `as_str()` and `Display` retain
the syntax spellings used in diagnostics. Token spans are UTF-8 byte offsets.
Nesting is bounded to avoid
unbounded parser recursion. The token tree borrows its input and does not resolve
imports or normalize literal/path spellings.
