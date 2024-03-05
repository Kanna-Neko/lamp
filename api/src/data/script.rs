use std::{sync::Arc, collections::HashMap};

use mongodb::{Client, ClientSession,error::Error, bson::{oid::ObjectId, doc, to_bson}, Collection};

use crate::models::Script;

use super::DATABASE;
const COLLECTION:&'static str = "script";

pub struct ScriptRepo {
    client: Arc<Client>
}

impl ScriptRepo {
    pub fn new(client: Arc<Client>) -> Self {
        Self {
            client
        }
    }

    fn colleciton(&self) -> Collection<Script>{
        self.client
            .database(DATABASE)
            .collection::<Script>(COLLECTION)
    }

    pub async fn read_one(&self, session: Option<&mut ClientSession>, script_id: &ObjectId) -> Result<Option<Script>,Error> {
        let collection = self.colleciton();
        let filter = doc! {
            "_id": script_id
        };
        match session {
            Some(session) => {
                collection.find_one_with_session(filter, None,session).await
            },
            None => {
                collection.find_one(filter, None).await
            }
        }
    }

    pub async fn delete_by_project_id(&self, session:Option<&mut ClientSession>,project_id: &ObjectId) -> Result<(),Error> {
        let collection = self.colleciton();
        let filter = doc! {
            "project_id": project_id
        };
        match session {
            Some(session) => {
                collection.delete_many_with_session(filter, None, session).await?;
            },
            None => {
                collection.delete_many(filter, None).await?;
            }
        }
        Ok(())
    }

    pub async fn create_one(&self, session:Option<&mut ClientSession>, script:&Script) -> Result<(),Error> {
        let collection = self.colleciton();
        match session {
            Some(session) => {
                collection.insert_one_with_session(script, None, session).await?;
            },
            None => {
                collection.insert_one(script,None).await?;
            }
        }
        Ok(())
    }

    pub async fn update_property_except_parent_folder_id(&self, session:Option<&mut ClientSession>, script_id: &ObjectId, repository_url:&str, repository_branch: &str, username: &str, password: &str, base_image: &str, command_before_run: &str, run_command: &str, env: &HashMap<String,String>) -> Result<(),Error> {
        let collection = self.colleciton();
        let query = doc! {
            "_id": script_id,
        };
        let update = doc! {
            "$set": {
                "repository_url": repository_url,
                "repository_branch": repository_branch,
                "username": username,
                "password": password,
                "base_image": base_image,
                "command_before_run": command_before_run,
                "run_command": run_command,
                "env": to_bson(env)?
            }
        };
        match session {
            Some(session) => {
                collection.update_one_with_session(query, update, None, session).await?;
            },
            None => {
                collection.update_one(query,update,None).await?;
            }
        }
        Ok(())
    }

    pub async fn update_parent_folder_id(&self, session: Option<&mut ClientSession>, script_id: &ObjectId, parent_folder_id: &ObjectId) -> Result<(),Error> {
        let collection = self.colleciton();
        let query = doc! {
            "_id": script_id
        };
        let update = doc! {
            "$set": {
                "parent_folder_id": parent_folder_id
            }
        };
        match session {
            Some(session) => {
                collection.update_one_with_session(query, update, None, session).await?;
            },
            None => {
                collection.update_one(query,update,None).await?;
            }
        }
        Ok(())
    }

    pub async fn delete_one(&self, session: Option<&mut ClientSession>, script_id: &ObjectId) -> Result<(),Error> {
        let collection = self.colleciton();
        let query = doc! {
            "_id": script_id
        };
        match session {
            Some(session) => {
                collection.delete_one_with_session(query, None, session).await?;
            },
            None => {
                collection.delete_one(query, None).await?;
            }
        }
        Ok(())
    }

    pub async fn delete_many(&self, session: Option<&mut ClientSession>, script_ids: Vec<&ObjectId>) -> Result<(),Error> {
        let collection = self.colleciton();
        let query = doc! {
            "_id": {
                "$in": script_ids
            }
        };
        match session {
            Some(session) => {
                collection.delete_many_with_session(query, None, session).await?;
            },
            None => {
                collection.delete_many(query, None).await?;
            }
        }
        Ok(())
    }
}
