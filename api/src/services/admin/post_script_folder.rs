use std::sync::Arc;

use axum::{extract::State, Json};

use serde::{Deserialize, Serialize};

use crate::error::Error;

use super::AdminServiceState;

pub async fn create_script_folder(State(service):State<Arc<AdminServiceState>>, Json(request):Json<Request>) -> Result<Json<Response>,Error> {
    let script_folder = service.admin_use_case.create_script_folder(&request.project_id, &request.parent_script_folder_id, &request.name).await?;
    Ok(Json(Response{
        code:200,
        message: String::from("ok"),
        data: Data{
            script_folder:ScriptFolder { 
                project_id: script_folder.project_id.to_hex(),
                _id: script_folder._id.to_hex(),
                name: script_folder.name 
            }
        }
    }))
}

#[derive(Deserialize)]
pub struct Request {
    project_id: String,
    parent_script_folder_id: String,
    name: String,
}

#[derive(Serialize)]
pub struct Response {
    code: i32,
    message: String,
    data: Data,
}

#[derive(Serialize)]
pub struct Data {
    script_folder: ScriptFolder
}

#[derive(Serialize)]
pub struct ScriptFolder {
    pub project_id:String,
    pub _id:String,
    pub name:String,
}