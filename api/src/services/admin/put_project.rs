use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::error::Error;

use super::AdminServiceState;

pub async fn update_project(State(service):State<Arc<AdminServiceState>>,Json(request):Json<Request>) -> Result<Json<Response>,Error> {
    service.admin_use_case.update_project(&request.project_id, &request.name, &request.description).await?;
    Ok(Json(Response{
        code: 200,
        message: String::from("ok")
    }))
}

#[derive(Deserialize)]
pub struct Request {
    project_id: String,
    name: String,
    description: String
}

#[derive(Serialize)]
pub struct Response {
    code: i32,
    message: String
}