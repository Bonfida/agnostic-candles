use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum ServerError {
    #[display(fmt = "An internal error occurred. Please try again later.")]
    InternalError,
    #[display(fmt = "Bad request parameters")]
    WrongParameters,
    #[display(fmt = "Wrong resolution")]
    WrongResolution,
    #[display(fmt = "DB error")]
    DbQuerryError,
    #[display(fmt = "Error getting connection")]
    DbPoolError,
    #[display(fmt = "Raw market not found")]
    RawMarketNotFound,
    #[display(fmt = "Request symbol not found")]
    SymbolNotFound,
}

impl error::ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ServerError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::WrongParameters => StatusCode::BAD_REQUEST,
            ServerError::WrongResolution => StatusCode::BAD_REQUEST,
            ServerError::DbQuerryError => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::DbPoolError => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::RawMarketNotFound => StatusCode::BAD_REQUEST,
            ServerError::SymbolNotFound => StatusCode::BAD_REQUEST,
        }
    }
}

impl From<ServerError> for std::io::Error {
    fn from(e: ServerError) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
    }
}
