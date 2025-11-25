// Gerenciamento centralizado de configuração

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub gateway_port: u16,
    pub broker_queue_size: usize,
    pub broker_storage_dir: PathBuf,
    pub collab_max_document_size: usize,
    pub collab_timeout_seconds: u64,
    pub log_level: String,
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        // Por enquanto, retorna uma config padrão
        // TODO: Carregar de arquivo YAML/TOML e variáveis de ambiente
        Ok(AppConfig {
            gateway_port: 8080,
            broker_queue_size: 1000,
            broker_storage_dir: PathBuf::from("./data"),
            collab_max_document_size: 10 * 1024 * 1024, // 10MB
            collab_timeout_seconds: 30,
            log_level: "info".to_string(),
        })
    }
}
