use {
    actix_web::{web::Query, HttpRequest},
    serde::de::DeserializeOwned,
};

use crate::error::ServerError;

pub fn parse_query<T: DeserializeOwned>(req: &HttpRequest) -> Result<T, ServerError> {
    match Query::<T>::from_query(req.query_string()) {
        Ok(x) => Ok(x.into_inner()),
        _ => Err(ServerError::WrongParameters),
    }
}
