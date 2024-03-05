use std::{collections::HashMap, sync::Arc};

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::error::Error;

use super::AdminServiceState;

pub async fn create_script(State(service):State<Arc<AdminServiceState>>, Json(request):Json<Request>) -> Result<Json<Response>,Error> {
    let script = service.admin_use_case.create_script(request.project_id, request.parent_folder_id, request.env).await?;
    let script = Script{
        _id:script._id.to_hex()
    };
    Ok(Json(Response{
        code: 200,
        message: String::from("ok"),
        data:Data{
            script
        }
    }))
}

#[derive(Deserialize)]
pub struct Request {
    project_id: String,
    parent_folder_id: String,
    env:HashMap<String,String>
}

#[derive(Serialize)]
pub struct Response {
    code: i32,
    message: String,
    data: Data
}

#[derive(Serialize)]
struct Data {
    script: Script
}

#[derive(Serialize)]
struct Script {
    _id: String,
}