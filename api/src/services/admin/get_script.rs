use std::{collections::HashMap, sync::Arc};

use axum::{Json, extract::{State, Path}};
use serde::{Deserialize, Serialize};

use crate::error::Error;

use super::AdminServiceState;

pub async fn read_script(State(service):State<Arc<AdminServiceState>>,Path(request):Path<Request>) -> Result<Json<Response>,Error> {
    let script = service.admin_use_case.read_script(request.script_id).await?;
    let script = Script{
        _id:script._id.to_hex(),
        project_id:script.project_id.to_hex(),
        parent_folder_id: script.parent_folder_id.to_hex(),
        env: script.env
    };
    Ok(Json(Response{
        code:200,
        message:String::from("ok"),
        data:Data{
            script
        }
    }))
}

#[derive(Deserialize)]
pub struct Request {
    script_id: String
}

#[derive(Serialize)]
pub struct Response{
    code: i32,
    message: String,
    data: Data
}

#[derive(Serialize)]
struct Data {
    script:Script
}

#[derive(Serialize)]
struct Script {
    _id: String,
    project_id: String,
    parent_folder_id: String,
    env: HashMap<String,String>
}