use crate::types::*;
use crate::logs::*;
use crate::read_from_buffer::*;

use std::io::{Error};
use std::sync::{Arc};
use std::process::{Stdio as StdStdio};

use tokio::{self, io::BufReader, process::Command, sync::Mutex};

pub async fn run_command(
    start_command: &Vec<String>,
    current_dir: String,
    with_logs: bool,
    log_annotations_for_service: Vec<LogAnnotation>,
    continue_on_log_regex: Option<String>,
) -> Result<(), Error> {
    let is_process_complete = Arc::new(Mutex::new(false));
    let is_process_complete_clone = is_process_complete.clone();
    let is_process_complete_clone_2 = is_process_complete.clone();

    let log_annotations_for_service_clone = log_annotations_for_service.clone();
    let log_annotations_for_service_clone_2 = log_annotations_for_service.clone();

    let continue_on_log_regex_clone = continue_on_log_regex.clone();
    let continue_on_log_regex_clone_2 = continue_on_log_regex.clone();

    let start_str = start_command.join(" ");

    log_running_command(&start_str);

    // actually start the service
    let args: Vec<String> = start_command[1..].to_vec();
    let mut child = Command::new(&start_command[0])
        .current_dir(current_dir.clone())
        .args(args)
        .stdout(StdStdio::piped())
        .stderr(StdStdio::piped())
        .spawn()
        .expect("error...");

    // TODO: error handling for below (they might not have any output)

    let child_stdout = child.stdout.take().expect("no stdout");
    let child_stdout_reader = BufReader::new(child_stdout);
    
    let child_stderr = child.stderr.take().expect("no stderr");
    let child_stderr_reader = BufReader::new(child_stderr);

    let start_command_clone = start_command.clone();
    let start_command_clone_2 = start_command.clone();

    let stdout_task = tokio::spawn(async move {
        let stdout_logs = log_output_from_reader(
            child_stdout_reader,
            is_process_complete_clone,
            with_logs,
            &start_command_clone[0],
            log_annotations_for_service_clone,
            continue_on_log_regex_clone,
            LogType::ProcessStdout
        );
        stdout_logs.await;
    });

    let stderr_task = tokio::spawn(async move {
        let stderr_logs = log_output_from_reader(
            child_stderr_reader,
            is_process_complete_clone_2,
            with_logs,
            &start_command_clone_2[0],
            log_annotations_for_service_clone_2,
            continue_on_log_regex_clone_2,
            LogType::ProcessStderr
        );
        stderr_logs.await;
    });
    
    
    let _a = stdout_task.await.unwrap();
    let _b = stderr_task.await.unwrap();

    Ok::<(), Error>(())
}