use std::collections::HashMap;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project{
    pub _id:ObjectId,
    pub name:String,
    pub description:String,
    pub root_folder_id:ObjectId,   
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptFolder {
    pub project_id:ObjectId,
    pub _id:ObjectId,
    pub name:String,
    pub description:String,
    pub parent_folder_id:Option<ObjectId>,
    pub child_folder_id:Vec<ObjectId>,
    pub script_id:Vec<ObjectId>,
    pub env:HashMap<String,String>
}

#[derive(Serialize,Deserialize)]
pub struct Script {
    pub _id:ObjectId,
    pub project_id: ObjectId,
    pub parent_folder_id:ObjectId,
    pub env:HashMap<String,String>
}