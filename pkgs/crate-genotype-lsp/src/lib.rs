//! Genotype's stdio language server, hosted by `gt lsp`.
mod document;
mod project;

use async_lsp::client_monitor::ClientProcessMonitorLayer;
use async_lsp::concurrency::ConcurrencyLayer;
use async_lsp::lsp_types::{self as lsp};
use async_lsp::panic::CatchUnwindLayer;
use async_lsp::router::Router;
use async_lsp::server::LifecycleLayer;
use async_lsp::{ClientSocket, ResponseError};
use std::collections::{HashMap, HashSet};
use std::ops::ControlFlow;
use std::path::PathBuf;
use tower::ServiceBuilder;

struct Server {
    client: ClientSocket,
    roots: Vec<PathBuf>,
    open: HashMap<lsp::Url, String>,
    versions: HashMap<lsp::Url, i32>,
    published: HashSet<lsp::Url>,
}

impl Server {
    fn publish_diagnostics(&mut self) -> ControlFlow<async_lsp::Result<()>> {
        let mut diagnostics = HashMap::new();
        for uri in self.open.keys() {
            if diagnostics.contains_key(uri) {
                continue;
            }
            let project = self.project(uri);
            for uri in project.documents.keys() {
                diagnostics.insert(uri.clone(), project.diagnostics(uri));
            }
        }
        let current: HashSet<_> = diagnostics.keys().cloned().collect();
        for uri in self.published.difference(&current) {
            diagnostics.insert(uri.clone(), vec![]);
        }
        for (uri, diagnostics) in diagnostics {
            let version = self.versions.get(&uri).copied();
            if let Err(error) = self.client.notify::<lsp::notification::PublishDiagnostics>(
                lsp::PublishDiagnosticsParams {
                    uri,
                    diagnostics,
                    version,
                },
            ) {
                return ControlFlow::Break(Err(error));
            }
        }
        self.published = current;
        ControlFlow::Continue(())
    }

    fn project(&self, uri: &lsp::Url) -> project::Project {
        let mut roots = self.roots.clone();
        if let Ok(path) = uri.to_file_path() {
            let root = project::project_root(&path);
            if root.join("genotype.toml").is_file()
                || !roots.iter().any(|root| path.starts_with(root))
            {
                roots = vec![root];
            }
        }
        project::Project::load(&roots, &self.open)
    }

