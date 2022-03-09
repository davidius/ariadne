use crate::logs::*;
use crate::types::*;
use std::env;

/// Takes a service definition and ensures environment variables are set up for the service to run.
pub fn prepare_env(service: &Service, user_config: &UserConfig) {
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

    match service.service_run_config.java_version.as_str() {
        "8" => {
            env::set_var("JAVA_HOME", &user_config.java_8_home);
            let new_java_home = env::var("JAVA_HOME").unwrap();
            log_new_env_variable("JAVA_HOME", &new_java_home);
            // TODO: only do the following if the user has requested debug mode
            env::set_var(
                "JAVA_OPTS",
                "-agentlib:jdwp=transport=dt_socket,server=y,suspend=n,address=*:5005",
            );
            let new_java_opts = env::var("JAVA_OPTS").unwrap();
            log_new_env_variable("JAVA_OPTS", &new_java_opts);
        }
        "11" => {
            env::set_var("JAVA_HOME", &user_config.java_11_home);
            let new_java_home = env::var("JAVA_HOME").unwrap();
            log_new_env_variable("JAVA_HOME", &new_java_home);
            // TODO: only do the following if the user has requested debug mode
            env::set_var(
                "JAVA_OPTS",
                "-agentlib:jdwp=transport=dt_socket,server=y,suspend=n,address=*:5005",
            );
            let new_java_opts = env::var("JAVA_OPTS").unwrap();
            log_new_env_variable("JAVA_OPTS", &new_java_opts);
        }
        &_ => {
            // do nothing
        }
    }

    if !service.service_run_config.node_version.as_str().is_empty() {
        println!("--- setting up node... ---");
        let full_node_path = format!(
            "{}/versions/node/v{}/bin",
            &user_config.node_base_path, service.service_run_config.node_version
        );
        env::set_var("NVM_BIN", &full_node_path);
        let new_nvm_bin = env::var("NVM_BIN").unwrap();
        log_new_env_variable("NVM_BIN", &new_nvm_bin);
        // TODO: figure this out
    }
}
