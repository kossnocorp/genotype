use serde_json::{Value, json};
use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Child, ChildStdout, Command, Stdio};

struct Client {
    process: Child,
    output: BufReader<ChildStdout>,
    id: u32,
    diagnostics: std::collections::HashMap<String, Value>,
}

impl Client {
    fn new() -> Self {
        let mut process = Command::new(env!("CARGO_BIN_EXE_gt"))
            .arg("lsp")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap();
        let output = BufReader::new(process.stdout.take().unwrap());
        Self {
            process,
            output,
            id: 0,
            diagnostics: Default::default(),
        }
    }

    fn send(&mut self, message: Value) {
        let body = serde_json::to_vec(&message).unwrap();
        let input = self.process.stdin.as_mut().unwrap();
        write!(input, "Content-Length: {}\r\n\r\n", body.len()).unwrap();
        input.write_all(&body).unwrap();
        input.flush().unwrap();
    }

    fn notify(&mut self, method: &str, params: Value) {
        self.send(json!({ "jsonrpc": "2.0", "method": method, "params": params }));
    }

    fn request(&mut self, method: &str, params: Value) -> Value {
        self.id += 1;
        self.send(json!({ "jsonrpc": "2.0", "id": self.id, "method": method, "params": params }));
        loop {
            let response = self.receive();
            if response["method"] == "textDocument/publishDiagnostics" {
                let params = response["params"].clone();
                self.diagnostics
                    .insert(params["uri"].as_str().unwrap().into(), params);
                continue;
            }
            assert_eq!(response["id"], self.id);
            assert!(response.get("error").is_none(), "{response}");
            return response["result"].clone();
        }
    }

    fn receive(&mut self) -> Value {
        let mut length = None;
        loop {
            let mut line = String::new();
            assert_ne!(
                self.output.read_line(&mut line).unwrap(),
                0,
                "server exited before responding"
            );
            if line == "\r\n" {
                break;
            }
            if let Some(value) = line.strip_prefix("Content-Length: ") {
                length = Some(value.trim().parse::<usize>().unwrap());
            }
        }
        let mut bytes = vec![0; length.unwrap()];
        self.output.read_exact(&mut bytes).unwrap();
        serde_json::from_slice(&bytes).unwrap()
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        let _ = self.process.kill();
        let _ = self.process.wait();
    }
}

