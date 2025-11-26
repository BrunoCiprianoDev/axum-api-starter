use crate::model::user_model::{UserForCreation, UserResponse};
use bcrypt::{hash, DEFAULT_COST};
use uuid::Uuid;

pub async fn execute(user_data: UserForCreation) -> Result<UserResponse, String> {
    let user_id = Uuid::new_v4().to_string();

    let hashed_password = hash(&user_data.password, DEFAULT_COST)
        .map_err(|_| "Falha ao hashear senha".to_string())?;

      println!(
            "Usuário criado! ID: {}, Email: {}, Hash da senha: {}",
            user_id, user_data.email, hashed_password
        );

    Ok(UserResponse {
        id: user_id
            .parse()
            .map_err(|_| "Erro ao converter UUID".to_string())?,
        email: user_data.email.clone(),
    })
}
     
