use std::sync::Arc;

use axum::{extract::State, Json};
use serde::Serialize;

use crate::error::Error;

use super::AdminServiceState;

pub async fn read_project_all(State(service):State<Arc<AdminServiceState>>) -> Result<Json<Response>,Error> {
    let result = service.admin_use_case.read_project_all().await?;
    let result:Vec<Project> = result.into_iter().map(|v| {
        Project{
            _id: v._id.to_hex(),
            name: v.name,
            description: v.description,
            root_folder_id: v.root_folder_id.to_hex(),
        }
    }).collect();
    Ok(Json(Response {
        code: 200,
        message: String::from("ok"),
        data: Data{
            project: result
        }
    }))
}

#[derive(Serialize)]
pub struct Response {
    code: i32,
    message: String,
    data: Data
}

#[derive(Serialize)]
pub struct Data {
    project: Vec<Project>
}

#[derive(Serialize)]
pub struct Project {
    _id: String,
    name: String,
    description:String,
    root_folder_id:String,
}