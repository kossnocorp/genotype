use super::*;
use crate::document::{Document, offset, position};
use crate::project::Project;
use lsp::{Position, Url};

fn uri(name: &str) -> Url {
    Url::parse(&format!("file:///project/{name}.type")).unwrap()
}

fn project(files: &[(&str, &str)]) -> Project {
    Project {
        documents: files
            .iter()
            .map(|(name, text)| (uri(name), Document::new((*text).into())))
            .collect(),
    }
}

fn at(text: &str, needle: &str) -> Position {
    position(text, text.find(needle).unwrap())
}

#[test]
fn rename_tracks_imports_aliases_globs_and_inline_references() {
    let main = "use ./model/{User as Person}\nuse ./model/User\nUsers: [User]\nPeople: [Person]\nInline: ./model/User\n";
    let glob = "use ./model/*\nOther: User\n";
    let p = project(&[
        ("model", "User: { name: string }\n"),
        ("main", main),
        ("glob", glob),
        ("unrelated", "User: string\nOther: User\n"),
    ]);
    let edits = p
        .rename(&uri("model"), Position::new(0, 0), "Account")
        .unwrap()
        .unwrap()
        .changes
        .unwrap();
    assert_eq!(edits.len(), 3);
    assert_eq!(edits[&uri("main")].len(), 4);
    assert_eq!(edits[&uri("glob")].len(), 1);
    assert!(!edits.contains_key(&uri("unrelated")));
    let definition = p.definition(&uri("main"), at(main, "Person]\n")).unwrap();
    assert_eq!(definition.uri, uri("model"));
    assert_eq!(definition.range.start, Position::new(0, 0));
    let alias = p
        .rename(&uri("main"), at(main, "Person]\n"), "Human")
        .unwrap()
        .unwrap()
        .changes
        .unwrap();
    assert_eq!(alias.len(), 1);
    assert_eq!(alias[&uri("main")].len(), 2);
}

#[test]
fn generic_parameters_are_scoped_and_inline_arguments_are_local() {
    let text =
        "T: string\nBox<T>: { value: T }\nOther<T>: [T]\nExternal<T>: ./model/Box<T>\nGlobal: T\n";
    let p = project(&[("main", text), ("model", "Box<T>: [T]\n")]);
    let edits = p
        .rename(&uri("main"), at(text, "T>: {"), "Item")
        .unwrap()
        .unwrap()
        .changes
        .unwrap();
    assert_eq!(edits[&uri("main")].len(), 2);
    let definition = p.definition(&uri("main"), at(text, "T>\nGlobal")).unwrap();
    assert_eq!(definition.range.start, at(text, "T>: ./"));
    let edits = p
        .rename(&uri("main"), Position::new(0, 0), "Value")
        .unwrap()
        .unwrap()
        .changes
        .unwrap();
    assert_eq!(edits[&uri("main")].len(), 2);
}

#[test]
fn rejects_capture_reserved_names_and_invalid_documents() {
    let p = project(&[("main", "User: string\nOther: User\n")]);
    for name in ["Other", "string", "bad-name", "", "1Name"] {
        assert!(p.rename(&uri("main"), Position::new(0, 0), name).is_err());
    }
    let p = project(&[("main", "User: string\n"), ("broken", "Oops: {")]);
    assert!(
        p.rename(&uri("main"), Position::new(0, 0), "Account")
            .is_err()
    );
    assert!(p.prepare(&uri("main"), Position::new(0, 7)).is_none());
}

#[test]
fn comments_strings_and_properties_are_not_symbol_references() {
    let text = "User: string\n// User\nOther: { User: \"User\", value: User }\n";
    let p = project(&[("main", text)]);
    let edits = p
        .rename(&uri("main"), Position::new(0, 0), "Account")
        .unwrap()
        .unwrap()
        .changes
        .unwrap();
    assert_eq!(edits[&uri("main")].len(), 2);
    assert!(p.definition(&uri("main"), at(text, "User: \"")).is_none());
}

#[test]
fn cyclic_and_ambiguous_imports_do_not_resolve_arbitrarily() {
    let p = project(&[
        ("a", "use ./b/*\nuse ./c/*\nRef: User"),
        ("b", "use ./a/*\nUser: string"),
        ("c", "User: boolean"),
    ]);
    assert!(p.definition(&uri("a"), Position::new(2, 5)).is_none());
    let p = project(&[("a", "use ./b/*\nRef: Missing"), ("b", "use ./a/*")]);
    assert!(p.definition(&uri("a"), Position::new(1, 5)).is_none());
}

