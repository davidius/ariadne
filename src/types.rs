use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::oneshot;

#[derive(std::cmp::PartialEq)]
pub enum LogType {
    ProcessStdout,
    ProcessStderr,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserConfig {
    pub node_base_path: String,
    pub java_8_home: String,
    pub java_11_home: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServicesConfig {
    pub services: Vec<Service>,
    pub recipes: Vec<Recipe>,
}

pub trait ServicesConfigExt {
    fn is_empty(&self) -> bool;
}

impl ServicesConfigExt for ServicesConfig {
    fn is_empty(&self) -> bool {
        self.services.is_empty() && self.recipes.is_empty()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LogAnnotations {
    pub annotations: Vec<LogAnnotation>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LogAnnotation {
    pub annotation_type: String,
    pub regex: String,
    pub hint: String,
    pub affected_services: Vec<String>,
    pub links: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Service {
    pub name: String,
    pub service_run_config: ServiceRunConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServiceRunConfig {
    pub dir: String,
    pub pre_commands: Option<Vec<String>>,
    pub start_command: String,
    #[serde(default)]
    pub java_version: String,
    #[serde(default)]
    pub node_version: String,
    pub env: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Recipe {
    pub name: String,
    pub services: Vec<RecipeService>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RecipeService {
    pub name: String,
    pub runtype: String,
    pub continue_on_log_regex: Option<String>,
}

#[derive(Clone, Debug)]
pub struct MutexWrapper {
    pub is_process_complete_mutex: std::sync::Arc<std::sync::Mutex<bool>>,
}

pub trait MutexWrapperExt {
    fn get_is_process_complete(&self) -> bool;
    fn set_is_process_complete(&self, is_process_complete: bool);
}

impl MutexWrapperExt for MutexWrapper {
    fn get_is_process_complete(&self) -> bool {
        let try_lock_result = self.is_process_complete_mutex.lock().unwrap();
        let is_process_complete = *try_lock_result;
        return is_process_complete;
    }

    fn set_is_process_complete(&self, is_process_complete: bool) {
        let mut is_process_complete_new = self.is_process_complete_mutex.lock().unwrap();
        *is_process_complete_new = is_process_complete;
    }
}

#[derive(Clone, Debug)]
pub struct RunningService {
    pub running_process: std::sync::Arc<tokio::sync::Mutex<tokio::process::Child>>,
    pub service_name: String,
    pub is_process_complete: MutexWrapper,
    pub log_annotations_for_service: Vec<LogAnnotation>,
    pub continue_on_log_regex: Option<String>,
    pub status: ServiceStatus,
    pub is_superseded: bool,
}

#[derive(Clone, Debug, std::cmp::PartialEq)]
pub enum ServiceStatus {
    NotStarted,
    Running,
    Complete,
    Failed,
}

#[derive(Debug)]
pub enum RecipeCommand {
    GetRunningServiceByName {
        service_name: String,
        resp: oneshot::Sender<Option<RunningService>>,
    },
    GetServiceStatusByRecipeIndex {
        recipe_index_opt: Option<usize>,
        resp: oneshot::Sender<ServiceStatus>,
    },
    SetServiceStatus {
        service_name: String,
        status: ServiceStatus,
        resp: oneshot::Sender<()>,
    },
    GetIsServiceSupersededByRecipeIndex {
        recipe_index_opt: Option<usize>,
        resp: oneshot::Sender<bool>,
    },
    SetIsServiceSuperseded {
        service_name: String,
        is_superseded: bool,
        resp: oneshot::Sender<()>,
    },
    AddRunningService {
        running_service: RunningService,
    },
}
