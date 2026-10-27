use super::*;
use crate::document::{Document, offset, position};
use crate::project::Project;
use lsp::{Position, Url};

fn uri(name: &str) -> Url {
    Url::parse(&format!("file:///project/{name}.type")).unwrap()
}

#[test]
fn generated_symbols_use_compiler_target_paths_and_unsaved_sources() {
    use crate::project::generated::{GeneratedSymbolsParams, generated_symbols};
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::create_dir_all(root.join("types/schemas/shop_goods")).unwrap();
    std::fs::write(
        root.join("genotype.toml"),
        r#"
root = "types"
src = "schemas"
dist = "../generated"
package = false
[typescript]
enabled = true
dist = "client"
package = true
[typescript.naming]
source_file = "PascalCase"
source_dir = "kebab-case"
[rust]
enabled = true
[python]
enabled = true
package = true
module = "models"
"#,
    )
    .unwrap();
    let source =
        Url::from_file_path(root.join("types/schemas/shop_goods/order_item.type")).unwrap();
    std::fs::write(source.to_file_path().unwrap(), "Old: string").unwrap();
    let open = HashMap::from([(source.clone(), "/// 😀\nOrder: string".to_string())]);
    let results = generated_symbols(
        &[root.to_path_buf()],
        &open,
        GeneratedSymbolsParams {
            uri: source.clone(),
            position: Some(Position::new(1, 1)),
        },
    )
    .unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Order");
    assert_eq!(results[0].source.range.start, Position::new(1, 0));
    let paths: Vec<_> = results[0]
        .targets
        .iter()
        .map(|target| {
            target
                .uri
                .to_file_path()
                .unwrap()
                .strip_prefix(root)
                .unwrap()
                .to_path_buf()
        })
        .collect();
    assert_eq!(
        paths,
        [
            "generated/client/src/shop-goods/OrderItem.ts",
            "generated/rs/shop_goods/order_item.rs",
            "generated/py/models/shop_goods/order_item.py"
        ]
        .map(std::path::PathBuf::from)
    );
    for target in &results[0].targets {
        let reverse = generated_symbols(
            &[root.to_path_buf()],
            &open,
            GeneratedSymbolsParams {
                uri: target.uri.clone(),
                position: None,
            },
        )
        .unwrap();
        assert_eq!(reverse.len(), 1);
        assert_eq!(reverse[0].source.uri, source);
        assert_eq!(reverse[0].targets.len(), 3);
    }
}

