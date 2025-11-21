// Define o PersistenceAdapter struct

pub mod adapters;
pub mod schema;

pub use adapters::*;
pub use schema::*;

pub struct PersistenceAdapter {
    // TODO: Implementar adaptador de storage
}

impl PersistenceAdapter {
    pub fn new(_config: &config::AppConfig) -> Result<Self, Box<dyn std::error::Error>> {
        // TODO: Inicializar conexão com banco de dados
        Ok(PersistenceAdapter {})
    }

    pub async fn shutdown(&self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Fazer shutdown gracioso
        Ok(())
    }
}
