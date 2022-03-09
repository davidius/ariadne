use crate::command_parse::*;
use crate::run_commands::*;
use crate::types::*;

pub async fn run_service(
    service: &Service,
    runtype: String,
    log_annotations_for_service: Vec<LogAnnotation>,
    continue_on_log_regex: Option<String>,
) {
    let start_command_str = &service.service_run_config.start_command;
    let start_command_vec = get_command_vec(&start_command_str);
    let current_dir = service.service_run_config.dir.clone();
    let with_logs: bool = runtype == "foreground";

    match &service.service_run_config.pre_commands {
        Some(pre_commands) => {
            for pre_command in pre_commands {
                let pre_command_vec = get_command_vec(&pre_command);
                let command_result = run_command(
                    &pre_command_vec,
                    current_dir.clone(),
                    with_logs,
                    log_annotations_for_service.clone(),
                    continue_on_log_regex.clone(),
                ).await;
            }
        }
        None => {
            // Do nothing
        }
    }

    let command_result = run_command(
        &start_command_vec,
        current_dir,
        with_logs,
        log_annotations_for_service,
        continue_on_log_regex,
    ).await;
}