#[test]
fn generated_symbols_exclude_disabled_targets_aliases_and_generics() {
    use crate::project::generated::{GeneratedSymbolsParams, generated_symbols};
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::create_dir(root.join("src")).unwrap();
    std::fs::write(
        root.join("genotype.toml"),
        "[ts]\nenabled = true\n[rs]\nenabled = false",
    )
    .unwrap();
    std::fs::write(root.join("src/model.type"), "User: string").unwrap();
    let text = "use ./model/{User as Person}\nBox<T>: { user: Person, value: T }\nOriginal: User\n";
    let uri = Url::from_file_path(root.join("src/main.type")).unwrap();
    let open = HashMap::from([(uri.clone(), text.to_string())]);
    for needle in ["Person,", "T }"] {
        let results = generated_symbols(
            &[root.to_path_buf()],
            &open,
            GeneratedSymbolsParams {
                uri: uri.clone(),
                position: Some(at(text, needle)),
            },
        )
        .unwrap();
        assert!(results.is_empty());
    }
    let results = generated_symbols(
        &[root.to_path_buf()],
        &open,
        GeneratedSymbolsParams {
            uri: uri.clone(),
            position: Some(at(text, "Box<")),
        },
    )
    .unwrap();
    assert_eq!(results[0].targets.len(), 1);
    assert_eq!(results[0].targets[0].language, "typescript");
    let unrelated = Url::from_file_path(root.join("app.ts")).unwrap();
    assert!(
        generated_symbols(
            &[root.to_path_buf()],
            &open,
            GeneratedSymbolsParams {
                uri: unrelated,
                position: None
            }
        )
        .unwrap()
        .is_empty()
    );
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

#[test]
fn imports_are_private_bindings_not_exports() {
    for import in ["use ./b/User", "use ./b/*", "use ./b/{User as User}"] {
        let p = project(&[
            ("a", import),
            ("b", "User: string"),
            ("inline", "Value: ./a/User"),
            ("named", "use ./a/User\nValue: User"),
            ("glob", "use ./a/*\nValue: User"),
        ]);
        assert!(
            p.diagnostics(&uri("a")).is_empty(),
            "{import}: {:?}",
            p.diagnostics(&uri("a"))
        );
        for module in ["inline", "named", "glob"] {
            assert!(
                !p.diagnostics(&uri(module)).is_empty(),
                "{import}: {module}"
            );
            let text = &p.documents[&uri(module)].text;
            assert!(
                p.definition(&uri(module), position(text, text.len() - 1))
                    .is_none()
            );
        }
    }
}

#[test]
fn rename_rejects_glob_conflicts_in_modules_without_edits() {
    let p = project(&[
        ("a", "User: string"),
        ("b", "use ./a/*\nuse ./c/*\nValue: Account"),
        ("c", "Account: string"),
    ]);
    assert!(p.rename(&uri("a"), Position::new(0, 0), "Account").is_err());
    assert!(p.rename(&uri("a"), Position::new(0, 0), "Customer").is_ok());
}

#[test]
fn imports_are_inserted_after_complete_multiline_comments() {
    for comment in [
        "/* keep\nthis comment */",
        "/* keep\n/* nested */ comment */",
        "// keep",
    ] {
        let text = format!("use ./existing/Existing {comment}\nValue: Missing");
        let p = project(&[
            ("main", &text),
            ("existing", "Existing: string"),
            ("model", "Missing: string"),
        ]);
        let pos = at(&text, "Missing");
        let actions = p.import_actions(&uri("main"), lsp::Range::new(pos, pos));
        assert!(
            !actions.is_empty(),
            "{comment}: {:?}",
            p.diagnostics(&uri("main"))
        );
        let lsp::CodeActionOrCommand::CodeAction(action) = &actions[0] else {
            panic!()
        };
        let updated = apply_edits(
            &text,
            &action.edit.as_ref().unwrap().changes.as_ref().unwrap()[&uri("main")],
        );
        let expected =
            format!("use ./existing/Existing {comment}\nuse ./model/Missing\nValue: Missing");
        assert_eq!(updated, expected);
        let items = p.completions(&uri("main"), position(&text, text.len()));
        let item = items.iter().find(|item| item.label == "Missing").unwrap();
        assert_eq!(apply_edits(&text, &completion_edits(item)), expected);
        let updated = project(&[
            ("main", &updated),
            ("existing", "Existing: string"),
            ("model", "Missing: string"),
        ]);
        assert!(updated.diagnostics(&uri("main")).is_empty());
    }
}

#[test]
fn configured_index_excludes_unrelated_files_but_follows_imports_and_buffers() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::create_dir_all(root.join("project/types")).unwrap();
    std::fs::create_dir(root.join("examples")).unwrap();
    std::fs::write(
        root.join("genotype.toml"),
        "root = 'project'\nsrc = 'types'",
    )
    .unwrap();
    std::fs::write(root.join("examples/broken.type"), "Broken: {").unwrap();
    std::fs::write(root.join("external.type"), "External: string").unwrap();
    let source = Url::from_file_path(root.join("project/types/main.type")).unwrap();
    std::fs::write(
        source.to_file_path().unwrap(),
        "use ../../external/External\nUser: External",
    )
    .unwrap();
    let p = Project::load(&[root.into()], &HashMap::new());
    assert_eq!(p.documents.len(), 2);
    assert!(p.diagnostics(&source).is_empty());
    assert!(p.rename(&source, Position::new(1, 0), "Account").is_ok());
    let extra = Url::from_file_path(root.join("examples/open.type")).unwrap();
    let open = HashMap::from([(extra.clone(), "Extra: string".into())]);
    assert!(
        Project::load(&[root.into()], &open)
            .documents
            .contains_key(&extra)
    );
}

