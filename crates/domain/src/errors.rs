// Erros comuns do sistema

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Erro de persistência: {0}")]
    Persistence(String),
    
    #[error("Erro de validação: {0}")]
    Validation(String),
    
    #[error("Recurso não encontrado: {0}")]
    NotFound(String),
    
    #[error("Erro de autenticação: {0}")]
    Authentication(String),
    
    #[error("Erro de autorização: {0}")]
    Authorization(String),
}

