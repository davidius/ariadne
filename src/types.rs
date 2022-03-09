use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

pub struct MutexWrapper {
    pub is_process_complete_mutex: std::sync::Arc<std::sync::Mutex<bool>>,
}

pub trait MutexWrapperExt {
    fn get_is_process_complete(&self) -> bool;
}

impl MutexWrapperExt for MutexWrapper {
    fn get_is_process_complete(&self) -> bool {
        let try_lock_result = self.is_process_complete_mutex.lock().unwrap();
        let is_process_complete = *try_lock_result;
        return is_process_complete;
    }
}