#[test]
fn incremental_updates_preserve_other_documents_and_follow_new_imports() {
    let dir = tempfile::tempdir().unwrap();
    let main = Url::from_file_path(dir.path().join("main.type")).unwrap();
    let model = Url::from_file_path(dir.path().join("model.type")).unwrap();
    std::fs::write(main.to_file_path().unwrap(), "Value: string").unwrap();
    std::fs::write(model.to_file_path().unwrap(), "User: string").unwrap();
    let mut p = Project::load(&[dir.path().into()], &HashMap::new());
    std::fs::write(model.to_file_path().unwrap(), "Account: string").unwrap();
    let open = HashMap::from([(main.clone(), "Value: ./model/User".into())]);
    p.update(&main, &open);
    assert!(p.diagnostics(&main).is_empty());
    p.update(&model, &open);
    assert!(!p.diagnostics(&main).is_empty());
    std::fs::remove_file(model.to_file_path().unwrap()).unwrap();
    p.update(&model, &open);
    assert!(!p.documents.contains_key(&model));
    p.update(&main, &HashMap::new());
    assert_eq!(p.documents[&main].text, "Value: string");
}

#[test]
fn cached_index_follows_new_external_imports_and_drops_detached_documents() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("project");
    std::fs::create_dir(&root).unwrap();
    let main = Url::from_file_path(root.join("main.type")).unwrap();
    let external = Url::from_file_path(dir.path().join("external.type")).unwrap();
    std::fs::write(main.to_file_path().unwrap(), "Value: string").unwrap();
    let mut p = Project::load(std::slice::from_ref(&root), &HashMap::new());
    let mut open = HashMap::from([(main.clone(), "Value: ../external/User".into())]);
    p.update(&main, &open);
    assert!(p.references_module(&external));
    std::fs::write(external.to_file_path().unwrap(), "User: string").unwrap();
    p.update(&external, &open);
    assert!(p.diagnostics(&main).is_empty());
    open.insert(main.clone(), "Value: string".into());
    p.update(&main, &open);
    p.prune(&[root], &open);
    assert!(!p.documents.contains_key(&external));
}

#[test]
fn semantic_token_cursor_preserves_utf16_crlf_and_multiline_positions() {
    let mut text = String::new();
    for index in 0..1000 {
        text.push_str(&format!(
            "/* 😀 é\r\n\r\nend */ Type{index}: {{ field: \"😀\", other: string }}\r\n"
        ));
    }
    let doc = Document::new(text);
    assert!(doc.valid);
    let mut expected = vec![];
    let mut spans = doc.highlights.clone();
    spans.sort_by_key(|(span, _, _)| span.start);
    for (span, kind, modifiers) in spans {
        let mut byte = span.start;
        for line in doc.text[span].split_inclusive('\n') {
            let value = line.trim_end_matches(['\r', '\n']);
            if !value.is_empty() {
                expected.push((
                    position(&doc.text, byte),
                    value.encode_utf16().count() as u32,
                    kind,
                    modifiers,
                ));
            }
            byte += line.len();
        }
    }
    let mut pos = Position::default();
    let actual: Vec<_> = doc
        .tokens()
        .iter()
        .map(|token| {
            pos = Position::new(
                pos.line + token.delta_line,
                if token.delta_line == 0 {
                    pos.character + token.delta_start
                } else {
                    token.delta_start
                },
            );
            (
                pos,
                token.length,
                token.token_type,
                token.token_modifiers_bitset,
            )
        })
        .collect();
    assert_eq!(actual, expected);
}

