use crate::types::*;

/// Parses the user.yaml file and retrieves its values. Consumes any errors that can happen while parsing, and in
/// case of any error just returns an empty UserConfig object.
pub fn parse_user_yaml(raw_user_yaml: String) -> UserConfig {
    let user_config_result = serde_yaml::from_str(&raw_user_yaml);
    match user_config_result {
        Ok(user_config) => user_config,
        Err(_) => UserConfig {
            node_base_path: "".to_string(),
            java_8_home: "".to_string(),
            java_11_home: "".to_string(),
        },
    }
}

/// Parses the tasks.yaml file and retrieves its values. Consumes any errors that can happen while parsing, and in
/// case of any error just returns an empty TasksConfig object.
pub fn parse_tasks_yaml(raw_tasks_yaml: String) -> TasksConfig {
    let tasks_config_result = serde_yaml::from_str(&raw_tasks_yaml);
    match tasks_config_result {
        Ok(tasks_config) => tasks_config,
        Err(_) => TasksConfig {
            tasks: Vec::new(),
            recipes: Vec::new(),
        },
    }
}

/// Parses the log_annotations.yaml file and retrieves its values. Consumes any errors that can happen while parsing, and in
/// case of any error just returns an empty LogAnnotations object.
pub fn parse_log_annotations_yaml(raw_log_annotations_yaml: String) -> LogAnnotations {
    let log_annotations_result = serde_yaml::from_str(&raw_log_annotations_yaml);
    match log_annotations_result {
        Ok(log_annotations) => log_annotations,
        Err(_) => LogAnnotations {
            annotations: Vec::new(),
        },
    }
}
