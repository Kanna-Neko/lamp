use std::{sync::Arc, collections::HashMap};
use mongodb::bson::oid::ObjectId;
use crate::{error::Error, models::{ScriptFolder, Script}, data::{ScriptFolderRepo, ScriptRepo}};
use futures::future::FutureExt;

use crate::{data::{ProjectRepo,TransactionClient}, models::Project};
use crate::error::ErrorType;
#[derive(Clone)]
pub struct AdminUseCase {
    transaction_client: Arc<TransactionClient>,
    project_repo: Arc<ProjectRepo>,
    script_folder_repo:Arc<ScriptFolderRepo>,
    script_repo:Arc<ScriptRepo>,
}

impl AdminUseCase {
    pub fn new(transaction_client: Arc<TransactionClient>,project_repo: Arc<ProjectRepo>,script_folder_repo:Arc<ScriptFolderRepo>,script_repo:Arc<ScriptRepo>) -> Self {
        Self {
            transaction_client,
            project_repo,
            script_folder_repo,
            script_repo,
        }
    }

    pub async fn create_project(&self, project_name: &str) -> Result<(Project,ScriptFolder),Error>{
        let mut session = self.transaction_client.start_transaction(None).await?;
        let project = Project{
            _id:ObjectId::new(),
            name: project_name.to_string(),
            description: "".to_string(),
            root_folder_id: ObjectId::new(),
        };
        let script_folder = ScriptFolder{
            project_id:project._id,
            _id:project.root_folder_id,
            name:String::from("root"),
            description:String::new(),
            parent_folder_id:None,
            child_folder_id:Vec::new(),
            script_id:Vec::new(),
            env:HashMap::new(),
        };
        session.with_transaction((&project,&script_folder), |session,(project,script_folder)| async move{
            self.project_repo.create_one(Some(session),&project).await?;
            self.script_folder_repo.create_one(Some(session),&script_folder).await?;
            Ok(())
        }.boxed(), None).await?;
        Ok((project,script_folder))
    }
    
    pub async fn read_project(&self, project_id: &str) -> Result<Project,Error> {
        let object_id = ObjectId::parse_str(project_id)?;
        let project = self.project_repo.read_one(None, &object_id).await?;
        match project {
            Some(project) => {
                Ok(project)
            },
            None => {
                Err(Error{
                    error_type: ErrorType::NotFoundRecordError,
                    reason:format!("record is not found, the project id is {project_id}")
                })
            }
        }
    }

    pub async fn delete_project(&self, project_id: &str) -> Result<(),Error> {
        let project_id = ObjectId::parse_str(project_id)?;
        let mut session = self.transaction_client.start_transaction(None).await?;
        session.with_transaction(&project_id, |session,project_id| async move{
            self.project_repo.delete_one(Some(session), project_id).await?;
            self.script_folder_repo.delete_by_project_id(Some(session), project_id).await?;
            self.script_repo.delete_by_project_id(Some(session), project_id).await?;
            Ok(())
        }.boxed(), None).await?;
        Ok(())
    }

    pub async fn create_script_folder(&self, project_id:&str, parent_script_folder_id:&str, name: &str) -> Result<ScriptFolder,Error> {
        let project_id = ObjectId::parse_str(project_id)?;
        let parent_script_folder_id = ObjectId::parse_str(parent_script_folder_id)?;
        let mut session = self.transaction_client.start_transaction(None).await?;
        let script_folder = session.with_transaction((&project_id,&parent_script_folder_id,&name), |session,(project_id, parent_script_folder_id,name)| async move {
            let parent_script_folder = self.script_folder_repo.read_one(Some(session), &parent_script_folder_id).await?;
            match parent_script_folder {
                None => {
                    return Err(mongodb::error::Error::custom(Error{
                        error_type: ErrorType::NotFoundRecordError,
                        reason: format!("not found parent script folder in project, the parent script folder id is {}, the project id is {}",parent_script_folder_id.to_hex(),project_id.to_hex()),
                    }));
                },
                Some(parent_script_folder) => {
                    if parent_script_folder.project_id.to_hex() != project_id.to_hex() {
                        return Err(mongodb::error::Error::custom(Error{
                            error_type: ErrorType::ProjectNotSameError,
                            reason: String::from("project is not same with parent script folder")
                        }))
                    }
                    let script_folder = ScriptFolder{
                        project_id: project_id.clone(),
                        _id:ObjectId::new(),
                        name:name.to_string(),
                        description:String::new(),
                        parent_folder_id: Some(parent_script_folder_id.clone()),
                        child_folder_id: Vec::new(),
                        script_id: Vec::new(),
                        env: HashMap::new(),
                    };
                    self.script_folder_repo.create_one(Some(session), &script_folder).await?;
                    self.script_folder_repo.add_child_script_folder(Some(session), &parent_script_folder_id, &script_folder._id).await?;
                    Ok(script_folder)
                },
            }
        }.boxed(), None).await?;
        Ok(script_folder)
    }

