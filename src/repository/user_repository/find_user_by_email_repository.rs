use crate::model::user_model::User;
use async_trait::async_trait;

#[async_trait]
pub trait FindUserByEmailRepository {
    async fn execute(&self, email: String) -> Result<Option<User>, String>;
}
