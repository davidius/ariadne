use crate::parse_log_annotations_yaml;
use crate::parse_tasks_yaml;
use crate::parse_user_yaml;
use crate::LogAnnotations;
use crate::TasksConfig;
use crate::UserConfig;

use dirs::home_dir;
use std::fs;

pub fn get_user_home_path() -> String {
    let user_home_path_buf = home_dir().unwrap();
    let user_home_path = user_home_path_buf.to_str().unwrap().to_string();
    return user_home_path;
}

pub fn get_tasks_config() -> TasksConfig {
    let user_home_path = get_user_home_path();

    let tasks_yaml_path = format!("{}/.ariadne/tasks.yaml", user_home_path);
    let raw_tasks_yaml_result = fs::read_to_string(&tasks_yaml_path);

    let raw_tasks_yaml = raw_tasks_yaml_result.unwrap_or("---".to_string());

    let tasks_config = parse_tasks_yaml(raw_tasks_yaml);

    return tasks_config;
}

pub fn get_user_config() -> UserConfig {
    let user_home_path = get_user_home_path();

    let user_yaml_path = format!("{}/.ariadne/user.yaml", user_home_path);
    let raw_user_yaml_result = fs::read_to_string(&user_yaml_path);

    let raw_user_yaml = raw_user_yaml_result.unwrap_or(String::from("---"));

    let user_config = parse_user_yaml(raw_user_yaml);

    return user_config;
}

pub fn get_log_annotations() -> LogAnnotations {
    let user_home_path = get_user_home_path();

    let log_annotations_yaml_path = format!("{}/.ariadne/log_annotations.yaml", user_home_path);
    let raw_log_annotations_yaml_result = fs::read_to_string(&log_annotations_yaml_path);

    let raw_log_annotations_yaml = raw_log_annotations_yaml_result.unwrap_or(String::from("---"));
    let log_annotations = parse_log_annotations_yaml(raw_log_annotations_yaml);

    return log_annotations;
}

pub fn create_tasks_config_file() {
    let user_home_path = get_user_home_path();

    let tasks_config_file_path = format!("{}/.ariadne/tasks.yaml", user_home_path);

    let tasks_config_file_contents = String::from("---");
    let tasks_config_file_contents_buf = tasks_config_file_contents.as_bytes();

    fs::write(tasks_config_file_path, tasks_config_file_contents_buf).unwrap();
}
