use axum::Json;
use axum::response::{ IntoResponse, Response};
use axum::http::{ StatusCode, Error as AxumError };
use serde_json::json;
use std::io::Error as IoError;

#[derive(Debug)]
pub enum TodoError
{
    InternalServer(String),
}

impl IntoResponse for TodoError
{
    fn into_response(self) -> Response
    {
        let(status, error_message) = match self
        {
            Self::InternalServer(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(json!(
        {
            "error": error_message,
        }
        ));

        (status, body).into_response()
    }
}

impl From<AxumError> for TodoError
{
    fn from(err: AxumError) ->  Self 
    {
        Self::InternalServer(err.to_string())
    }
}

impl From<IoError> for TodoError
{
    fn from(err: IoError) ->  Self 
    {
        Self::InternalServer(err.to_string())
    }
}
