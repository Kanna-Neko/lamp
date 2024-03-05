use std::sync::Arc;

use crate::controllers::AdminUseCase;
mod post_project;
mod get_project;
mod delete_project;
mod post_script_folder;
mod get_script_folder_by_project;
mod get_project_all;
mod post_script;
mod get_script;
mod put_project;
mod put_script_folder;
mod put_script;
mod put_script_move;
mod put_script_folder_move;
mod delete_script;
mod delete_script_folder;
use axum::{
    Router,
    routing::{post,get, delete, put}
};

pub struct AdminService {
    admin_use_case: Arc<AdminUseCase>
}

impl AdminService {
    pub fn new(admin_use_case: Arc<AdminUseCase>) -> Self {
        Self {
            admin_use_case:Arc::clone(&admin_use_case)
        }
    }
    pub fn router(&self) -> axum::Router {
        let router = Router::new()
            // create project
            .route("/project",post(post_project::create_project))
            // read project
            .route("/project/:project_id",get(get_project::read_project))
            // read all project
            .route("/project_all",get(get_project_all::read_project_all))
            // update project
            .route("/project",put(put_project::update_project))
            // delete project
            .route("/project",delete(delete_project::delete_project))
            // create script folder
            .route("/script_folder",post(post_script_folder::create_script_folder))
            // update script folder
            .route("/script_folder",put(put_script_folder::update_script_folder))
            // read script folder by project id
            .route("/script_folder_by_project/:project_id",get(get_script_folder_by_project::read_script_folder_by_project))
            // move script folder
            .route("/script_folder_move", put(put_script_folder_move::move_script_folder))
            // delete script_folder
            .route("/script_folder", delete(delete_script_folder::delete_script_folder))
            // create script
            .route("/script",post(post_script::create_script))
            // read script
            .route("/script/:script_id",get(get_script::read_script))
            // update script
            .route("/script",put(put_script::update_script))
            // move script
            .route("/script_move",put(put_script_move::move_script))
            // delete script
            .route("/script",delete(delete_script::delete_script))
            .with_state(Arc::new(AdminServiceState{
                admin_use_case:Arc::clone(&self.admin_use_case)
            }));
        router
    }
}


pub struct AdminServiceState {
    admin_use_case:Arc<AdminUseCase>
}