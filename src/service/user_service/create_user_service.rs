use crate::model::user_model::{User, UserForCreation, UserPublic};
use crate::repository::user_repository::create_user_repository::CreateUserRepository;
use crate::repository::user_repository::find_user_by_email_repository::FindUserByEmailRepository;
use crate::util::app_error::AppError;
use crate::util::encryptor::Encryptor;
use crate::util::uuid_generator::UuidGenerator;
use async_trait::async_trait;
use chrono::Utc;

#[async_trait]
pub trait CreateUserService {
    async fn execute(&self, user_for_creation: UserForCreation) -> Result<UserPublic, AppError>;
}

pub struct CreateUserServiceImpl {
    pub create_user_repository: Box<dyn CreateUserRepository + Send + Sync + 'static>,
    pub find_user_by_email_repository: Box<dyn FindUserByEmailRepository + Send + Sync + 'static>,
    pub encryptor: Box<dyn Encryptor + Send + Sync + 'static>,
    pub uuid_generator: Box<dyn UuidGenerator + Send + Sync + 'static>,
}

#[async_trait]
impl CreateUserService for CreateUserServiceImpl {
    async fn execute(&self, user_for_creation: UserForCreation) -> Result<UserPublic, AppError> {
        let user_exists = self
            .find_user_by_email_repository
            .execute(user_for_creation.email.clone())
            .await
            .map_err(|err| AppError::server_error(err))?;

        if user_exists.is_some() {
            return Err(AppError::client_error("User with this email already exists".to_string()));
        }

        let user_id = self
            .uuid_generator
            .generate()
        .map_err(|err| AppError::server_error(err))?;

        let hashed_password = self
            .encryptor
            .encrypt(&user_for_creation.password)
        .map_err(|err| AppError::server_error(err))?;

        let user = User {
            id: user_id.clone(),
            email: user_for_creation.email.clone(),
            password: hashed_password,
            role: user_for_creation.role,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let user_created = self
            .create_user_repository
            .execute(user)
            .await
        .map_err(|err| AppError::server_error(err))?;

        Ok(user_created)
    }
}
