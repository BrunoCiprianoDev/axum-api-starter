use crate::service::user_service::create_user_service::CreateUserService;
use crate::model::user_model::{UserForCreation, UserPublic};
use crate::util::app_error::{AppError, ErrorType};
use crate::util::http_status_code::{HttpResponse, HttpStatus};
use async_trait::async_trait;
use axum::Json;

#[async_trait]
pub trait CreateUserHandler {
    async fn execute(
        &self,
        user_for_creation: UserForCreation
    ) -> Result<Json<HttpResponse<UserPublic>>, Json<HttpResponse<AppError>>>;
}

pub struct CreateUserHandlerImpl {
    pub create_user_service: Box<dyn CreateUserService + Send + Sync + 'static>,
}

#[async_trait]
impl CreateUserHandler for CreateUserHandlerImpl {
    async fn execute(
        &self,
        user_for_creation: UserForCreation
    ) -> Result<Json<HttpResponse<UserPublic>>, Json<HttpResponse<AppError>>> {

        let created_user = self
            .create_user_service
            .execute(user_for_creation)
            .await
            .map_err(|err| {
 
                Json(HttpResponse {
                    status: match err.error_type {
                        ErrorType::ClientError => HttpStatus::BadRequest,
                        ErrorType::ServerError => HttpStatus::InternalServerError,
                    },
                    detail: err.detail.clone(),
                    content: err,
                })
            })?;


        Ok(Json(HttpResponse {
            status: HttpStatus::Created,
            detail: "User created successfully".to_string(),
            content: created_user,
        }))
    }
}