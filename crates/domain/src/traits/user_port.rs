// crates/domain/src/traits/user_port.rs
use crate::types::{UserID, User, NewUser}; 
use crate::errors::DomainError; 

#[async_trait::async_trait]
pub trait UserStorage {
    async fn load_user_by_id(&self, user_id: &UserID) -> Result<User, DomainError>;
    async fn save_user(&self, new_user: NewUser) -> Result<User, DomainError>;
    
    // ... outras operações necessárias (e.g., update_profile, find_by_email)
}