#[test]
fn utf16_positions_and_multiline_semantic_tokens() {
    let text = "/* 😀\r\ncomment */\r\nUser: { name: \"😀\", other: User }\r\n";
    let doc = Document::new(text.into());
    assert!(doc.valid);
    for (index, _) in text.char_indices() {
        assert_eq!(offset(text, position(text, index)), Some(index));
    }
    assert_eq!(offset("😀", Position::new(0, 1)), None);
    let mut pos = Position::default();
    let mut highlighted = vec![];
    for token in doc.tokens() {
        pos.character = if token.delta_line == 0 {
            pos.character + token.delta_start
        } else {
            token.delta_start
        };
        pos.line += token.delta_line;
        let start = offset(text, pos).unwrap();
        let end = offset(text, Position::new(pos.line, pos.character + token.length)).unwrap();
        let fragment = &text[start..end];
        assert!(!fragment.contains(['\r', '\n']));
        highlighted.push((fragment, token.token_type));
    }
    assert!(highlighted.contains(&("\"😀\"", 4)));
    assert!(highlighted.contains(&("name", 2)));
    assert!(highlighted.contains(&("User", 0)));
    assert!(highlighted.contains(&("/* 😀", 7)));
}

#[test]
fn import_keywords_do_not_overlap_comments() {
    let text = "use ./model/{\nUser as /* as */ Person\n}\nRef: Person";
    let doc = Document::new(text.into());
    assert!(doc.valid);
    let mut spans = doc.highlights.clone();
    spans.sort_by_key(|(span, _, _)| span.start);
    assert!(
        spans
            .windows(2)
            .all(|pair| pair[0].0.end <= pair[1].0.start)
    );
    assert_eq!(doc.tokens().len(), spans.len());
}

#[test]
fn diagnostics_report_syntax_and_unresolved_references_at_utf16_ranges() {
    let text = "Thing: { label: \"😀\", value: Missing }";
    let p = project(&[("main", text)]);
    let diagnostics = p.diagnostics(&uri("main"));
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].range.start, at(text, "Missing"));
    assert_eq!(
        diagnostics[0].range.end.character - diagnostics[0].range.start.character,
        7
    );
    assert_eq!(
        diagnostics[0].severity,
        Some(lsp::DiagnosticSeverity::ERROR)
    );
    assert!(diagnostics[0].message.contains("Missing"));
    let doc = Document::new("Thing: {".into());
    assert_eq!(doc.diagnostics.len(), 1);
    assert_eq!(doc.diagnostics[0].range.start, Position::new(0, 8));
    assert!(doc.diagnostics[0].message.starts_with("Syntax error:"));
}

#[test]
fn diagnostics_resolve_imports_and_generics_without_external_false_positives() {
    let p = project(&[
        ("model", "User: string"),
        (
            "main",
            "use ./model/{User as Person}\nuse package/External\nBox<T>: { user: Person, item: T, external: External }\nBad: Unknown\nInline: ./model/Unknown",
        ),
    ]);
    let diagnostics = p.diagnostics(&uri("main"));
    assert_eq!(diagnostics.len(), 2);
    assert!(diagnostics.iter().all(|d| d.message.contains("Unknown")));
    let p = project(&[("main", "use package/*\nValue: External")]);
    assert!(p.diagnostics(&uri("main")).is_empty());
}

#[test]
fn diagnostics_find_missing_imports_and_suppress_syntax_error_cascades() {
    let p = project(&[
        ("main", "use ./missing/{One, Two}\nuse ./model/Unknown"),
        ("model", "User: string"),
    ]);
    let diagnostics = p.diagnostics(&uri("main"));
    assert_eq!(diagnostics.len(), 2);
    assert!(diagnostics[0].message.contains("Module not found"));
    assert!(diagnostics[1].message.contains("Unknown"));
    let p = project(&[
        ("main", "use ./model/User\nRef: User"),
        ("model", "User: {"),
    ]);
    assert!(p.diagnostics(&uri("main")).is_empty());
    assert_eq!(p.diagnostics(&uri("model")).len(), 1);
}

fn hover_markdown(p: &Project, file: &str, text: &str, needle: &str) -> String {
    let hover = p.hover(&uri(file), at(text, needle)).unwrap();
    let lsp::HoverContents::Markup(contents) = hover.contents else {
        panic!("Expected Markdown hover")
    };
    assert_eq!(contents.kind, lsp::MarkupKind::Markdown);
    contents.value
}

