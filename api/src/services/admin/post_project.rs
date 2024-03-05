use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::error::Error;

use super::AdminServiceState;

pub async fn create_project(State(service):State<Arc<AdminServiceState>>,Json(request):Json<Request>) -> Result<Json<Response>,Error> {
    let (project,script_folder) = service.admin_use_case.create_project(&request.project_name).await?;
    Ok(Json(Response{
        code:200,
        message:String::from("ok"),
        data:Data{
            project:Project{
                _id:project._id.to_hex(),
                name:project.name,
                description: project.description,
                root_folder_id: project.root_folder_id.to_hex(),
            },
            script_folder: ScriptFolder{
                _id: script_folder._id.to_hex(),
                name: script_folder.name
            }
        }
    }))
}

#[derive(Deserialize)]
pub struct Request {
    project_name:String
}

#[derive(Serialize)]
pub struct Response {
    code: i32,
    message: String,
    data: Data,
}

#[derive(Serialize)]
pub struct Data {
    project:Project,
    script_folder:ScriptFolder
}

#[derive(Serialize)]
pub struct Project {
    _id: String,
    name: String,
    description: String,
    root_folder_id: String,
}

#[derive(Serialize)]
pub struct ScriptFolder {
    _id: String,
    name: String,
}