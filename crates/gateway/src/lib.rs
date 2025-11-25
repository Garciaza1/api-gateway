// Define o build do servidor

pub mod auth;
pub mod http;
pub mod ws;

pub use http::*;
pub use ws::*;
// pub use auth::*;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::{oneshot, Mutex};
// use axum::serve;
// use crate::http::create_router;

pub struct Gateway {
    port: u16,
    shutdown_tx: Arc<Mutex<Option<oneshot::Sender<()>>>>,
    // TODO: Adicionar dependências do broker
}

impl Gateway {
    pub fn new(
        _broker: &broker::Broker,
        config: &config::AppConfig,
    ) -> Result<(Self, oneshot::Receiver<()>), Box<dyn std::error::Error>> {
        let (shutdown_tx, shutdown_rx) = oneshot::channel();

        let gateway = Gateway {
            port: config.gateway_port,
            shutdown_tx: Arc::new(Mutex::new(Some(shutdown_tx))),
        };

        Ok((gateway, shutdown_rx))
    }

    pub async fn run_server(
        &self,
        shutdown_rx: tokio::sync::oneshot::Receiver<()>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let addr = format!("0.0.0.0:{}", self.port);
        tracing::info!("Gateway servidor rodando na porta {}", addr);

        let listener = TcpListener::bind(&addr).await?;
        let app = create_router();
        axum::serve(listener, app)
            .with_graceful_shutdown(async move {
                shutdown_rx.await.ok();
            })
            .await?;

        tracing::info!("Servidor HTTP finalizado.");
        Ok(())
    }

    pub async fn shutdown(&self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::warn!("Iniciando o envio do sinal de shutdown do Gateway...");
        let mut tx = self.shutdown_tx.lock().await;

        if let Some(tx) = tx.take() {
            tx.send(()).ok();
            tracing::info!("Sinal de shutdown do Gateway enviado com sucesso.");
        } else {
            tracing::warn!("O sinal de shutdown do Gateway já havia sido enviado.");
        }

        Ok(())
    }
}