    pub async fn read_script_folder_by_project(&self, project_id:&str) -> Result<Vec<ScriptFolder>,Error> {
        let project_id = ObjectId::parse_str(project_id)?;
        match self.script_folder_repo.read_by_projetc_id(None, &project_id).await {
            Ok(results) => {
                Ok(results)
            },
            Err(err) => {
                Err(err.into())
            }
        }
    }

    pub async fn read_project_all(&self) -> Result<Vec<Project>,Error> {
        let result = self.project_repo.read_all(None).await?;
        Ok(result)
    }

    pub async fn create_script(&self, project_id:String, parent_folder_id:String, env: HashMap<String,String>) -> Result<Script,Error> {
        let project_id = ObjectId::parse_str(project_id)?;
        let parent_folder_id = ObjectId::parse_str(parent_folder_id)?;
        let script = Script {
            _id: ObjectId::new(),
            project_id,
            parent_folder_id,
            env
        };
        let mut session = self.transaction_client.start_transaction(None).await?;
        session.with_transaction((&project_id,&script), |session,(project_id,script)| async {
            let parent_folder = self.script_folder_repo.read_one(Some(session), &parent_folder_id).await?;
            match parent_folder {
                Some(parent_folder) => {
                    if parent_folder.project_id.to_hex() != project_id.to_hex() {
                        return Err(mongodb::error::Error::custom(Error{
                            error_type: ErrorType::ProjectNotSameError,
                            reason: format!("the project is not same with parent folder, the project id is {}, project id of parent folder is {}",project_id.to_hex(),parent_folder.project_id.to_hex())
                        }))
                    }
                },
                None => {
                    return Err(mongodb::error::Error::custom(Error{
                        error_type: ErrorType::NotFoundRecordError,
                        reason: format!("not found script folder, the script_folder id is {}",parent_folder_id.to_hex())
                    }))
                }
            }
            self.script_repo.create_one(Some(session), script).await?;
            self.script_folder_repo.add_script(Some(session), &script.parent_folder_id, &script._id).await?;
            Ok(())
        }.boxed(), None).await?;
        Ok(script)
    }

    pub async fn read_script(&self, script_id: String) -> Result<Script,Error> {
        let script_id = ObjectId::parse_str(script_id)?;
        let script = self.script_repo.read_one(None, &script_id).await?;
        match script {
            Some(script) => {
                Ok(script)
            }
            None => {
                Err(Error{
                    error_type:ErrorType::NotFoundRecordError,
                    reason: format!("script not found, the script id is {}",script_id)
                })
            }
        }
    }
    
    pub async fn update_project(&self,project_id: &str, name:&str, description:&str) -> Result<(),Error> {
        let project_id = ObjectId::parse_str(project_id)?;
        self.project_repo.update_name_and_description(None, &project_id, name, description).await?;
        Ok(())
    }

    pub async fn update_script_folder(&self,script_folder_id: &str ,name:&str, description:&str, env:&HashMap<String,String>) -> Result<(),Error> {
        let script_folder_id = ObjectId::parse_str(script_folder_id)?;
        self.script_folder_repo.update_name_and_description_and_env(None,&script_folder_id, name, description, env).await?;
        Ok(())
    }

    pub async fn update_script(&self, script_id: &str, repository_url:&str, repository_branch: &str, username: &str, password: &str, base_image: &str, command_before_run: &str, run_command: &str, env: &HashMap<String,String>) -> Result<(),Error> {
        let script_id = ObjectId::parse_str(script_id)?;
        self.script_repo.update_property_except_parent_folder_id(None, &script_id, repository_url, repository_branch, username, password, base_image, command_before_run, run_command, env).await?;
        Ok(())
    }

    pub async fn move_script(&self, script_id: &str, destination_folder_id:&str) -> Result<(),Error> {
        let script_id = ObjectId::parse_str(script_id)?;
        let destination_folder_id = ObjectId::parse_str(destination_folder_id)?;
        let mut session = self.transaction_client.start_transaction(None).await?;
        session.with_transaction((&script_id,&destination_folder_id), |session,(script_id, destination_folder_id)| async {
            if let Some(script) = self.script_repo.read_one(Some(session), script_id).await? {
                if script.parent_folder_id.to_hex() == destination_folder_id.to_hex() {
                    return Ok(())
                }
                self.script_repo.update_parent_folder_id(Some(session), script_id, destination_folder_id).await?;
                self.script_folder_repo.add_script(Some(session), destination_folder_id, script_id).await?;
                self.script_folder_repo.remove_script(Some(session), &script.parent_folder_id, script_id).await?;
            } else {
                return Err(mongodb::error::Error::custom(Error{
                    error_type: ErrorType::NotFoundRecordError,
                    reason: format!("not found script, the script id is {}",script_id.to_hex()),
                }))
            }
            Ok(())
        }.boxed(), None).await?;
        Ok(())
    }

