use mongodb::{Client, Database, Collection};
use domain::types::{UserID, User, NewUser};
use domain::errors::DomainError;
use domain::traits::user_port::UserStorage;
use async_trait::async_trait;

// Este struct será o seu Adaptador
pub struct MongoDBUserStorage {
    db: Database,
    users: Collection<User>, 
}

// crates/persistence/src/adapters/mongodb_user_storage.rs

impl MongoDBUserStorage {
    pub async fn new() -> Result<Self, mongodb::error::Error> {
        let uri = std::env::var("MONGO_URL")
            .map_err(|e| mongodb::error::Error::custom(format!("MONGO_URL não configurado: {}", e)))?;
        
        let client = Client::with_uri_str(uri).await?;
        let db = client.database("gateway");
        let users = db.collection::<User>("users");

        Ok(MongoDBUserStorage {
            db,
            users,
        })
    }
}

#[async_trait]
impl UserStorage for MongoDBUserStorage {
    
    async fn save_user(&self, new_user: NewUser) -> Result<User, DomainError> {
        use chrono::Utc;
        
        let now = Utc::now();
        let user_to_insert = User {
            id: None, 
            username: new_user.username.clone(),
            email: new_user.email.clone(),
            password: new_user.password,
            last_login: now,
            created_at: now,
        };

        match self.users.insert_one(user_to_insert, None).await {
            Ok(result) => {
                let oid = result.inserted_id
                    .as_object_id()
                    .ok_or_else(|| DomainError::Persistence("ID retornado não é um ObjectId válido".to_string()))?;
                
                Ok(User { 
                    id: Some(oid.clone()), 
                    username: new_user.username,
                    email: new_user.email,
                    password: String::new(), // Não retornamos a senha
                    last_login: now,
                    created_at: now,
                })
            }
            Err(e) => {
                Err(DomainError::Persistence(format!("Erro ao salvar usuário no MongoDB: {}", e)))
            }
        }
    }
    
    async fn load_user_by_id(&self, user_id: &domain::types::UserID) -> Result<User, DomainError> {
        todo!()
    }
}