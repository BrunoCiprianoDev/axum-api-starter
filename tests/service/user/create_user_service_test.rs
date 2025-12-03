use axum_api_starter::model::user_model::{User, UserForCreation, UserPublic, UserRole};
use axum_api_starter::repository::user_repository::create_user_repository::CreateUserRepository;
use axum_api_starter::repository::user_repository::find_user_by_email_repository::FindUserByEmailRepository;
use axum_api_starter::service::user_service::create_user_service::{
    CreateUserService, CreateUserServiceImpl,
};
use axum_api_starter::util::encryptor::Encryptor;
use axum_api_starter::util::error::app_error::AppError;
use axum_api_starter::util::uuid_generator::UuidGenerator;
use chrono::Utc;
use async_trait::async_trait;

#[cfg(test)]
mod tests {
    use super::*; // Importa as definições do arquivo principal (models, service, etc.)
    use tokio; // Necessário para rodar testes assíncronos

    // --- Mocks ---

    pub struct MockCreateUserRepository {
        pub result: Result<UserPublic, String>,
    }

    #[async_trait]
    impl CreateUserRepository for MockCreateUserRepository {
        async fn execute(&self, _user: User) -> Result<UserPublic, String> {
            self.result.clone()
        }
    }

    pub struct MockFindUserByEmailRepository {
        pub result: Result<Option<User>, String>,
    }

    #[async_trait]
    impl FindUserByEmailRepository for MockFindUserByEmailRepository {
        async fn execute(&self, _email: String) -> Result<Option<User>, String> {
            self.result.clone()
        }
    }

    pub struct MockEncryptor {
        pub encrypt_result: Result<String, String>,
        // O verify não é usado, mas precisa ser implementado
        pub verify_result: Result<bool, String>,
    }

    impl Encryptor for MockEncryptor {
        fn encrypt(&self, _data: &str) -> Result<String, String> {
            self.encrypt_result.clone()
        }

        fn verify(&self, _data: &str, _hash: &str) -> Result<bool, String> {
            self.verify_result.clone()
        }
    }

    pub struct MockUuidGenerator {
        pub result: Result<String, String>,
    }

    impl UuidGenerator for MockUuidGenerator {
        fn generate(&self) -> Result<String, String> {
            self.result.clone()
        }
    }

    // --- Helpers ---

    fn setup_user_for_creation() -> UserForCreation {
        UserForCreation {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            role: UserRole::User,
        }
    }