#[test]
fn hover_renders_markdown_docs_on_declarations_references_and_import_aliases() {
    let model = "/// Markdown and **ordered**, repeated tags are retained verbatim.\n///\n/// - First\n/// - Second\nJsDoc: [Tag]\nTag: string\n";
    let main = "use ./model/{JsDoc as Documentation}\nValue: Documentation\n";
    let p = project(&[("model", model), ("main", main)]);
    let expected = "```genotype\nJsDoc: [Tag]\n```\n\n---\n\nMarkdown and **ordered**, repeated tags are retained verbatim.\n\n- First\n- Second";
    assert_eq!(hover_markdown(&p, "model", model, "JsDoc:"), expected);
    assert_eq!(
        hover_markdown(&p, "main", main, "Documentation\n"),
        expected
    );
    assert_eq!(hover_markdown(&p, "main", main, "JsDoc as"), expected);
}

#[test]
fn hover_includes_property_docs_and_scoped_type_parameters() {
    let text = "Box<T>: {\n/// The **item**.\nvalue: T\n}\n";
    let p = project(&[("main", text)]);
    assert!(hover_markdown(&p, "main", text, "value:").contains("The **item**."));
    assert!(hover_markdown(&p, "main", text, "T\n").contains("type parameter"));
}

fn apply_edits(text: &str, edits: &[lsp::TextEdit]) -> String {
    let mut edits: Vec<_> = edits
        .iter()
        .map(|edit| {
            (
                offset(text, edit.range.start).unwrap(),
                offset(text, edit.range.end).unwrap(),
                &edit.new_text,
            )
        })
        .collect();
    edits.sort_by_key(|(start, end, _)| std::cmp::Reverse((*start, *end)));
    let mut result = text.to_string();
    for (start, end, value) in edits {
        result.replace_range(start..end, value);
    }
    result
}

fn completion_edits(item: &lsp::CompletionItem) -> Vec<lsp::TextEdit> {
    let Some(lsp::CompletionTextEdit::Edit(edit)) = &item.text_edit else {
        panic!("Expected text edit")
    };
    let mut edits = vec![edit.clone()];
    edits.extend(item.additional_text_edits.clone().unwrap_or_default());
    edits
}

#[test]
fn completion_autoimports_project_symbols_with_documentation() {
    let text = "/// A value.\nValue: Js";
    let p = project(&[
        ("main", text),
        ("model", "/// Markdown docs.\nJsDoc: string"),
    ]);
    let items = p.completions(&uri("main"), position(text, text.len()));
    let item = items.iter().find(|item| item.label == "JsDoc").unwrap();
    assert!(
        matches!(&item.documentation, Some(lsp::Documentation::MarkupContent(content)) if content.value.contains("Markdown docs."))
    );
    let updated = apply_edits(text, &completion_edits(item));
    assert_eq!(updated, "use ./model/JsDoc\n/// A value.\nValue: JsDoc");
    let p = project(&[("main", &updated), ("model", "JsDoc: string")]);
    assert!(p.diagnostics(&uri("main")).is_empty());
}

#[test]
fn completion_works_in_incomplete_objects_and_scopes_generics() {
    let text = "Local: string\nBox<T>: { value: ";
    let items = project(&[("main", text), ("model", "Remote: string")])
        .completions(&uri("main"), position(text, text.len()));
    for name in ["T", "Local", "Remote", "string"] {
        assert!(
            items.iter().any(|item| item.label == name),
            "missing {name}"
        );
    }
    let text = "Box<T>: T\nValue: ";
    let items = project(&[("main", text)]).completions(&uri("main"), position(text, text.len()));
    assert!(!items.iter().any(|item| item.label == "T"));
}

#[test]
fn completion_uses_existing_imports_and_avoids_shadowing() {
    let text = "use ./model/{JsDoc as Docs}\nuse ./other/*\nLocal: string\nValue: ";
    let items = project(&[
        ("main", text),
        ("model", "JsDoc: string\nLocal: number"),
        ("other", "Other: boolean"),
    ])
    .completions(&uri("main"), position(text, text.len()));
    for name in ["Docs", "Other", "Local"] {
        let matching: Vec<_> = items.iter().filter(|item| item.label == name).collect();
        assert_eq!(matching.len(), 1);
        assert!(matching[0].additional_text_edits.is_none());
    }
}

