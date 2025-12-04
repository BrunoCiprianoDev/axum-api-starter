use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum HttpStatus {
    // 2xx Success
    Ok,
    Created,
    NoContent,

    // 4xx Client Errors
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    TooManyRequests,

    // 5xx Server Errors
    InternalServerError,
}

impl HttpStatus {
    pub fn code(&self) -> u16 {
        match self {
            HttpStatus::Ok => 200,
            HttpStatus::Created => 201,
            HttpStatus::NoContent => 204,

            HttpStatus::BadRequest => 400,
            HttpStatus::Unauthorized => 401,
            HttpStatus::Forbidden => 403,
            HttpStatus::NotFound => 404,
            HttpStatus::TooManyRequests => 429,

            HttpStatus::InternalServerError => 500,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            HttpStatus::Ok => "OK",
            HttpStatus::Created => "Created",
            HttpStatus::NoContent => "NoContent",

            HttpStatus::BadRequest => "BadRequest",
            HttpStatus::Unauthorized => "Unauthorized",
            HttpStatus::Forbidden => "Forbidden",
            HttpStatus::NotFound => "NotFound",
            HttpStatus::TooManyRequests => "TooManyRequests",

            HttpStatus::InternalServerError => "InternalServerError",
        }
    }
}

#[derive(Debug, Serialize)]
pub struct HttpResponse<T> {
    pub status: HttpStatus,
    pub detail: String,
    pub content: T,
}

