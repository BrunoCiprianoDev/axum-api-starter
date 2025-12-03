pub trait UuidGenerator {
    fn generate(&self) -> Result<String, String>;
}

pub struct UuidV4Generator;

impl UuidGenerator for UuidV4Generator {
    fn generate(&self) -> Result<String, String> {
        Ok(uuid::Uuid::new_v4().to_string())
    }
}