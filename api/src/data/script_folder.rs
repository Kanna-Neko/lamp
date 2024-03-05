use std::{sync::Arc, collections::HashMap};
use mongodb::{Client,error::Error,ClientSession, bson::{oid::ObjectId, doc, to_bson}, Collection};
use super::DATABASE;
use crate::models::ScriptFolder;
use futures::stream::TryStreamExt;

const COLLECTION: &'static str = "script_folder";
pub struct ScriptFolderRepo {
    client: Arc<Client>
}

impl ScriptFolderRepo {
    pub fn new(client: Arc<Client>) -> Self {
        Self{
            client
        }
    }

    fn colleciton(&self) -> Collection<ScriptFolder>{
        self.client
            .database(DATABASE)
            .collection::<ScriptFolder>(COLLECTION)
    }

    pub async fn read_one(&self,session:Option<&mut ClientSession>,script_folder_id: &ObjectId) -> Result<Option<ScriptFolder>,Error> {
        let collection = self.colleciton();
        let filter = doc! {
            "_id": script_folder_id
        };
        match session {
            Some(session) => {
                collection.find_one_with_session(filter, None, session).await
            },
            None => {
                collection.find_one(filter, None).await
            }
        }
    }

    pub async fn create_one(&self,session:Option<&mut ClientSession>,script_folder: &ScriptFolder) -> Result<(),Error> {
        let collection = self.colleciton();
        match session {
            Some(session)=> {
                collection.insert_one_with_session(script_folder, None,session).await?;
            }
            None => {
                collection.insert_one(script_folder, None).await?;
            }
        }
        Ok(())
    }

    pub async fn delete_many(&self, session:Option<&mut ClientSession>, script_folder_ids: Vec<&ObjectId>) -> Result<(),Error> {
        let collection = self.colleciton();
        let query = doc! {
            "_id": {
                "$in": script_folder_ids
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

    pub async fn delete_by_project_id(&self, session:Option<&mut ClientSession>, project_id:&ObjectId) -> Result<(),Error> {
        let collection = self.colleciton();
        let filter = doc! {
            "project_id": project_id,
        };
        match session {
            Some(session) => {
                collection.delete_many_with_session(filter, None, session).await?;
                Ok(())
            },
            None => {
                collection.delete_many(filter, None).await?;
                Ok(())
            }
        }
    }

    pub async fn read_by_projetc_id(&self,session:Option<&mut ClientSession>, project_id: &ObjectId) -> Result<Vec<ScriptFolder>,Error> {
        let collection = self.colleciton();
        let filter = doc! {
            "project_id": project_id,
        };
        match session {
            Some(session) => {
                let mut cursor= collection.find_with_session(filter,None,session).await?;
                let results:Vec<ScriptFolder> = cursor.stream(session).try_collect().await?;
                Ok(results)
            }
            None => {
                let cursor = collection.find(filter,None).await?;
                let results:Vec<ScriptFolder> = cursor.try_collect().await?;
                Ok(results)
            }
        }
    }

    pub async fn add_script(&self, session:Option<&mut ClientSession>, script_folder_id: &ObjectId, script_id: &ObjectId) -> Result<(),Error> {
        let collection = self.colleciton();
        let query = doc! {
            "_id":script_folder_id,
        };
        let update = doc!{
            "$addToSet": {
                "script_id": script_id
            }
        };
        match session {
            Some(session) => {
                collection.update_one_with_session(query, update, None, session).await?;
            },
            None => {
                collection.update_one(query, update, None).await?;
            }
        }
        Ok(())
    }

    pub async fn add_child_script_folder(&self, session:Option<&mut ClientSession>,script_folder_id: &ObjectId, child_script_folder_id: &ObjectId) -> Result<(),Error> {
        let collection = self.colleciton();
        let query = doc! {
            "_id":script_folder_id,
        };
        let update = doc!{
            "$addToSet": {
                "child_folder_id": child_script_folder_id
            }
        };
        match session {
            Some(session) => {
                collection.update_one_with_session(query, update, None, session).await?;
            },
            None => {
                collection.update_one(query, update, None).await?;
            }
        }
        Ok(())
    }

    pub async fn update_parent_folder_id(&self, session:Option<&mut ClientSession>, script_folder_id: &ObjectId, destination_folder_id: &ObjectId) -> Result<(),Error> {
        let collection = self.colleciton();
        let query = doc! {
            "_id": script_folder_id
        };
        let update = doc! {
            "$set": {
                "parent_folder_id": destination_folder_id
            }
        };
        match session {
            Some(session) => {
                collection.update_one_with_session(query, update, None, session).await?;
            },
            None => {
                collection.update_one(query, update, None).await?;
            }
        }
        Ok(())
    }

    pub async fn update_name_and_description_and_env(&self, session:Option<&mut ClientSession>,script_folder_id: &ObjectId,name:&str, description:&str, env:&HashMap<String,String>) -> Result<(),Error> {
        let collection = self.colleciton();
        let query = doc! {
            "_id":script_folder_id
        };
        let update = doc!{
            "$set": {
                "name":name,
                "description":description,
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

    pub async fn remove_script(&self,session:Option<&mut ClientSession>, script_folder_id: &ObjectId, script_id:&ObjectId) -> Result<(),Error> {
        let collection = self.colleciton();
        let query = doc! {
            "_id": script_folder_id
        };
        let update = doc! {
            "$pull": {
                "script_id": script_id
            }
        };
        match session {
            Some(session) => {
                collection.update_one_with_session(query, update, None, session).await?;
            },
            None => {
                collection.update_one(query, update, None).await?;
            }
        }
        Ok(())
    }

    pub async fn remove_child_folder(&self, session:Option<&mut ClientSession>, script_folder_id: &ObjectId, child_folder_id: &ObjectId) -> Result<(),Error> {
        let collection = self.colleciton();
        let query = doc! {
            "_id": script_folder_id
        };
        let update = doc! {
            "$pull": {
                "child_folder_id": child_folder_id
            }
        };
        match session {
            Some(session) => {
                collection.update_one_with_session(query, update, None, session).await?;
            },
            None => {
                collection.update_one(query, update, None).await?;
            }
        }
        Ok(())
    }
}
