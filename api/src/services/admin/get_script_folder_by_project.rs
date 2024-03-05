use std::{sync::Arc, collections::HashMap};

use axum::{extract::{State, Path}, Json};
use serde::{Deserialize, Serialize};

use crate::error::Error;

use super::AdminServiceState;

pub async fn read_script_folder_by_project(State(service):State<Arc<AdminServiceState>>, Path(request):Path<Request>) -> Result<Json<Response>,Error> {
    let script_folder = service.admin_use_case.read_script_folder_by_project(&request.project_id).await?;
    let script_folder = script_folder.into_iter().map(|script_folder| {
        ScriptFolder{
            project_id: script_folder.project_id.to_hex(),
            _id: script_folder._id.to_hex(),
            name: script_folder.name,
            description: script_folder.description,
            parent_folder_id: script_folder.parent_folder_id.map(|parent_folder_id| parent_folder_id.to_hex()),
            child_folder_id: script_folder.child_folder_id.into_iter().map(|child_folder_id| child_folder_id.to_hex()).collect(),
            script_id: script_folder.script_id.into_iter().map(|script_id| script_id.to_hex()).collect(),
            env: script_folder.env,
        }
    }).collect();
    Ok(Json(Response{
        code: 200,
        message: String::from("ok"),
        data: Data{
            script_folder
        }
    }))
}

#[derive(Deserialize)]
pub struct Request {
    project_id: String
}

#[derive(Serialize)]
pub struct Response {
    code: i32,
    message: String,
    data: Data
}

#[derive(Serialize)]
pub struct Data {
    script_folder: Vec<ScriptFolder>
}

#[derive(Serialize)]
pub struct ScriptFolder {
     project_id:String,
     _id:String,
     name:String,
     description:String,
     parent_folder_id:Option<String>,
     child_folder_id:Vec<String>,
     script_id:Vec<String>,
     env:HashMap<String,String>
}