#[test]
fn configured_entries_seed_both_initial_and_incremental_graphs() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("project[1]");
    std::fs::create_dir_all(root.join("src/fixtures")).unwrap();
    std::fs::write(root.join("genotype.toml"), "entry = 'main.type'").unwrap();
    let main = Url::from_file_path(root.join("src/main.type")).unwrap();
    let dep = Url::from_file_path(root.join("src/dependency.type")).unwrap();
    let broken = Url::from_file_path(root.join("src/fixtures/broken.type")).unwrap();
    std::fs::write(main.to_file_path().unwrap(), "Value: ./dependency/User").unwrap();
    std::fs::write(dep.to_file_path().unwrap(), "User: string").unwrap();
    std::fs::write(broken.to_file_path().unwrap(), "Broken: {").unwrap();
    let mut p = Project::load(std::slice::from_ref(&root), &HashMap::new());
    assert_eq!(p.documents.len(), 2);
    assert!(p.rename(&dep, Position::default(), "Account").is_ok());
    assert!(crate::project::in_source(
        &root,
        &main.to_file_path().unwrap()
    ));
    assert!(!crate::project::in_source(
        &root,
        &broken.to_file_path().unwrap()
    ));
    let open = HashMap::from([(broken.clone(), "Extra: string".into())]);
    p.update(&broken, &open);
    p.prune(std::slice::from_ref(&root), &open);
    assert!(p.documents.contains_key(&broken));
    p.update(&broken, &HashMap::new());
    p.prune(std::slice::from_ref(&root), &HashMap::new());
    assert!(!p.documents.contains_key(&broken));
    std::fs::write(main.to_file_path().unwrap(), "Value: string").unwrap();
    p.update(&main, &HashMap::new());
    p.prune(std::slice::from_ref(&root), &HashMap::new());
    assert_eq!(p.documents.len(), 1);
    std::fs::write(root.join("genotype.toml"), "entry = 'fixtures/*.type'").unwrap();
    assert!(crate::project::in_source(
        &root,
        &broken.to_file_path().unwrap()
    ));
    assert!(!crate::project::in_source(
        &root,
        &main.to_file_path().unwrap()
    ));
    assert_eq!(Project::load(&[root], &HashMap::new()).documents.len(), 1);
}

#[test]
fn generated_mapping_cache_tracks_notifications_and_open_buffers() {
    use crate::project::generated::{Cache, GeneratedSymbolsParams};
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    let roots = [root.to_path_buf()];
    let mut cache = Cache::default();
    let target = Url::from_file_path(root.join("out/ts/src/main.ts")).unwrap();
    let query = || GeneratedSymbolsParams {
        uri: target.clone(),
        position: None,
    };
    assert!(
        cache
            .query(&roots, &HashMap::new(), query())
            .unwrap()
            .is_empty()
    );
    std::fs::create_dir_all(root.join("nested/src")).unwrap();
    let config = Url::from_file_path(root.join("nested/genotype.toml")).unwrap();
    let source = Url::from_file_path(root.join("nested/src/main.type")).unwrap();
    std::fs::write(
        config.to_file_path().unwrap(),
        "dist = '../out'\nentry = 'main.type'\n[ts]\nenabled = true",
    )
    .unwrap();
    std::fs::write(source.to_file_path().unwrap(), "User: string").unwrap();
    // Negative queries do not rediscover every configuration in the workspace.
    assert!(
        cache
            .query(&roots, &HashMap::new(), query())
            .unwrap()
            .is_empty()
    );
    cache.invalidate(&config, &HashMap::new());
    assert_eq!(
        cache.query(&roots, &HashMap::new(), query()).unwrap()[0].name,
        "User"
    );
    std::fs::write(source.to_file_path().unwrap(), "Account: string").unwrap();
    assert_eq!(
        cache.query(&roots, &HashMap::new(), query()).unwrap()[0].name,
        "User"
    );
    cache.invalidate(&source, &HashMap::new());
    assert_eq!(
        cache.query(&roots, &HashMap::new(), query()).unwrap()[0].name,
        "Account"
    );
    let open = HashMap::from([(source.clone(), "Unsaved: string".into())]);
    cache.invalidate(&source, &open);
    assert_eq!(
        cache.query(&roots, &open, query()).unwrap()[0].name,
        "Unsaved"
    );
    let forward = cache
        .query(
            &roots,
            &open,
            GeneratedSymbolsParams {
                uri: source.clone(),
                position: Some(Position::default()),
            },
        )
        .unwrap();
    assert_eq!(forward[0].targets[0].uri, target);
    cache.invalidate(&source, &HashMap::new());
    assert_eq!(
        cache.query(&roots, &HashMap::new(), query()).unwrap()[0].name,
        "Account"
    );
    std::fs::remove_file(config.to_file_path().unwrap()).unwrap();
    cache.invalidate(&config, &HashMap::new());
    assert!(
        cache
            .query(&roots, &HashMap::new(), query())
            .unwrap()
            .is_empty()
    );
}

