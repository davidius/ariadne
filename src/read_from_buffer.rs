use crate::log_annotations::*;
use crate::logs::*;
use crate::types::types::*;
use regex::Regex;
use tokio::io::{AsyncBufReadExt, AsyncRead, BufReader};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

pub async fn watch_logs_for_task(
    chef_sender: tokio::sync::mpsc::Sender<RecipeCommand>,
    task_name: &String,
    with_logs: bool,
    log_annotations_for_task: Vec<LogAnnotation>,
    continue_on_log_regex: Option<String>,
) {
    let (resp_tx, resp_rx) = oneshot::channel();

    chef_sender
        .send(RecipeCommand::GetRunningTaskByName {
            task_name: task_name.clone(),
            resp: resp_tx,
        })
        .await
        .unwrap();
    let chef_sender_clone = chef_sender.clone();
    let chef_sender_clone_2 = chef_sender.clone();

    let running_task_opt = resp_rx.await.unwrap();

    match running_task_opt {
        Some(running_task) => {
            let mut rsp_lock = running_task.running_process.lock().await;

            let child_stdout = (*&mut rsp_lock).stdout.take().expect("no stdout");
            let child_stdout_reader = BufReader::new(child_stdout);

            let child_stderr = (*&mut rsp_lock).stderr.take().expect("no stderr");
            let child_stderr_reader = BufReader::new(child_stderr);

            let task_name_clone = task_name.clone();
            let task_name_clone_2 = task_name.clone();

            let continue_on_log_regex_clone = continue_on_log_regex.clone();
            let continue_on_log_regex_clone_2 = continue_on_log_regex.clone();

            let log_annotations_for_task_clone = log_annotations_for_task.clone();
            let log_annotations_for_task_clone_2 = log_annotations_for_task.clone();

            let mut stdout_task_join_handle: JoinHandle<bool> = tokio::spawn(async move {
                log_output_from_reader(
                    chef_sender.clone(),
                    child_stdout_reader,
                    with_logs,
                    &task_name_clone,
                    &log_annotations_for_task_clone,
                    continue_on_log_regex_clone,
                    LogType::ProcessStdout,
                )
                .await;
                return true;
            });

            let mut stderr_task_join_handle: JoinHandle<bool> = tokio::spawn(async move {
                log_output_from_reader(
                    chef_sender_clone,
                    child_stderr_reader,
                    with_logs,
                    &task_name_clone_2,
                    &log_annotations_for_task_clone_2,
                    continue_on_log_regex_clone_2,
                    LogType::ProcessStderr,
                )
                .await;
                return true;
            });

            let join_result =
                tokio::try_join!(&mut stdout_task_join_handle, &mut stderr_task_join_handle);

            let task_name_clone_3 = task_name.clone();

            match join_result {
                Ok(_) => {
                    let (resp_tx, _) = oneshot::channel();
                    chef_sender_clone_2
                        .send(RecipeCommand::SetTaskStatus {
                            task_name: task_name_clone_3.clone(),
                            status: TaskStatus::Running,
                            resp: resp_tx,
                        })
                        .await
                        .unwrap();
                }
                Err(_err) => {
                    // TODO: handle error
                }
            }
        }
        None => {
            // TODO: handle this scenario
            return;
        }
    }
}

/// Reads from a buffer and logs the output
pub async fn log_output_from_reader(
    chef_sender: tokio::sync::mpsc::Sender<RecipeCommand>,
    log_reader: BufReader<impl AsyncRead + std::marker::Unpin>,
    with_logs: bool,
    task_name: &String,
    log_annotations_for_task: &Vec<LogAnnotation>,
    continue_on_log_regex: Option<String>,
    log_type: LogType,
) {
    let mut lines = log_reader.lines();

    let (resp_tx, _) = oneshot::channel();

    while let Some(log_line) = lines.next_line().await.unwrap() {
        if with_logs {
            log_output(&log_line, &log_type, task_name);
        }

        parse_for_matching_log_annotations(&log_line, log_annotations_for_task.to_owned());

        if continue_on_log_regex.is_some() {
            let regex_string = format!("{}", continue_on_log_regex.as_ref().unwrap());
            // TODO: use the static syntax below to prevent re-compiling the regex
            let re = Regex::new(&regex_string).unwrap(); // TODO: should handle errors (i.e. if provided regex is invalid)
            if re.is_match(&log_line) {
                log_continue_to_next_process();
                chef_sender
                    .send(RecipeCommand::SetIsTaskSuperseded {
                        task_name: task_name.to_owned(),
                        is_superseded: true,
                        resp: resp_tx,
                    })
                    .await;

                // TODO: technically we should not break here, since this task may still have log output for an undefined
                // period of time.
                break;
            }
        }
    }
}
