use std::sync::Arc;

use futures::TryStreamExt;
use mongodb::{Client, ClientSession, error::Error, bson::{oid::ObjectId, doc}, Collection};
use crate::models::Project;
use super::DATABASE;

const COLLECTION:&'static str = "project";
pub struct ProjectRepo {
    client: Arc<Client>
}

impl ProjectRepo{
    pub fn new(client: Arc<Client>) -> Self {
        Self{
            client
        }
    }

    fn colleciton(&self) -> Collection<Project>{
        self.client
            .database(DATABASE)
            .collection::<Project>(COLLECTION)
    }

    pub async fn create_one(&self,session:Option<&mut ClientSession>,project: &Project) -> Result<(),Error>{
        let collection = self.colleciton();
        match session {
            Some(session)=> {
                collection.insert_one_with_session(project, None,session).await?;
            }
            None => {
                collection.insert_one(project, None).await?;
            }
        }
        Ok(())
    }

    pub async fn read_one(&self,session:Option<&mut ClientSession>, _id:&ObjectId) -> Result<Option<Project>,Error> {
        let collection = self.colleciton();
        let filter = doc! {
            "_id":_id
        };
        // let mut document:Option<Project>;
        match session {
            Some(session) => {
                collection.find_one_with_session(filter, None,session).await
            }
            None => {
                collection.find_one(filter, None).await
            }
        }
    }

    pub async fn delete_one(&self,session:Option<&mut ClientSession>, _id:&ObjectId) -> Result<(),Error> {
        let collection = self.colleciton();
        let filter = doc! {
            "_id":_id,
        };
        match session {
            Some(session) => {
                collection.delete_one_with_session(filter, None, session).await?;
                Ok(())
            },
            None => {
                collection.delete_one(filter, None).await?;
                Ok(())
            }
        }
    }

    pub async fn read_all(&self, session:Option<&mut ClientSession>) -> Result<Vec<Project>,Error> {
        let collection = self.colleciton();
        let filter = doc! {};
        match session {
            Some(session) => {
                let mut cursor = collection.find_with_session(filter, None, session).await?;
                let result:Vec<Project> = cursor.stream(session).try_collect().await?;
                Ok(result)
            },
            None => {
                let cursor = collection.find(filter,None).await?;
                let result:Vec<Project> = cursor.try_collect().await?;
                Ok(result)
            }
        }
    }

    pub async fn update_name_and_description(&self, session:Option<&mut ClientSession>,project_id: &ObjectId, name: &str, description: &str) -> Result<(),Error> {
        let collection = self.colleciton();
        let query = doc!{
            "_id":project_id
        };
        let update = doc!{
            "$set":doc! {
                "name":name,
                "description":description
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
