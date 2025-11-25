// Define o PersistenceAdapter struct
use domain::traits::user_port::UserStorage;
use crate::adapters::mongodb_user_storage::MongoDBUserStorage;

pub mod adapters;
pub mod schema;

pub use adapters::*;
pub use schema::*;

pub struct PersistenceAdapter {
    user_storage: MongoDBUserStorage,
}

impl PersistenceAdapter {
    pub async fn new(
        _config: &config::AppConfig
    ) -> Result<Self, Box<dyn std::error::Error>> {
        
        let user_storage = MongoDBUserStorage::new().await?;

        Ok(PersistenceAdapter { user_storage })
    }

    pub async fn shutdown(&self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("Desligamento do PersistenceAdapter: Fechando conexões de DB...");
        // O driver do MongoDB gerencia o fechamento, mas é um bom lugar para lógica de limpeza futura.
        Ok(())
    }
}

// Implementação da Porta (Trait)
#[async_trait::async_trait]
impl UserStorage for PersistenceAdapter {
    
    async fn save_user(
        &self, 
        new_user: domain::types::NewUser
    ) -> Result<domain::types::User, domain::errors::DomainError> {
        self.user_storage.save_user(new_user).await
    }
    
    async fn load_user_by_id(
        &self, 
        user_id: &domain::types::UserID
    ) -> Result<domain::types::User, domain::errors::DomainError> {
        self.user_storage.load_user_by_id(user_id).await
    }
}