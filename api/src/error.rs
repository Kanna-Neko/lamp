use axum::{response::{IntoResponse,Response}, Json,http::StatusCode};
use serde_json::json;

#[derive(Clone)]
pub enum ErrorType {
    DatabaseError,
    ObjectIdParseError,
    ValidateError,
    AuthError,
    NotFoundRecordError,
    ProjectNotSameError,
    MoveRootScriptFolderError,
    DeleteRootScriptFolderError,
}

#[derive(Clone)]
pub struct Error {
    pub error_type: ErrorType,
    pub reason: String
}

impl Error {
    pub fn code(&self) -> i32 {
        match self.error_type {
            ErrorType::DatabaseError => 1001,
            ErrorType::ObjectIdParseError => 1002,

            ErrorType::ValidateError => 2001,
            ErrorType::NotFoundRecordError => 2002,
            ErrorType::AuthError => 2003,
            ErrorType::ProjectNotSameError => 2004,
            ErrorType::MoveRootScriptFolderError => 2005,
            ErrorType::DeleteRootScriptFolderError => 2006,
        }
    }
    pub fn description(&self) -> String{
        self.reason.clone()
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "code":self.code(),
            "message":self.description(),
        }));
        let status_code = match self.error_type  {
            ErrorType::ValidateError => StatusCode::BAD_REQUEST,
            ErrorType::AuthError => StatusCode::UNAUTHORIZED,
            ErrorType::ObjectIdParseError => StatusCode::BAD_REQUEST,
            ErrorType::ProjectNotSameError => StatusCode::BAD_REQUEST,
            ErrorType::MoveRootScriptFolderError => StatusCode::BAD_REQUEST,
            ErrorType::DeleteRootScriptFolderError => StatusCode::BAD_REQUEST,
            _ => StatusCode::SERVICE_UNAVAILABLE,
        };

        (status_code,body).into_response()
    }
}

impl From<mongodb::error::Error> for Error {
    fn from(value: mongodb::error::Error) -> Self {
        match value.get_custom::<Error>() {
            Some(err) => {
                err.clone()
            },
            None => {
                Self {
                    error_type: ErrorType::DatabaseError,
                    reason: value.to_string()
                }
            }
        }
    }
}

impl From<mongodb::bson::oid::Error> for Error {
    fn from(value: mongodb::bson::oid::Error) -> Self {
        Self {
            error_type: ErrorType::ObjectIdParseError,
            reason: value.to_string()
        }
    }
}

impl From<validator::ValidationErrors> for Error {
    fn from(value: validator::ValidationErrors) -> Self {
        Self {
            error_type: ErrorType::ValidateError,
            reason: value.to_string()
        }
    }
}