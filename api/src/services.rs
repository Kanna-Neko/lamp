mod admin;
use axum::Router;
use std::sync::Arc;
pub use admin::AdminService;

pub struct Services {
    admin_service: Arc<AdminService>
}

impl Services {
    pub fn new(admin_service: AdminService) ->Self {
        Self {
            admin_service:Arc::new(admin_service)
        }
    }
    pub fn router(&self) -> Router {
        let router = Router::new().nest("/api",Router::new()
            .nest("/admin",self.admin_service.router()));
        router
    }
}