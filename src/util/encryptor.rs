pub trait Encryptor {
    fn encrypt(&self, data: &str) -> Result<String, String>;
    fn verify(&self, data: &str, hash: &str) -> Result<bool, String>;
}
pub struct BcryptEncryptor;

impl Encryptor for BcryptEncryptor {
    fn encrypt(&self, data: &str) -> Result<String, String> {
        bcrypt::hash(data, bcrypt::DEFAULT_COST)
            .map_err(|e| format!("Erro ao criptografar senha: {}", e))
    }

    fn verify(&self, data: &str, hash: &str) -> Result<bool, String> {
        bcrypt::verify(data, hash)
            .map_err(|e| format!("Erro ao validar senha: {}", e))
    }
}