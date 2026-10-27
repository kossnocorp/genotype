use super::*;
use crate::document::{Entity, position};
use async_lsp::lsp_types::{
    CodeAction, CodeActionKind, CodeActionOrCommand, CompletionItem, CompletionItemKind,
    CompletionItemLabelDetails, CompletionTextEdit, Documentation, Hover, HoverContents,
    MarkupContent, MarkupKind,
};

const BUILTINS: &[&str] = &[
    "any", "string", "boolean", "number", "int", "float", "i8", "i16", "i32", "i64", "i128",
    "isize", "u8", "u16", "u32", "u64", "u128", "usize", "f32", "f64", "null", "true", "false",
];

impl Project {
    pub fn hover(&self, uri: &Url, pos: Position) -> Option<Hover> {
        let doc = self.documents.get(uri)?;
        let span = word_span(&doc.text, offset(&doc.text, pos)?);
        let contents = if let Some(entity) = doc.entities.get(&span.start) {
            entity_markdown(entity)
        } else {
            let location = self.definition(uri, position(&doc.text, span.start))?;
            let target = self.documents.get(&location.uri)?;
            let start = offset(&target.text, location.range.start)?;
            if let Some(entity) = target.entities.get(&start) {
                entity_markdown(entity)
            } else {
                let generic = target
                    .generics
                    .iter()
                    .find(|name| name.span.start == start)?;
                markdown(format!(
                    "```genotype\n{} (type parameter)\n```",
                    generic.text
                ))
            }
        };
        Some(Hover {
            contents: HoverContents::Markup(contents),
            range: Some(doc.range(&span)),
        })
    }

    pub fn completions(&self, uri: &Url, pos: Position) -> Vec<CompletionItem> {
        self.clone().completion_snapshot(uri, pos)
    }

    fn completion_snapshot(mut self, uri: &Url, pos: Position) -> Vec<CompletionItem> {
        let Some(original) = self.documents.get(uri) else {
            return vec![];
        };
        let Some(byte) = offset(&original.text, pos) else {
            return vec![];
        };
        let span = word_span(&original.text, byte);
        let prefix = original.text[span.start..byte].to_owned();
        let range = original.range(&span);
        let source = original.text.clone();

        // Completion requests commonly arrive with an empty descriptor or an
        // unclosed object. Repair only the temporary completion snapshot. The
        // actual buffer and its diagnostics retain the original syntax.
        let mut repaired = source.clone();
        repaired.replace_range(span.clone(), &"Z".repeat(span.len().max(1)));
        close_delimiters(&mut repaired);
        let repaired = Document::new(repaired);
        if !repaired.valid {
            return vec![];
        }
        let original = self.documents.insert(uri.clone(), repaired).unwrap();
        let doc = &self.documents[uri];
        let import_end = if original.valid {
            original.import_end
        } else if doc.import_end > span.end {
            doc.import_end.saturating_sub(usize::from(span.is_empty()))
        } else {
            doc.import_end
        };
        let reference = doc
            .references
            .iter()
            .find(|reference| reference.name.span.start == span.start);
        let import = doc.imports.iter().find(|import| {
            import
                .name
                .as_ref()
                .is_some_and(|name| name.span.start == span.start)
        });
        let Some(path) = reference
            .map(|r| r.path.as_deref())
            .or_else(|| import.map(|i| Some(i.path.as_str())))
        else {
            // Declarations, property names, comments and literals aren't type positions.
            return vec![];
        };
        let mut items = vec![];
        if let Some(path) = path {
            if let Some(target) = module_uri(uri, path)
                && let Some(target_doc) = self.documents.get(&target)
            {
                for name in &target_doc.definitions {
                    if name.text.starts_with(&prefix) {
                        items.push(completion(
                            &name.text,
                            range,
                            target_doc.entities.get(&name.span.start),
                            None,
                        ));
                    }
                }
            }
            return items;
        }

        let mut bound = HashSet::new();
        for (scope, index) in doc.generic_scopes.iter().rev() {
            let name = &doc.generics[*index];
            if scope.contains(&span.start)
                && bound.insert(name.text.clone())
                && name.text.starts_with(&prefix)
            {
                let mut item = completion(&name.text, range, None, None);
                item.kind = Some(CompletionItemKind::TYPE_PARAMETER);
                items.push(item);
            }
        }
        for name in doc.definitions.iter().map(|n| &n.text).chain(
            doc.imports
                .iter()
                .filter_map(|i| i.alias.as_ref().or(i.name.as_ref()).map(|n| &n.text)),
        ) {
            if !bound.insert(name.clone()) || !name.starts_with(&prefix) {
                continue;
            }
            let entity = self
                .resolve(uri, name, &mut HashSet::new())
                .and_then(|symbol| {
                    if &symbol.0 == uri {
                        original
                            .entities
                            .get(&symbol.1)
                            .or_else(|| self.entity(&symbol))
                    } else {
                        self.entity(&symbol)
                    }
                });
            items.push(completion(name, range, entity, None));
        }
        let mut exported = self
            .documents
            .iter()
            .flat_map(|(uri, doc)| doc.definitions.iter().map(move |name| (uri, doc, name)))
            .collect::<Vec<_>>();
        exported.sort_by_key(|(uri, _, name)| (name.text.as_str(), uri.as_str()));
        for (target, target_doc, name) in exported {
            if target == uri || bound.contains(&name.text) || !name.text.starts_with(&prefix) {
                continue;
            }
            let symbol = Symbol(target.clone(), name.span.start);
            let resolved = self.resolve(uri, &name.text, &mut HashSet::new());
            if resolved.as_ref() == Some(&symbol) {
                items.push(completion(
                    &name.text,
                    range,
                    target_doc.entities.get(&name.span.start),
                    None,
                ));
                bound.insert(name.text.clone());
            } else if resolved.is_none()
                && let Some(path) = import_path(uri, target)
            {
                let mut item = completion(
                    &name.text,
                    range,
                    target_doc.entities.get(&name.span.start),
                    Some(&path),
                );
                item.additional_text_edits =
                    Some(vec![import_edit(&source, import_end, &path, &name.text)]);
                items.push(item);
            }
        }
        for name in BUILTINS {
            if name.starts_with(&prefix) && !bound.contains(*name) {
                let mut item = completion(name, range, None, None);
                item.kind = Some(CompletionItemKind::KEYWORD);
                items.push(item);
            }
        }
        items.sort_by(|a, b| a.sort_text.cmp(&b.sort_text));
        items
    }