#[test]
fn external_dependencies_are_revalidated_without_file_events() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("workspace");
    std::fs::create_dir(&root).unwrap();
    let main = Url::from_file_path(root.join("main.type")).unwrap();
    let external = Url::from_file_path(dir.path().join("external.type")).unwrap();
    std::fs::write(main.to_file_path().unwrap(), "Value: ../external/User").unwrap();
    let roots = [root];
    let mut p = Project::load(&roots, &HashMap::new());
    std::fs::write(external.to_file_path().unwrap(), "User: string").unwrap();
    assert!(p.refresh_external(&roots, &HashMap::new()));
    assert!(p.diagnostics(&main).is_empty());
    std::fs::write(external.to_file_path().unwrap(), "Other: string").unwrap();
    assert!(p.refresh_external(&roots, &HashMap::new()));
    assert!(!p.diagnostics(&main).is_empty());
    let open = HashMap::from([(external.clone(), "User: string".into())]);
    p.update(&external, &open);
    assert!(!p.refresh_external(&roots, &open));
    assert!(p.diagnostics(&main).is_empty());
    std::fs::remove_file(external.to_file_path().unwrap()).unwrap();
    assert!(p.refresh_external(&roots, &HashMap::new()));
    assert!(!p.documents.contains_key(&external));
}

#[test]
fn standalone_generated_mappings_refresh_sources_and_configuration() {
    use crate::project::generated::{Cache, GeneratedSymbolsParams};
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::create_dir(root.join("src")).unwrap();
    let source = Url::from_file_path(root.join("src/main.type")).unwrap();
    let config = root.join("genotype.toml");
    std::fs::write(&config, "[ts]\nenabled = true").unwrap();
    std::fs::write(source.to_file_path().unwrap(), "User: string").unwrap();
    let mut cache = Cache::default();
    let query = || GeneratedSymbolsParams {
        uri: source.clone(),
        position: Some(Position::default()),
    };
    let first = cache.query(&[], &HashMap::new(), query()).unwrap();
    assert_eq!(first[0].name, "User");
    std::fs::write(source.to_file_path().unwrap(), "Account: string").unwrap();
    let reverse = cache
        .query(
            &[],
            &HashMap::new(),
            GeneratedSymbolsParams {
                uri: first[0].targets[0].uri.clone(),
                position: None,
            },
        )
        .unwrap();
    assert_eq!(reverse[0].name, "Account");
    std::fs::write(&config, "[ts]\nenabled = false\n[rs]\nenabled = true").unwrap();
    let updated = cache.query(&[], &HashMap::new(), query()).unwrap();
    assert_eq!(updated[0].targets.len(), 1);
    assert_eq!(updated[0].targets[0].language, "rust");
}
