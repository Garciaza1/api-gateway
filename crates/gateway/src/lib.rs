// Define o build do servidor

pub mod http;
pub mod ws;
pub mod auth;

pub use http::*;
pub use ws::*;
pub use auth::*;

pub struct Gateway {
    port: u16,
    // TODO: Adicionar dependências do broker
}

impl Gateway {
    pub fn new(
        _broker: &broker::Broker,
        config: &config::AppConfig,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Gateway {
            port: config.gateway_port,
        })
    }

    pub async fn run_server(&self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Iniciar servidor HTTP e WebSocket
        tracing::info!("Gateway servidor rodando na porta {}", self.port);
        
        // Por enquanto, apenas espera indefinidamente
        tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
        
        Ok(())
    }

    pub async fn shutdown(&self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Fazer shutdown gracioso do servidor
        Ok(())
    }
}
