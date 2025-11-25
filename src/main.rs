use tracing;
use tracing_subscriber;

// Importar os módulos do workspace
use config::AppConfig;
use broker::Broker;
use persistence::PersistenceAdapter;
use collab::CollabModule;
use gateway::Gateway;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Carregar configuração
    let config = AppConfig::load()?;
    
    // 2. Inicializar logs e tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&config.log_level)),
        )
        .init();
    
    tracing::info!("Iniciando Gateway Monolito...");
    
    // 3. Inicializar runtime async (já está ativo via #[tokio::main])
    
    // 4. Criar recursos compartilhados (DB, configs, etc)
    // (Por enquanto, não há recursos compartilhados além da config)
    
    // 5. Inicializar broker
    let broker = Broker::new(&config);
    broker.start().await?;
    tracing::info!("Broker inicializado");
    
    // 6. Inicializar persistence
    let persistence = match PersistenceAdapter::new(&config).await {
        Ok(p) => {
            tracing::info!("Persistence inicializado");
            p
        }
        Err(e) => {
            tracing::error!("Falha ao inicializar persistence (Display): {}", e);
            tracing::error!("Falha ao inicializar persistence (Debug): {:?}", e);
            return Err(e);
        }
    };
    
    // 7. Inicializar collab com acesso ao broker e persistência
    let collab = CollabModule::new(&broker, &persistence, &config)?;
    collab.start().await?;
    tracing::info!("Collab inicializado");
    
    // 8. Inicializar gateway com acesso ao broker
    let (gateway, shutdown_rx) = Gateway::new(&broker, &config)?;
    tracing::info!("Gateway inicializado");
    
    // 9. Iniciar servidor HTTP do gateway
    let gateway_handle = tokio::spawn(async move {
        if let Err(e) = gateway.run_server(shutdown_rx).await {
            tracing::error!("Erro no servidor gateway: {:?}", e-);
        }
    });
    
    tracing::info!("Servidor Gateway iniciado na porta: {}", config.gateway_port);
    
    // 10. Registrar handlers de shutdown gracioso
    match tokio::signal::ctrl_c().await {
        Ok(()) => {
            tracing::warn!("Sinal de interrupção (Ctrl+C) recebido. Iniciando shutdown gracioso...");
            
            // Shutdown gracioso na ordem inversa
            gateway_handle.abort();
            tracing::info!("Gateway encerrado");
            
            if let Err(e) = collab.shutdown().await {
                tracing::error!("Erro ao fazer shutdown do collab: {:?}", e);
            }
            tracing::info!("Collab encerrado");
            
            if let Err(e) = persistence.shutdown().await {
                tracing::error!("Erro ao fazer shutdown da persistence: {:?}", e);
            }
            tracing::info!("Persistence encerrado");
            
            if let Err(e) = broker.shutdown().await {
                tracing::error!("Erro ao fazer shutdown do broker: {:?}", e);
            }
            tracing::info!("Broker encerrado");
        }
        Err(err) => {
            tracing::error!("Erro ao escutar sinal de shutdown: {:?}", err);
        }
    }

    tracing::info!("Shutdown completo. O Monolito está encerrando.");
    Ok(())
}