    fn router(client: ClientSocket) -> Router<Self> {
        let mut router = Router::new(Self {
            client,
            roots: vec![],
            open: HashMap::new(),
            versions: HashMap::new(),
            published: HashSet::new(),
        });
        // Optional client notifications must not terminate the server.
        router.unhandled_notification(|_, _| ControlFlow::Continue(()));
        router.request::<lsp::request::Shutdown, _>(|_, ()| async { Ok(()) });
        router.notification::<lsp::notification::Exit>(|_, ()| ControlFlow::Continue(()));
        router.request::<lsp::request::Initialize, _>(|server, params| {
            server.roots = params
                .workspace_folders
                .unwrap_or_default()
                .into_iter()
                .filter_map(|f| f.uri.to_file_path().ok())
                .collect();
            #[allow(deprecated)]
            if server.roots.is_empty() {
                server
                    .roots
                    .extend(params.root_uri.and_then(|uri| uri.to_file_path().ok()));
            }
            async move {
                Ok(lsp::InitializeResult {
                    capabilities: lsp::ServerCapabilities {
                        position_encoding: Some(lsp::PositionEncodingKind::UTF16),
                        text_document_sync: Some(lsp::TextDocumentSyncCapability::Kind(
                            lsp::TextDocumentSyncKind::FULL,
                        )),
                        definition_provider: Some(lsp::OneOf::Left(true)),
                        hover_provider: Some(lsp::HoverProviderCapability::Simple(true)),
                        completion_provider: Some(lsp::CompletionOptions {
                            trigger_characters: Some(vec![":".into(), "/".into(), "<".into()]),
                            ..Default::default()
                        }),
                        code_action_provider: Some(lsp::CodeActionProviderCapability::Options(
                            lsp::CodeActionOptions {
                                code_action_kinds: Some(vec![lsp::CodeActionKind::QUICKFIX]),
                                ..Default::default()
                            },
                        )),
                        rename_provider: Some(lsp::OneOf::Right(lsp::RenameOptions {
                            prepare_provider: Some(true),
                            work_done_progress_options: Default::default(),
                        })),
                        semantic_tokens_provider: Some(
                            lsp::SemanticTokensServerCapabilities::SemanticTokensOptions(
                                lsp::SemanticTokensOptions {
                                    legend: lsp::SemanticTokensLegend {
                                        token_types: document::TOKEN_TYPES.to_vec(),
                                        token_modifiers: vec![
                                            lsp::SemanticTokenModifier::DECLARATION,
                                        ],
                                    },
                                    full: Some(lsp::SemanticTokensFullOptions::Bool(true)),
                                    ..Default::default()
                                },
                            ),
                        ),
                        ..Default::default()
                    },
                    server_info: Some(lsp::ServerInfo {
                        name: "Genotype".into(),
                        version: Some(env!("CARGO_PKG_VERSION").into()),
                    }),
                })
            }
        });
        router.notification::<lsp::notification::Initialized>(|_, _| ControlFlow::Continue(()));
        router.notification::<lsp::notification::DidOpenTextDocument>(|server, params| {
            server.versions.insert(
                params.text_document.uri.clone(),
                params.text_document.version,
            );
            server
                .open
                .insert(params.text_document.uri, params.text_document.text);
            server.publish_diagnostics()
        });
        router.notification::<lsp::notification::DidChangeTextDocument>(|server, params| {
            if server
                .versions
                .get(&params.text_document.uri)
                .is_some_and(|version| *version >= params.text_document.version)
            {
                return ControlFlow::Continue(());
            }
            server.versions.insert(
                params.text_document.uri.clone(),
                params.text_document.version,
            );
            if let Some(text) = server.open.get_mut(&params.text_document.uri) {
                for change in params.content_changes {
                    if let Some(range) = change.range {
                        if let (Some(start), Some(end)) = (
                            document::offset(text, range.start),
                            document::offset(text, range.end),
                        ) && start <= end
                        {
                            text.replace_range(start..end, &change.text);
                        }
                    } else {
                        *text = change.text;
                    }
                }
            }
            server.publish_diagnostics()
        });
        router.notification::<lsp::notification::DidCloseTextDocument>(|server, params| {
            server.open.remove(&params.text_document.uri);
            server.versions.remove(&params.text_document.uri);
            server.publish_diagnostics()
        });
        // Requests read current files from disk, with unsaved buffers overlaid.
        router.notification::<lsp::notification::DidChangeWatchedFiles>(|server, _| {
            server.publish_diagnostics()
        });
        router.request::<lsp::request::GotoDefinition, _>(|server, params| {
            let params = params.text_document_position_params;
            let result = server
                .project(&params.text_document.uri)
                .definition(&params.text_document.uri, params.position)
                .map(lsp::GotoDefinitionResponse::Scalar);
            async move { Ok(result) }
        });
        router.request::<lsp::request::HoverRequest, _>(|server, params| {
            let params = params.text_document_position_params;
            let result = server
                .project(&params.text_document.uri)
                .hover(&params.text_document.uri, params.position);
            async move { Ok(result) }
        });
        router.request::<lsp::request::Completion, _>(|server, params| {
            let params = params.text_document_position;
            let items = server
                .project(&params.text_document.uri)
                .completions(&params.text_document.uri, params.position);
            async move { Ok(Some(lsp::CompletionResponse::Array(items))) }
        });
        router.request::<lsp::request::CodeActionRequest, _>(|server, params| {
            let requested = params.context.only.as_ref().is_none_or(|kinds| {
                kinds
                    .iter()
                    .any(|kind| kind.as_str().is_empty() || kind == &lsp::CodeActionKind::QUICKFIX)
            });
            let actions = if requested {
                server
                    .project(&params.text_document.uri)
                    .import_actions(&params.text_document.uri, params.range)
            } else {
                vec![]
            };
            async move { Ok(Some(actions)) }
        });
        router.request::<lsp::request::PrepareRenameRequest, _>(|server, params| {
            let result = server
                .project(&params.text_document.uri)
                .prepare(&params.text_document.uri, params.position)
                .map(
                    |(range, placeholder)| lsp::PrepareRenameResponse::RangeWithPlaceholder {
                        range,
                        placeholder,
                    },
                );
            async move { Ok(result) }
        });
        router.request::<lsp::request::Rename, _>(|server, params| {
            let position = params.text_document_position;
            let result = server
                .project(&position.text_document.uri)
                .rename(
                    &position.text_document.uri,
                    position.position,
                    &params.new_name,
                )
                .map_err(|message| {
                    ResponseError::new(async_lsp::ErrorCode::INVALID_PARAMS, message)
                });
            async move { result }
        });
        router.request::<project::generated::GeneratedSymbols, _>(|server, params| {
            let result = project::generated::generated_symbols(&server.roots, &server.open, params)
                .map_err(|message| {
                    ResponseError::new(async_lsp::ErrorCode::INVALID_PARAMS, message)
                });
            async move { result }
        });
        router.request::<lsp::request::SemanticTokensFullRequest, _>(|server, params| {
            let uri = params.text_document.uri;
            let text = server
                .open
                .get(&uri)
                .cloned()
                .or_else(|| std::fs::read_to_string(uri.to_file_path().ok()?).ok());
            let result = text.map(|text| {
                lsp::SemanticTokensResult::Tokens(lsp::SemanticTokens {
                    result_id: None,
                    data: document::Document::new(text).tokens(),
                })
            });
            async move { Ok(result) }
        });
        router
    }
}

/// Runs the language server until the client exits. Stdout is reserved for LSP.
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (server, _) = async_lsp::MainLoop::new_server(|client: ClientSocket| {
        ServiceBuilder::new()
            .layer(LifecycleLayer::default())
            .layer(CatchUnwindLayer::default())
            .layer(ConcurrencyLayer::default())
            .layer(ClientProcessMonitorLayer::new(client.clone()))
            .service(Server::router(client))
    });

    #[cfg(unix)]
    let (stdin, stdout) = (
        async_lsp::stdio::PipeStdin::lock_tokio()?,
        async_lsp::stdio::PipeStdout::lock_tokio()?,
    );

    #[cfg(not(unix))]
    let (stdin, stdout) = (
        tokio_util::compat::TokioAsyncReadCompatExt::compat(tokio::io::stdin()),
        tokio_util::compat::TokioAsyncWriteCompatExt::compat_write(tokio::io::stdout()),
    );

    server.run_buffered(stdin, stdout).await?;
    Ok(())
}

#[cfg(test)]
mod tests;
