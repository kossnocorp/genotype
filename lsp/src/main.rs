use std::ops::ControlFlow;
use std::time::Duration;

use async_lsp::client_monitor::ClientProcessMonitorLayer;
use async_lsp::concurrency::ConcurrencyLayer;
use async_lsp::lsp_types;
use async_lsp::panic::CatchUnwindLayer;
use async_lsp::router::Router;
use async_lsp::server::LifecycleLayer;
use async_lsp::tracing::TracingLayer;
use async_lsp::{ClientSocket, LanguageClient, LanguageServer, ResponseError};
use futures::future::BoxFuture;
use tower::ServiceBuilder;
use tracing::{info, Level};

struct ServerState {
    client: ClientSocket,
    counter: i32,
}

impl LanguageServer for ServerState {
    type Error = ResponseError;
    type NotifyResult = ControlFlow<async_lsp::Result<()>>;

    fn initialize(
        &mut self,
        params: lsp_types::InitializeParams,
    ) -> BoxFuture<'static, Result<lsp_types::InitializeResult, Self::Error>> {
        eprintln!("Initialize with {params:?}");
        Box::pin(async move {
            Ok(lsp_types::InitializeResult {
                capabilities: lsp_types::ServerCapabilities {
                    hover_provider: Some(lsp_types::HoverProviderCapability::Simple(true)),
                    definition_provider: Some(lsp_types::OneOf::Left(true)),
                    ..lsp_types::ServerCapabilities::default()
                },
                server_info: None,
            })
        })
    }

    fn hover(
        &mut self,
        _: lsp_types::HoverParams,
    ) -> BoxFuture<'static, Result<Option<lsp_types::Hover>, Self::Error>> {
        let mut client = self.client.clone();
        let counter = self.counter;
        Box::pin(async move {
            tokio::time::sleep(Duration::from_secs(1)).await;
            client
                .show_message(lsp_types::ShowMessageParams {
                    typ: lsp_types::MessageType::INFO,
                    message: "Hello LSP".into(),
                })
                .unwrap();
            Ok(Some(lsp_types::Hover {
                contents: lsp_types::HoverContents::Scalar(lsp_types::MarkedString::String(
                    format!("I am a hover text {counter}!"),
                )),
                range: None,
            }))
        })
    }

    fn definition(
        &mut self,
        _: lsp_types::GotoDefinitionParams,
    ) -> BoxFuture<'static, Result<Option<lsp_types::GotoDefinitionResponse>, ResponseError>> {
        unimplemented!("Not yet implemented!");
    }

    fn did_change_configuration(
        &mut self,
        _: lsp_types::DidChangeConfigurationParams,
    ) -> ControlFlow<async_lsp::Result<()>> {
        ControlFlow::Continue(())
    }
}

struct TickEvent;

impl ServerState {
    fn new_router(client: ClientSocket) -> Router<Self> {
        let mut router = Router::from_language_server(Self { client, counter: 0 });

        router.event(Self::on_tick);

        router.request::<lsp_types::request::Initialize, _>(|_, params| async move {
            eprintln!("Initialize with {params:?}");
            Ok(lsp_types::InitializeResult {
                capabilities: lsp_types::ServerCapabilities {
                    hover_provider: Some(lsp_types::HoverProviderCapability::Simple(true)),
                    definition_provider: Some(lsp_types::OneOf::Left(true)),
                    ..lsp_types::ServerCapabilities::default()
                },
                server_info: None,
            })
        });

        router.request::<lsp_types::request::HoverRequest, _>(|st, _| {
            let client = st.client.clone();
            let counter = st.counter;
            async move {
                tokio::time::sleep(Duration::from_secs(1)).await;
                client
                    .notify::<lsp_types::notification::ShowMessage>(lsp_types::ShowMessageParams {
                        typ: lsp_types::MessageType::INFO,
                        message: "Hello LSP".into(),
                    })
                    .unwrap();
                Ok(Some(lsp_types::Hover {
                    contents: lsp_types::HoverContents::Scalar(lsp_types::MarkedString::String(
                        format!("I am a hover text {counter}!"),
                    )),
                    range: None,
                }))
            }
        });

        router.request::<lsp_types::request::GotoDefinition, _>(|_, _| async move {
            unimplemented!("Not yet implemented!")
        });

        router
            .notification::<lsp_types::notification::Initialized>(|_, _| ControlFlow::Continue(()));

        router.notification::<lsp_types::notification::DidChangeConfiguration>(|_, _| {
            ControlFlow::Continue(())
        });

        router.notification::<lsp_types::notification::DidOpenTextDocument>(|_, _| {
            ControlFlow::Continue(())
        });

        router.notification::<lsp_types::notification::DidChangeTextDocument>(|_, _| {
            ControlFlow::Continue(())
        });

        router.notification::<lsp_types::notification::DidCloseTextDocument>(|_, _| {
            ControlFlow::Continue(())
        });

        router.notification::<lsp_types::notification::DidChangeWatchedFiles>(|_, params| {
            info!("watched files changed: {:?}", params);
            ControlFlow::Continue(())
        });

        router
    }

    fn on_tick(&mut self, _: TickEvent) -> ControlFlow<async_lsp::Result<()>> {
        info!("tick");
        self.counter += 1;
        ControlFlow::Continue(())
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (server, _) = async_lsp::MainLoop::new_server(|client| {
        tokio::spawn({
            let client = client.clone();
            async move {
                let mut interval = tokio::time::interval(Duration::from_secs(1));
                loop {
                    interval.tick().await;
                    if client.emit(TickEvent).is_err() {
                        break;
                    }
                }
            }
        });

        ServiceBuilder::new()
            .layer(TracingLayer::default())
            .layer(LifecycleLayer::default())
            .layer(CatchUnwindLayer::default())
            .layer(ConcurrencyLayer::default())
            .layer(ClientProcessMonitorLayer::new(client.clone()))
            .service(ServerState::new_router(client))
    });

    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_ansi(false)
        .with_writer(std::io::stderr)
        .init();

    // Prefer truly asynchronous piped stdin/stdout without blocking tasks.
    #[cfg(unix)]
    let (stdin, stdout) = (
        async_lsp::stdio::PipeStdin::lock_tokio().unwrap(),
        async_lsp::stdio::PipeStdout::lock_tokio().unwrap(),
    );

    // Fallback to spawn blocking read/write otherwise.
    #[cfg(not(unix))]
    let (stdin, stdout) = (
        tokio_util::compat::TokioAsyncReadCompatExt::compat(tokio::io::stdin()),
        tokio_util::compat::TokioAsyncWriteCompatExt::compat_write(tokio::io::stdout()),
    );

    server.run_buffered(stdin, stdout).await.unwrap();
}