#[test]
fn completion_disambiguates_same_named_exports_and_replaces_whole_word() {
    let text = "Value: Thing";
    let items = project(&[
        ("main", text),
        ("a", "Thing: string"),
        ("b", "Thing: number"),
    ])
    .completions(&uri("main"), Position::new(0, 9));
    let matching: Vec<_> = items.iter().filter(|item| item.label == "Thing").collect();
    assert_eq!(matching.len(), 2);
    assert_ne!(matching[0].label_details, matching[1].label_details);
    assert_eq!(
        apply_edits(text, &completion_edits(matching[0])),
        "use ./a/Thing\nValue: Thing"
    );
}

#[test]
fn completion_in_inline_and_named_imports_does_not_add_imports() {
    for text in ["Value: ./model/Js", "use ./model/Js"] {
        let items = project(&[("main", text), ("model", "JsDoc: string")])
            .completions(&uri("main"), position(text, text.len()));
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].label, "JsDoc");
        assert!(items[0].additional_text_edits.is_none());
    }
}

#[test]
fn completion_does_not_offer_types_in_comments_strings_or_declaration_names() {
    for text in [
        "// Js",
        "Value: \"Js\"",
        "Js: string",
        "Value: { Js: string }",
    ] {
        let pos = position(text, text.find("Js").unwrap() + 2);
        let items =
            project(&[("main", text), ("model", "JsDoc: string")]).completions(&uri("main"), pos);
        assert!(items.is_empty(), "unexpected suggestions for {text}");
    }
}

#[test]
fn quickfix_adds_import_after_module_docs_preserving_crlf() {
    let text = "//! Module docs.\r\n/// Value docs.\r\nValue: JsDoc\r\n";
    let p = project(&[("main", text), ("model", "JsDoc: string")]);
    let pos = at(text, "JsDoc");
    let actions = p.import_actions(&uri("main"), lsp::Range::new(pos, pos));
    assert_eq!(actions.len(), 1);
    let lsp::CodeActionOrCommand::CodeAction(action) = &actions[0] else {
        panic!("Expected code action")
    };
    assert_eq!(action.kind, Some(lsp::CodeActionKind::QUICKFIX));
    let updated = apply_edits(
        text,
        &action.edit.as_ref().unwrap().changes.as_ref().unwrap()[&uri("main")],
    );
    assert_eq!(
        updated,
        "//! Module docs.\r\nuse ./model/JsDoc\r\n/// Value docs.\r\nValue: JsDoc\r\n"
    );
    let p = project(&[("main", &updated), ("model", "JsDoc: string")]);
    assert!(p.diagnostics(&uri("main")).is_empty());
    assert!(
        p.import_actions(&uri("main"), lsp::Range::new(pos, pos))
            .is_empty()
    );
}

#[test]
fn autoimport_uses_relative_paths_and_preserves_existing_import_comments() {
    let text = "use ../existing/Existing\n// keep this\nValue: Js";
    let items = project(&[
        ("nested/main", text),
        ("existing", "Existing: string"),
        ("model", "JsDoc: string"),
    ])
    .completions(&uri("nested/main"), position(text, text.len()));
    let item = items.iter().find(|item| item.label == "JsDoc").unwrap();
    assert_eq!(
        apply_edits(text, &completion_edits(item)),
        "use ../existing/Existing\nuse ../model/JsDoc\n// keep this\nValue: JsDoc"
    );
}

#[test]
fn loads_unopened_files_root_imports_and_unsaved_buffers() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir(dir.path().join("types")).unwrap();
    std::fs::write(dir.path().join("genotype.toml"), "src = 'types'").unwrap();
    let model = dir.path().join("types/model.type");
    let main = dir.path().join("types/main.type");
    std::fs::write(&model, "User: string").unwrap();
    std::fs::write(&main, "Ref: string").unwrap();
    let main_uri = Url::from_file_path(&main).unwrap();
    let model_uri = Url::from_file_path(&model).unwrap();
    let text = "use ~/model/User\nRef: User";
    let open = HashMap::from([(main_uri.clone(), text.into())]);
    let p = Project::load(&[dir.path().into()], &open);
    assert_eq!(
        p.definition(&main_uri, Position::new(1, 5)).unwrap().uri,
        model_uri
    );
    assert_eq!(
        p.rename(&main_uri, Position::new(1, 5), "Account")
            .unwrap()
            .unwrap()
            .changes
            .unwrap()
            .len(),
        2
    );
    std::fs::write(&model, "Account: string").unwrap();
    let p = Project::load(&[dir.path().into()], &HashMap::new());
    assert!(p.definition(&main_uri, Position::new(0, 5)).is_none());
}
