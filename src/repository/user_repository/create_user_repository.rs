use crate::model::user_model::{User, UserPublic};
use async_trait::async_trait;

#[async_trait]
pub trait CreateUserRepository {
    async fn execute(&self, user: User) -> Result<UserPublic, String>;
}