use axum_api_starter::model::user_model::UserForCreation;
use axum_api_starter::service::user_service::create_user_service;

#[tokio::test]
async fn test_create_user_success() {
    let input = UserForCreation {
        email: "teste@example.com".to_string(),
        password: "minha_senha_secreta_123".to_string(),
    };

    let result = create_user_service::execute(input).await;

    assert!(result.is_ok());
    let user = result.unwrap();
    assert_eq!(user.email, "teste@example.com");
    assert_eq!(user.id.to_string().len(), 36);

    let parsed = uuid::Uuid::parse_str(&user.id.to_string());
    assert!(parsed.is_ok());
    let uuid = parsed.unwrap();
    assert_eq!(uuid.get_version_num(), 4);
}

#[tokio::test]
async fn test_create_user_with_empty_email() {
    let input = UserForCreation {
        email: "".to_string(),
        password: "123456".to_string(),
    };

    let result = create_user_service::execute(input).await;
    assert!(result.is_ok()); // ainda passa (validação é no handler)
}