use crate::logs::*;
use crate::types::types::*;

use std::process::Stdio as StdStdio;
use std::sync::Arc;

use tokio::{self, process::Command, sync::Mutex};

/// Runs the specified start_command and returns a `RunningTask` object representing the task it started.
pub async fn run_command(
    start_command: &Vec<String>,
    current_dir: String,
    with_logs: bool,
    log_annotations_for_task: Vec<LogAnnotation>,
    continue_on_log_regex: Option<String>,
    associated_task_name: String,
) -> RunningTask {
    let is_process_complete = Arc::new(std::sync::Mutex::new(false));

    let is_process_complete_mutex_wrapper = MutexWrapper {
        is_process_complete_mutex: is_process_complete.clone(),
    };

    let log_annotations_for_task_clone = log_annotations_for_task.clone();

    let continue_on_log_regex_clone = continue_on_log_regex.clone();

    let start_str = start_command.join(" ");

    let is_superseded = false;

    log_running_command(&start_str);

    // actually start the task
    let args: Vec<String> = start_command[1..].to_vec();
    let child = Command::new(&start_command[0])
        .current_dir(current_dir.clone())
        .args(args)
        .stdout(StdStdio::piped())
        .stderr(StdStdio::piped())
        .spawn()
        .expect("error...");

    return RunningTask {
        running_process: Arc::new(Mutex::new(child)),
        task_name: associated_task_name,
        is_process_complete: is_process_complete_mutex_wrapper,
        log_annotations_for_task: log_annotations_for_task_clone,
        continue_on_log_regex: continue_on_log_regex_clone,
        status: TaskStatus::Running,
        is_superseded,
    };
}