    fn entity(&self, symbol: &Symbol) -> Option<&Entity> {
        let doc = self.documents.get(&symbol.0)?;
        if let Some(entity) = doc.entities.get(&symbol.1) {
            return Some(entity);
        }
        // Import aliases display the original declaration's documentation.
        let location = self.definition(&symbol.0, position(&doc.text, symbol.1))?;
        let target = self.documents.get(&location.uri)?;
        target
            .entities
            .get(&offset(&target.text, location.range.start)?)
    }

    pub fn import_actions(&self, uri: &Url, range: Range) -> Vec<CodeActionOrCommand> {
        let Some(doc) = self.documents.get(uri) else {
            return vec![];
        };
        let mut actions = vec![];
        for diagnostic in self.diagnostics(uri) {
            if diagnostic.code
                != Some(async_lsp::lsp_types::NumberOrString::String(
                    "reference-not-found".into(),
                ))
                || diagnostic.range.end < range.start
                || diagnostic.range.start > range.end
            {
                continue;
            }
            let Some(reference) = doc
                .references
                .iter()
                .find(|r| r.path.is_none() && doc.range(&r.name.span) == diagnostic.range)
            else {
                continue;
            };
            let name = &reference.name.text;
            // Don't add another binding over an existing (possibly broken) import.
            if doc.imports.iter().any(|i| {
                i.alias
                    .as_ref()
                    .or(i.name.as_ref())
                    .is_some_and(|n| n.text == *name)
            }) {
                continue;
            }
            for (target, target_doc) in &self.documents {
                if target == uri || !target_doc.definitions.iter().any(|n| n.text == *name) {
                    continue;
                }
                let Some(path) = import_path(uri, target) else {
                    continue;
                };
                actions.push(CodeActionOrCommand::CodeAction(CodeAction {
                    title: format!("Import '{name}' from '{path}'"),
                    kind: Some(CodeActionKind::QUICKFIX),
                    diagnostics: Some(vec![diagnostic.clone()]),
                    edit: Some(WorkspaceEdit {
                        changes: Some(HashMap::from([(
                            uri.clone(),
                            vec![import_edit(&doc.text, doc.import_end, &path, name)],
                        )])),
                        ..Default::default()
                    }),
                    ..Default::default()
                }));
            }
        }
        actions.sort_by_key(|action| match action {
            CodeActionOrCommand::CodeAction(action) => action.title.clone(),
            _ => String::new(),
        });
        actions
    }
}

fn markdown(value: String) -> MarkupContent {
    MarkupContent {
        kind: MarkupKind::Markdown,
        value,
    }
}

fn entity_markdown(entity: &Entity) -> MarkupContent {
    // A longer fence keeps literal backticks in a type's string values intact.
    let fence = "`".repeat(
        entity
            .signature
            .split(|ch| ch != '`')
            .map(str::len)
            .max()
            .unwrap_or(0)
            .max(2)
            + 1,
    );
    let signature = format!("{fence}genotype\n{}\n{fence}", entity.signature);
    markdown(if entity.documentation.is_empty() {
        signature
    } else {
        format!("{signature}\n\n---\n\n{}", entity.documentation)
    })
}

