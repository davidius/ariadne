use crate::logs::*;
use crate::types::*;
use std::env;

/// Takes a service definition and ensures environment variables are set up for the service to run.
pub fn prepare_env(service: &Service) {
    log_status_update(&"Preparing environment".to_string(), &service.name);

    match service.service_run_config.env {
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
