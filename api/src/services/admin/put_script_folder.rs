use std::{sync::Arc, collections::HashMap};

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::error::Error;

use super::AdminServiceState;

pub async fn update_script_folder(State(service):State<Arc<AdminServiceState>>, Json(request):Json<Request>) -> Result<Json<Response>,Error> {
    service.admin_use_case.update_script_folder(&request.script_folder_id, &request.name, &request.description, &request.env).await?;
    Ok(Json(Response{
        code: 200,
        message: String::from("ok")
    }))
}

#[derive(Deserialize)]
pub struct Request {
    script_folder_id: String,
    name: String,
    description: String,
    env: HashMap<String,String>
}

#[derive(Serialize)]
pub struct Response {
    code: i32,
    message: String
}