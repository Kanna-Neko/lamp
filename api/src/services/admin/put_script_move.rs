use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::error::Error;

use super::AdminServiceState;

pub async fn move_script(State(service):State<Arc<AdminServiceState>>, Json(request): Json<Request>) -> Result<Json<Response>,Error> {
    service.admin_use_case.move_script(&request.script_id, &request.destination_folder_id).await?;
    Ok(Json(Response {
        code:200, message: String::from("ok") 
    }))
}

#[derive(Deserialize)]
pub struct Request {
    script_id: String,
    destination_folder_id: String,
}

#[derive(Serialize)]
pub struct Response {
    code: i32,
    message: String
}