    pub async fn move_script_folder(&self, script_folder_id: &str, destination_folder_id:&str) -> Result<(),Error> {
        let script_folder_id = ObjectId::parse_str(script_folder_id)?;
        let destination_folder_id = ObjectId::parse_str(destination_folder_id)?;
        let mut session = self.transaction_client.start_transaction(None).await?;
        session.with_transaction((&script_folder_id,&destination_folder_id), |session,(script_folder_id,destination_folder_id)| async {
            let script_folder = self.script_folder_repo.read_one(Some(session), script_folder_id).await?;
            let parent_script_folder = self.script_folder_repo.read_one(Some(session), destination_folder_id).await?;
            if let Some(script_folder) = script_folder {
                if let Some(parent_script_folder) = parent_script_folder {
                    if parent_script_folder.project_id.to_hex() != script_folder.project_id.to_hex() {
                        return Err(mongodb::error::Error::custom(Error{
                            error_type: ErrorType::ProjectNotSameError,
                            reason: format!("the project is not same with destination script folder")
                        }))
                    }
                }else {
                    return Err(mongodb::error::Error::custom(Error{
                        error_type: ErrorType::NotFoundRecordError,
                        reason: format!("not found script folder, the folder id is {}", destination_folder_id.to_hex())
                    }))
                }
                if let Some(parent_folder_id) = script_folder.parent_folder_id {
                    if parent_folder_id.to_hex() == destination_folder_id.to_hex() {
                        return Ok(())
                    }else {
                        self.script_folder_repo.update_parent_folder_id(Some(session), script_folder_id, destination_folder_id).await?;
                        self.script_folder_repo.remove_child_folder(Some(session), &parent_folder_id, script_folder_id).await?;
                        self.script_folder_repo.add_child_script_folder(Some(session), destination_folder_id, script_folder_id).await?;
                    }
                }else {
                    return Err(mongodb::error::Error::custom(Error{
                        error_type: ErrorType::MoveRootScriptFolderError,
                        reason: format!("can't move root script folder, the folder id is {}", script_folder_id.to_hex())
                    }))
                }
            }else {
                return Err(mongodb::error::Error::custom(Error{
                    error_type: ErrorType::NotFoundRecordError,
                    reason: format!("not found script folder, the folder id is {}", script_folder_id.to_hex())
                }))
            }
            Ok(())
        }.boxed(), None).await?;
        Ok(())
    }

    pub async fn delete_script(&self, script_id: &str) -> Result<(),Error> {
        let script_id = ObjectId::parse_str(script_id)?;
        let mut session = self.transaction_client.start_transaction(None).await?;
        session.with_transaction(&script_id, |session,script_id| async {
            let script = self.script_repo.read_one(Some(session), script_id).await?;
            if let Some(script) = script{
                self.script_repo.delete_one(Some(session),script_id).await?;
                self.script_folder_repo.remove_script(Some(session), &script.parent_folder_id, script_id).await?;
            }else {
                return Err(mongodb::error::Error::custom(Error{
                    error_type:ErrorType::NotFoundRecordError,
                    reason: format!("not found script, the script id is {}", script_id.to_hex())
                }));
            }
            Ok(())
        }.boxed(), None).await?;
        Ok(())
    }

    pub async fn delete_script_folder(&self, script_folder_id: &str) -> Result<(),Error> {
        let script_folder_id = ObjectId::parse_str(script_folder_id)?;
        let mut session = self.transaction_client.start_transaction(None).await?;
        session.with_transaction(&script_folder_id, |session,script_folder_id| async {
            let mut delete_script_folder_ids:Vec<ObjectId> = vec![script_folder_id.clone()];
            let mut delete_script_folder_stack:Vec<ObjectId> = vec![script_folder_id.clone()];
            let mut delete_script_ids:Vec<ObjectId> = Vec::new();
            let mut is_first = true;
            while let Some(script_folder_id) = delete_script_folder_stack.pop() {
                let script_folder = self.script_folder_repo.read_one(Some(session), &script_folder_id).await?;
                if let Some(script_folder) = script_folder {
                    if is_first {
                        if let Some(parent_folder_id) = script_folder.parent_folder_id {
                            self.script_folder_repo.remove_child_folder(Some(session), &parent_folder_id, &script_folder._id).await?;
                        }else {
                            return Err(mongodb::error::Error::custom(Error{
                                error_type: ErrorType::DeleteRootScriptFolderError,
                                reason: format!("can't delete root folder")
                            }))
                        }
                        is_first = false;
                    }
                    delete_script_folder_ids.extend_from_slice(&script_folder.child_folder_id);
                    delete_script_folder_stack.extend_from_slice(&script_folder.child_folder_id);
                    delete_script_ids.extend_from_slice(&script_folder.script_id);
                }else {
                    continue;
                }
            }
            let delete_script_folder_ids = delete_script_folder_ids.iter().collect();
            self.script_folder_repo.delete_many(Some(session), delete_script_folder_ids).await?;
            let delete_script_ids = delete_script_ids.iter().collect();
            self.script_repo.delete_many(Some(session), delete_script_ids).await?;
            Ok(())
        }.boxed(), None).await?;
        Ok(())
    }
}