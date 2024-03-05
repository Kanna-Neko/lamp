use std::sync::Arc;

use axum::{extract::{State, Path}, Json};
use serde::{Serialize, Deserialize};
use validator::Validate;

use crate::error::Error;

use super::AdminServiceState;

pub async fn read_project(State(service):State<Arc<AdminServiceState>>, Path(request):Path<Request>) -> Result<Json<Response>,Error> {
    if let Err(err) = request.validate() {
        return Err(err.into());
    }
    let project = service.admin_use_case.read_project(&request.project_id).await?;
    Ok(Json(Response{
        code:200,
        message:String::from("ok"),
        data:Data{
            project: Project{
                _id: project._id.to_hex(),
                name: project.name,
                description: project.description,
                root_folder_id: project.root_folder_id.to_hex(), 
            }
        }
    }))
}

#[derive(Deserialize,Validate)]
pub struct Request {
    #[validate(length(min = 1))]
    project_id:String
}

#[derive(Serialize)]
pub struct Response {
    code: i32,
    message: String,
    data: Data,
}

#[derive(Serialize)]
struct Data {
    project: Project
}

#[derive(Serialize)]
struct Project {
    _id: String,
    name: String,
    description: String,
    root_folder_id: String,
}