#[test]
fn stdio_language_server_lifecycle_and_editor_features() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().to_str().unwrap().replace('\\', "/");
    let root = if root.starts_with('/') {
        format!("file://{root}")
    } else {
        format!("file:///{root}")
    };
    let main = format!("{root}/main.type");
    let model = format!("{root}/model.type");
    let text = "use ./model/User\nRef: User\n";
    std::fs::write(dir.path().join("main.type"), text).unwrap();
    std::fs::write(
        dir.path().join("genotype.toml"),
        "src = \".\"\n[ts]\nenabled = true\n",
    )
    .unwrap();
    std::fs::write(
        dir.path().join("model.type"),
        "/// A **user**.\nUser: string\n",
    )
    .unwrap();
    std::fs::write(
        dir.path().join("remote.type"),
        "/// Markdown documentation.\nJsDoc: string\nMissing: string\n",
    )
    .unwrap();
    let mut client = Client::new();
    let initialized = client.request(
        "initialize",
        json!({ "processId": null, "rootUri": root, "capabilities": {} }),
    );
    assert_eq!(
        initialized["capabilities"]["renameProvider"]["prepareProvider"],
        true
    );
    assert_eq!(
        initialized["capabilities"]["semanticTokensProvider"]["full"],
        true
    );
    assert_eq!(initialized["capabilities"]["hoverProvider"], true);
    assert!(initialized["capabilities"]["completionProvider"].is_object());
    assert_eq!(
        initialized["capabilities"]["codeActionProvider"]["codeActionKinds"],
        json!(["quickfix"])
    );
    client.notify("initialized", json!({}));
    client.notify("$/setTrace", json!({ "value": "off" }));
    client.notify("textDocument/didOpen", json!({ "textDocument": { "uri": main, "languageId": "genotype", "version": 1, "text": text } }));
    let position =
        json!({ "textDocument": { "uri": main }, "position": { "line": 1, "character": 5 } });
    let generated = client.request(
        "genotype/generatedSymbols",
        json!({ "uri": main, "position": { "line": 1, "character": 5 } }),
    );
    assert_eq!(generated[0]["name"], "User");
    assert_eq!(generated[0]["source"]["uri"], model);
    assert_eq!(
        generated[0]["targets"][0]["uri"],
        format!("{root}/dist/ts/src/model.ts")
    );
    assert_eq!(
        client.request(
            "genotype/generatedSymbols",
            json!({ "uri": format!("{root}/dist/ts/src/model.ts") })
        ),
        generated
    );
    assert_eq!(
        client.request("textDocument/definition", position.clone())["uri"],
        model
    );
    assert_eq!(
        client.request("textDocument/prepareRename", position.clone())["placeholder"],
        "User"
    );
    let hover = client.request("textDocument/hover", position.clone());
    assert_eq!(hover["contents"]["kind"], "markdown");
    assert!(
        hover["contents"]["value"]
            .as_str()
            .unwrap()
            .contains("A **user**.")
    );
    let completions = client.request("textDocument/completion", position.clone());
    let autoimport = completions
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["label"] == "JsDoc")
        .unwrap();
    assert_eq!(
        autoimport["additionalTextEdits"][0]["newText"],
        "use ./remote/JsDoc\n"
    );
    assert!(
        autoimport["documentation"]["value"]
            .as_str()
            .unwrap()
            .contains("Markdown documentation.")
    );
    let mut rename = position.clone();
    rename["newName"] = json!("Account");
    let edits = client.request("textDocument/rename", rename);
    assert_eq!(edits["changes"].as_object().unwrap().len(), 2);
    assert_eq!(edits["changes"][&main].as_array().unwrap().len(), 2);
    let tokens = client.request(
        "textDocument/semanticTokens/full",
        json!({ "textDocument": { "uri": main } }),
    );
    assert!(!tokens["data"].as_array().unwrap().is_empty());
    assert_eq!(client.diagnostics[&main]["diagnostics"], json!([]));
    client.notify("textDocument/didChange", json!({ "textDocument": { "uri": main, "version": 2 }, "contentChanges": [{ "text": "Ref: {" }] }));
    client.request("textDocument/definition", position.clone());
    assert_eq!(client.diagnostics[&main]["diagnostics"][0]["code"], "GT001");
    assert_eq!(client.diagnostics[&main]["version"], 2);
    client.notify("textDocument/didChange", json!({ "textDocument": { "uri": main, "version": 3 }, "contentChanges": [{ "text": "Ref: Missing" }] }));
    client.request("textDocument/definition", position.clone());
    assert_eq!(
        client.diagnostics[&main]["diagnostics"][0]["code"],
        "reference-not-found"
    );
    let actions = client.request("textDocument/codeAction", json!({
        "textDocument": { "uri": main },
        "range": { "start": { "line": 0, "character": 5 }, "end": { "line": 0, "character": 12 } },
        "context": { "diagnostics": client.diagnostics[&main]["diagnostics"], "only": ["quickfix"] }
    }));
    assert_eq!(actions[0]["title"], "Import 'Missing' from './remote'");
    assert_eq!(
        actions[0]["edit"]["changes"][&main][0]["newText"],
        "use ./remote/Missing\n"
    );
    client.notify("textDocument/didChange", json!({ "textDocument": { "uri": main, "version": 4 }, "contentChanges": [{ "text": text }] }));
    std::fs::write(dir.path().join("model.type"), "Account: string\n").unwrap();
    client.notify(
        "workspace/didChangeWatchedFiles",
        json!({ "changes": [{ "uri": model, "type": 2 }] }),
    );
    client.request("textDocument/definition", position.clone());
    assert!(
        !client.diagnostics[&main]["diagnostics"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    std::fs::write(dir.path().join("model.type"), "User: string\n").unwrap();
    client.notify(
        "workspace/didChangeWatchedFiles",
        json!({ "changes": [{ "uri": model, "type": 2 }] }),
    );
    client.request("textDocument/definition", position.clone());
    assert_eq!(client.diagnostics[&main]["diagnostics"], json!([]));
    client.notify("textDocument/didChange", json!({ "textDocument": { "uri": main, "version": 5 }, "contentChanges": [{ "text": "Local: string\nRef: Local\n" }] }));
    assert_eq!(
        client.request("textDocument/definition", position.clone())["uri"],
        main
    );
    client.notify("textDocument/didChange", json!({ "textDocument": { "uri": main, "version": 6 }, "contentChanges": [{ "text": "Ref: Missing" }] }));
    client.request("textDocument/definition", position.clone());
    assert!(
        !client.diagnostics[&main]["diagnostics"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    client.notify(
        "textDocument/didClose",
        json!({ "textDocument": { "uri": main } }),
    );
    assert_eq!(
        client.request("textDocument/definition", position)["uri"],
        model
    );
    assert!(client.request("shutdown", Value::Null).is_null());
    assert_eq!(client.diagnostics[&main]["diagnostics"], json!([]));
    client.notify("exit", Value::Null);
    assert!(client.process.wait().unwrap().success());
}