    fn setup_user_public(id: String) -> UserPublic {
        UserPublic {
            id,
            email: "test@example.com".to_string(),
            role: UserRole::User,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    // --- Casos de Teste ---

    #[tokio::test]
    async fn should_create_user_successfully() {
        let user_id = "mock-uuid-123".to_string();
        let hashed_pass = "mock_hashed_password".to_string();
        let user_creation_data = setup_user_for_creation();
        let expected_user_public = setup_user_public(user_id.clone());

        // 1. Configurar Mocks para Sucesso
        let create_repo_mock = MockCreateUserRepository {
            result: Ok(expected_user_public.clone()),
        };
        let find_repo_mock = MockFindUserByEmailRepository {
            result: Ok(None), // Usuário NÃO existe
        };
        let encryptor_mock = MockEncryptor {
            encrypt_result: Ok(hashed_pass),
            verify_result: Ok(true),
        };
        let uuid_mock = MockUuidGenerator {
            result: Ok(user_id),
        };

        // 2. Criar a instância do Serviço com os Mocks
        let service = CreateUserServiceImpl {
            create_user_repository: Box::new(create_repo_mock),
            find_user_by_email_repository: Box::new(find_repo_mock),
            encryptor: Box::new(encryptor_mock),
            uuid_generator: Box::new(uuid_mock),
        };

        // 3. Executar e Assert
        let result = service.execute(user_creation_data).await;

        assert!(result.is_ok());
        let created_user = result.unwrap();
        assert_eq!(created_user.email, expected_user_public.email);
        assert_eq!(created_user.id, expected_user_public.id);
    }

    #[tokio::test]
    async fn should_return_bad_request_if_user_already_exists() {
        let user_creation_data = setup_user_for_creation();
        let existing_user = User {
            id: "existing-id".to_string(),
            email: user_creation_data.email.clone(),
            password: "old-hash".to_string(),
            role: UserRole::User,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // 1. Configurar Mocks para Falha (Usuário Existente)
        let create_repo_mock = MockCreateUserRepository {
            result: Err("Should not be called".to_string()), // Não deve ser chamado
        };
        let find_repo_mock = MockFindUserByEmailRepository {
            result: Ok(Some(existing_user)), // Usuário JÁ existe!
        };
        let encryptor_mock = MockEncryptor {
            encrypt_result: Ok("mock_hashed_password".to_string()),
            verify_result: Ok(true),
        };
        let uuid_mock = MockUuidGenerator {
            result: Ok("mock-uuid".to_string()),
        };

        // 2. Criar a instância do Serviço com os Mocks
        let service = CreateUserServiceImpl {
            create_user_repository: Box::new(create_repo_mock),
            find_user_by_email_repository: Box::new(find_repo_mock),
            encryptor: Box::new(encryptor_mock),
            uuid_generator: Box::new(uuid_mock),
        };

        // 3. Executar e Assert
        let result = service.execute(user_creation_data).await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        // Verifica se o erro é o AppError::BadRequest correto
        match error {
            AppError::BadRequest(msg) => assert_eq!(msg, "Email already in use"),
            _ => panic!("Expected BadRequest error, got {:?}", error),
        }
    }

    #[tokio::test]
    async fn should_return_internal_error_on_encryption_failure() {
        let user_creation_data = setup_user_for_creation();

        // 1. Configurar Mocks para Falha (Encryptor)
        let create_repo_mock = MockCreateUserRepository {
            result: Err("Should not be called".to_string()),
        };
        let find_repo_mock = MockFindUserByEmailRepository { result: Ok(None) };
        let encryptor_mock = MockEncryptor {
            encrypt_result: Err("Encryptor failed".to_string()), // FALHA aqui
            verify_result: Ok(true),
        };
        let uuid_mock = MockUuidGenerator {
            result: Ok("mock-uuid".to_string()),
        };

        // 2. Criar a instância do Serviço com os Mocks
        let service = CreateUserServiceImpl {
            create_user_repository: Box::new(create_repo_mock),
            find_user_by_email_repository: Box::new(find_repo_mock),
            encryptor: Box::new(encryptor_mock),
            uuid_generator: Box::new(uuid_mock),
        };

        // 3. Executar e Assert
        let result = service.execute(user_creation_data).await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        // Verifica se o erro é o AppError::Internal correto
        match error {
            AppError::Internal(msg) => assert!(msg.contains("Encryption error")),
            _ => panic!("Expected Internal error, got {:?}", error),
        }
    }

    #[tokio::test]
    async fn should_return_internal_error_on_uuid_failure() {
        let user_creation_data = setup_user_for_creation();
        let uuid_error_message = "Failed to generate UUID".to_string();

        // 1. Configurar Mocks para Falha (UuidGenerator)
        let create_repo_mock = MockCreateUserRepository {
            result: Err("Should not be called".to_string()),
        };
        let find_repo_mock = MockFindUserByEmailRepository {
            result: Ok(None),
        };
        let encryptor_mock = MockEncryptor {
            encrypt_result: Ok("mock_hashed_password".to_string()),
            verify_result: Ok(true),
        };
        let uuid_mock = MockUuidGenerator {
            // FALHA aqui, simulando um erro do gerador
            result: Err(uuid_error_message.clone()), 
        };

        // 2. Criar a instância do Serviço com os Mocks
        let service = CreateUserServiceImpl {
            create_user_repository: Box::new(create_repo_mock),
            find_user_by_email_repository: Box::new(find_repo_mock),
            encryptor: Box::new(encryptor_mock),
            uuid_generator: Box::new(uuid_mock),
        };

        // 3. Executar e Assert
        let result = service.execute(user_creation_data).await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        
        // Verifica se o erro é o AppError::Internal correto com a mensagem de UUID
        match error {
            AppError::Internal(msg) => {
                assert!(msg.contains("UUID error"));
                assert!(msg.contains(&uuid_error_message));
            }
            _ => panic!("Expected Internal error for UUID, got {:?}", error),
        }
    }

    #[tokio::test]
    async fn should_return_internal_error_on_create_repository_failure() {
        let user_creation_data = setup_user_for_creation();
        let repo_error_message = "Database connection failed".to_string();

        // 1. Configurar Mocks para Falha (CreateUserRepository)
        let create_repo_mock = MockCreateUserRepository {
            // FALHA aqui, simulando um erro do DB
            result: Err(repo_error_message.clone()), 
        };
        let find_repo_mock = MockFindUserByEmailRepository {
            result: Ok(None), // Passa na verificação de email
        };
        let encryptor_mock = MockEncryptor {
            encrypt_result: Ok("mock_hashed_password".to_string()), // Passa na criptografia
            verify_result: Ok(true),
        };
        let uuid_mock = MockUuidGenerator {
            result: Ok("mock-uuid-success".to_string()), // Passa no UUID
        };

        // 2. Criar a instância do Serviço com os Mocks
        let service = CreateUserServiceImpl {
            create_user_repository: Box::new(create_repo_mock),
            find_user_by_email_repository: Box::new(find_repo_mock),
            encryptor: Box::new(encryptor_mock),
            uuid_generator: Box::new(uuid_mock),
        };

        // 3. Executar e Assert
        let result = service.execute(user_creation_data).await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        
        // Verifica se o erro é o AppError::Internal correto com a mensagem do repositório
        match error {
            AppError::Internal(msg) => {
                assert!(msg.contains("Repository error"));
                assert!(msg.contains(&repo_error_message));
            }
            _ => panic!("Expected Internal error for Repository failure, got {:?}", error),
        }
    }
}
