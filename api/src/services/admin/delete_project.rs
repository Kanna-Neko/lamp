use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::error::Error;

use super::AdminServiceState;

pub async fn delete_project(State(service):State<Arc<AdminServiceState>>, Json(request):Json<Request>) -> Result<Json<Response>,Error> {
    if let Err(err) = request.validate() {
        let err:Error = err.into();
        return Err(err)
    }

    service.admin_use_case.delete_project(&request.project_id).await?;
    Ok(Json(Response{
        code: 200,
        message: String::from("ok")
    }))
}

#[derive(Deserialize,Validate)]
pub struct Request {
    #[validate(length(min = 1))]
    project_id: String
}

#[derive(Serialize)]
pub struct Response {
    code: i32,
    message: String
}