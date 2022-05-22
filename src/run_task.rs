use crate::command_parse::*;
use crate::logs::log_status_update;
use crate::run_commands::*;
use crate::types::*;

pub async fn run_task(
    task: &Task,
    runtype: String,
    log_annotations_for_task: Vec<LogAnnotation>,
    continue_on_log_regex: Option<String>,
) -> RunningTask {
    log_status_update(&"Running task".to_string(), &task.name);
    let start_command_str = &task.task_run_config.start_command;
    let start_command_vec = get_command_vec(&start_command_str);
    let current_dir = task.task_run_config.dir.clone();
    let with_logs: bool = runtype == "foreground";

    // Note that the pre_commands should all be synchronous (at least currently)
    match &task.task_run_config.pre_commands {
        Some(pre_commands) => {
            for pre_command in pre_commands {
                let pre_command_vec = get_command_vec(&pre_command);
                run_command(
                    &pre_command_vec,
                    current_dir.clone(),
                    with_logs,
                    log_annotations_for_task.clone(),
                    continue_on_log_regex.clone(),
                    task.name.clone(),
                )
                .await;
            }
        }
        None => {
            // Do nothing
        }
    }

    let running_task = run_command(
        &start_command_vec,
        current_dir,
        with_logs,
        log_annotations_for_task,
        continue_on_log_regex,
        task.name.clone(),
    )
    .await;

    return running_task;
}
