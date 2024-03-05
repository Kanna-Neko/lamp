use mongodb::{error::Error,Client,options::SessionOptions,ClientSession};
use std::sync::Arc;
mod project;
mod script_folder;
mod script;

const DATABASE: &'static str = "lamp";

pub use project::ProjectRepo;
pub use script_folder::ScriptFolderRepo;
pub use script::ScriptRepo;

pub struct TransactionClient {
    client: Arc<Client>
}

impl TransactionClient {
    pub fn new (client: Arc<Client>) -> TransactionClient {
        TransactionClient{
            client:Arc::clone(&client)
        }
    }
    pub async fn start_transaction(&self,option:Option<SessionOptions>) -> Result<ClientSession,Error> {
        self.client.start_session(option).await
    }
}