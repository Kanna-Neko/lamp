use std::sync::Arc;

use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};

use crate::error::Error;

use super::AdminServiceState;

pub async fn delete_script(State(service): State<Arc<AdminServiceState>>, Json(request):Json<Request>) -> Result<Json<Response>, Error> {
    service.admin_use_case.delete_script(&request.script_id).await?;
    Ok(Json(Response{
        code: 200,
        message: String::from("ok")
    }))

}

#[derive(Deserialize)]
pub struct Request {
    script_id: String
}

#[derive(Serialize)]
pub struct Response {
    code: i32,
    message: String
}