use crate::prelude::internal::*;

fn check_leaves(token: &GtToken<'_>, output: &mut String) {
    assert_eq!(token.span.1 - token.span.0, token.text.len());
    if token.children.is_empty() {
        output.push_str(token.text);
    } else {
        let mut position = token.span.0;
        for child in &token.children {
            assert_eq!(child.span.0, position);
            check_leaves(child, output);
            position = child.span.1;
        }
        assert_eq!(position, token.span.1);
    }
}

fn check_source(source: &str) {
    let root = parse_tokens(source).unwrap().next().unwrap();
    let mut reconstructed = String::new();
    check_leaves(&root, &mut reconstructed);
    assert_eq!(reconstructed, source);
}

#[test]
fn syntax_corpus_is_lossless() {
    for entry in std::fs::read_dir("../../examples/02-syntax").unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|ext| ext == "type") {
            check_source(&std::fs::read_to_string(path).unwrap());
        }
    }
}

#[test]
fn comments_literals_and_order_are_lossless() {
    for source in [
        "// only a comment\n/* nested /* block */ comment */\n",
        "//! Module\r\n/// Alias\r\n#[x]\r\nA: { a: string, /* middle */ ...B, b?: [string | int], // end\r\n}\r\nuse ./a/A\r\n",
        "A: \"é🚀 // not a comment /* or this */ \\\" \\u00e9\"\n",
        "A: | +1_000 | -2.5e+3 | null |\n// tail\n",
        "A: { /* dangling */ }\nB: (/* tuple */)\n",
        "A: | /// member\n #[tag = \"a\"] \"a\" | /// next\n \"b\"",
        "use ./p/{A as B, C}\nA<T, U>: ./p/Foo<T | U, [T]>\n",
        "A: (string |)",
    ] {
        check_source(source);
    }
}

#[test]
fn malformed_input_is_rejected() {
    for source in [
        "A:",
        "A: {",
        "A: [string",
        "A: (string,)",
        "A<T,>: T",
        "A: Foo<string,>",
        "use ./a/{A,}",
        "A: @ string",
        "A: @ int",
        "A: @\tfloat",
        "A: @\nf64",
        "A: { [@string]: string }",
        "A: \"bad\\x\"",
        "A: \"unterminated",
        "A: string /* unclosed",
        "/* unclosed",
        "A: 01",
        "A: 1__0",
        "A: 1e",
        "A: 1e+",
        "A: 1.",
        "A: |",
        "A: || string",
        "A: []",
        "A: string\n//! late module doc",
        "/// orphan",
        "#[x]",
        "A: {... B}",
        "A: {...\nB}",
        "use ./a/ A",
        "use ./a/ {A}",
        "A: ./a/ B",
        "A: trueThing",
    ] {
        assert!(
            parse_tokens(source).is_err(),
            "accepted malformed input: {source:?}"
        );
    }
}

#[test]
fn branded_primitives_require_adjacency() {
    for primitive in ["string", "boolean", "number", "int", "u32", "float", "f64"] {
        for trivia in [" ", "\t", "\n", "/* comment */", "// comment\n"] {
            let source = format!("@{trivia}{primitive}");
            assert!(
                GtParser::parse(GtTokenKind::Branded, &source).is_err(),
                "accepted malformed brand: {source:?}"
            );
        }
        let source = format!("A: @{primitive}");
        check_source(&source);
        GtModule::parse("module".into(), &source).unwrap();
    }
}

#[test]
fn public_token_and_module_apis_agree() {
    let source = "//! Module\nA: { x: string, ...B }\nuse ./b/B\n";
    let root = parse_gt_code(source).unwrap().next().unwrap();
    assert_eq!(root.kind, GtTokenKind::Module);
    assert_eq!(root.text, source);
    let converted = convert_module("module".into(), root).unwrap();
    let parsed = GtModule::parse("module".into(), source).unwrap();
    assert_eq!(converted, parsed);
}

#[test]
fn syntax_errors_are_miette_diagnostics() {
    let source = "A: [string";
    let error = parse_tokens(source).unwrap_err();
    let diagnostic: &dyn miette::Diagnostic = &error;
    assert_eq!(diagnostic.code().unwrap().to_string(), "GT001");
    let label = diagnostic.labels().unwrap().next().unwrap();
    assert_eq!(label.offset(), source.len());
    assert_eq!(label.len(), 0);
    match error {
        GtParseError::Syntax { span, message } => {
            assert_eq!(span, (source.len(), source.len()).into());
            assert!(message.starts_with("expected "));
        }
        other => panic!("unexpected diagnostic: {other:?}"),
    }
}

#[test]
fn invalid_record_key_diagnostic_points_to_the_key() {
    for source in [
        "A: { [@string]: string }",
        "A: { [@string]: string }\nB: int",
    ] {
        let error = parse_tokens(source).unwrap_err();
        let GtParseError::Syntax { span, .. } = error else {
            panic!("unexpected diagnostic: {error:?}");
        };
        let offset = source.find('@').unwrap();
        assert_eq!(span, (offset, offset).into());
    }
}

#[test]
fn deeply_nested_input_returns_an_error() {
    let source = format!("A: {}string{}", "[".repeat(300), "]".repeat(300));
    assert!(parse_tokens(&source).is_err());
}

#[test]
fn syntax_retains_comment_kinds_and_annotation_order() {
    let source = "//! module\n/// before\n#[tag]\n/// after\nA: { /* dangling */ } // trailing\n";
    let token = parse_tokens(source).unwrap().next().unwrap();
    fn collect(token: &GtToken<'_>, kinds: &mut Vec<GtTokenKind>) {
        if matches!(
            token.kind,
            GtTokenKind::ModuleDoc
                | GtTokenKind::LineDoc
                | GtTokenKind::Attribute
                | GtTokenKind::BlockComment
                | GtTokenKind::LineComment
        ) {
            kinds.push(token.kind);
        }
        for child in &token.children {
            collect(child, kinds);
        }
    }
    let mut kinds = Vec::new();
    collect(&token, &mut kinds);
    assert_eq!(
        kinds,
        [
            GtTokenKind::ModuleDoc,
            GtTokenKind::LineDoc,
            GtTokenKind::Attribute,
            GtTokenKind::LineDoc,
            GtTokenKind::BlockComment,
            GtTokenKind::LineComment
        ]
    );
}