fn completion(
    name: &str,
    range: Range,
    entity: Option<&Entity>,
    import: Option<&str>,
) -> CompletionItem {
    CompletionItem {
        label: name.into(),
        kind: Some(CompletionItemKind::CLASS),
        detail: Some(
            import
                .map(|path| format!("Auto import from {path}"))
                .unwrap_or_else(|| {
                    entity
                        .map(|e| e.signature.clone())
                        .unwrap_or_else(|| name.into())
                }),
        ),
        label_details: import.map(|path| CompletionItemLabelDetails {
            description: Some(path.into()),
            detail: None,
        }),
        documentation: entity.map(|e| Documentation::MarkupContent(entity_markdown(e))),
        sort_text: Some(format!(
            "{}:{name}:{}",
            u8::from(import.is_some()),
            import.unwrap_or("")
        )),
        text_edit: Some(CompletionTextEdit::Edit(TextEdit::new(range, name.into()))),
        ..Default::default()
    }
}

fn word_span(text: &str, byte: usize) -> std::ops::Range<usize> {
    let is_name = |ch: u8| ch.is_ascii_alphanumeric() || ch == b'_';
    let mut start = byte;
    let mut end = byte;
    while start > 0 && is_name(text.as_bytes()[start - 1]) {
        start -= 1;
    }
    while end < text.len() && is_name(text.as_bytes()[end]) {
        end += 1;
    }
    start..end
}

fn import_path(from: &Url, target: &Url) -> Option<String> {
    let from = from.to_file_path().ok()?;
    let target_path = target.to_file_path().ok()?;
    let from: Vec<_> = from.parent()?.components().collect();
    let to: Vec<_> = target_path.components().collect();
    let common = from.iter().zip(&to).take_while(|(a, b)| a == b).count();
    if common == 0 {
        return None;
    }
    let mut path = "../".repeat(from.len() - common);
    if path.is_empty() {
        path.push_str("./");
    }
    path.push_str(
        &to[common..]
            .iter()
            .map(|part| part.as_os_str().to_str())
            .collect::<Option<Vec<_>>>()?
            .join("/"),
    );
    let path = path.strip_suffix(".type")?.to_string();
    Some(path)
}

fn import_edit(text: &str, import_end: usize, path: &str, name: &str) -> TextEdit {
    let newline = if text.contains("\r\n") { "\r\n" } else { "\n" };
    let mut byte = import_end.min(text.len());
    if byte > 0 {
        // Skip complete trailing comments, never a newline inside a block comment.
        let mut depth = 0;
        while byte < text.len() {
            let rest = &text[byte..];
            if rest.starts_with("/*") {
                depth += 1;
                byte += 2;
            } else if depth > 0 && rest.starts_with("*/") {
                depth -= 1;
                byte += 2;
            } else if depth == 0 && rest.starts_with("//") {
                byte += rest.find('\n').unwrap_or(rest.len());
            } else {
                let ch = rest.chars().next().unwrap();
                if depth == 0 && !ch.is_whitespace() {
                    break;
                }
                byte += ch.len_utf8();
                if depth == 0 && ch == '\n' {
                    break;
                }
            }
        }
    }
    let prefix = if byte > 0 && !text[..byte].ends_with('\n') {
        newline
    } else {
        ""
    };
    let pos = position(text, byte);
    TextEdit::new(
        Range::new(pos, pos),
        format!("{prefix}use {path}/{name}{newline}"),
    )
}

fn close_delimiters(text: &mut String) {
    let mut stack = vec![];
    let mut chars = text.chars().peekable();
    let mut string = false;
    let mut comment_depth = 0;
    while let Some(ch) = chars.next() {
        if comment_depth > 0 {
            if ch == '/' && chars.peek() == Some(&'*') {
                chars.next();
                comment_depth += 1;
            }
            if ch == '*' && chars.peek() == Some(&'/') {
                chars.next();
                comment_depth -= 1;
            }
        } else if string {
            if ch == '\\' {
                chars.next();
            } else if ch == '"' {
                string = false;
            }
        } else if ch == '/' && chars.peek() == Some(&'/') {
            for ch in chars.by_ref() {
                if ch == '\n' {
                    break;
                }
            }
        } else if ch == '/' && chars.peek() == Some(&'*') {
            chars.next();
            comment_depth = 1;
        } else {
            match ch {
                '"' => string = true,
                '{' => stack.push('}'),
                '[' => stack.push(']'),
                '(' => stack.push(')'),
                '<' => stack.push('>'),
                '}' | ']' | ')' | '>' if stack.last() == Some(&ch) => {
                    stack.pop();
                }
                _ => {}
            }
        }
    }
    text.extend(stack.into_iter().rev());
}
