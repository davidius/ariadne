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
pub struct TasksConfig {
    pub tasks: Vec<Task>,
    pub recipes: Vec<Recipe>,
}

pub trait TasksConfigExt {
    fn is_empty(&self) -> bool;
}

impl TasksConfigExt for TasksConfig {
    fn is_empty(&self) -> bool {
        self.tasks.is_empty() && self.recipes.is_empty()
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
    pub affected_tasks: Vec<String>,
    pub links: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Task {
    pub name: String,
    pub task_run_config: TaskRunConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TaskRunConfig {
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
    pub tasks: Vec<RecipeTask>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RecipeTask {
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
pub struct RunningTask {
    pub running_process: std::sync::Arc<tokio::sync::Mutex<tokio::process::Child>>,
    pub task_name: String,
    pub is_process_complete: MutexWrapper,
    pub log_annotations_for_task: Vec<LogAnnotation>,
    pub continue_on_log_regex: Option<String>,
    pub status: TaskStatus,
    pub is_superseded: bool,
}

#[derive(Clone, Debug, std::cmp::PartialEq)]
pub enum TaskStatus {
    NotStarted,
    Running,
    Complete,
    Failed,
}

#[derive(Debug)]
pub enum RecipeCommand {
    GetRunningTaskByName {
        task_name: String,
        resp: oneshot::Sender<Option<RunningTask>>,
    },
    GetTaskStatusByRecipeIndex {
        recipe_index_opt: Option<usize>,
        resp: oneshot::Sender<TaskStatus>,
    },
    SetTaskStatus {
        task_name: String,
        status: TaskStatus,
        resp: oneshot::Sender<()>,
    },
    GetIsTaskSupersededByRecipeIndex {
        recipe_index_opt: Option<usize>,
        resp: oneshot::Sender<bool>,
    },
    SetIsTaskSuperseded {
        task_name: String,
        is_superseded: bool,
        resp: oneshot::Sender<()>,
    },
    AddRunningTask {
        running_task: RunningTask,
    },
}
