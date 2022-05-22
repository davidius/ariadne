use crate::logs::*;
use crate::types::*;
use std::env;

/// Takes a task definition and ensures environment variables are set up for the task to run.
pub fn prepare_env(task: &Task) {
    log_status_update(&"Preparing environment".to_string(), &task.name);

    match task.task_run_config.env {
        Some(ref env) => {
            env.into_iter().for_each(|(key, value)| {
                if !value.is_empty() {
                    env::set_var(key, value);
                    let new_env_value = env::var(key).unwrap();
                    log_new_env_variable(key, &new_env_value);
                }
            });
        }
        None => {
            log_no_env_variables();
        }
    }
}
