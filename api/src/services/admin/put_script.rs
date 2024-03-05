use std::{sync::Arc, collections::HashMap};

use axum::{extract::State, Json};
use serde::{Serialize, Deserialize};

use crate::error::Error;

use super::AdminServiceState;

pub async fn update_script(State(service): State<Arc<AdminServiceState>>, Json(request):Json<Request>) -> Result<Json<Response>,Error> {
    service.admin_use_case.update_script(&request.script_id, &request.repository_url, &request.repository_branch, &request.username, &request.password, &request.base_image, &request.command_before_run, &request.run_command, &request.env).await?;
    Ok(Json(Response{
        code: 200,
        message: String::from("ok")
    }))
}

#[derive(Deserialize)]
pub struct Request {
    script_id: String,
    repository_url:String,
    repository_branch: String,
    username: String,
    password: String,
    base_image: String,
    command_before_run: String,
    run_command: String,
    env:HashMap<String,String>
}

#[derive(Serialize)]
pub struct Response {
    code: i32,
    message: String
}
