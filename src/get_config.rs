use crate::parse_log_annotations_json;
use crate::parse_services_json;
use crate::parse_user_json;
use crate::LogAnnotations;
use crate::ServicesConfig;
use crate::UserConfig;

use dirs::home_dir;
use std::fs;

pub fn get_user_home_path() -> String {
    let user_home_path_buf = home_dir().unwrap();
    let user_home_path = user_home_path_buf.to_str().unwrap().to_string();
    return user_home_path;
}

pub fn get_services_config() -> ServicesConfig {
    let user_home_path = get_user_home_path();

    let service_json_path = format!("{}/.ariadne/services.json", user_home_path);
    let raw_services_json_result = fs::read_to_string(&service_json_path);

    let raw_services_json = match raw_services_json_result {
        Ok(raw_json) => raw_json,
        Err(_) => {
            panic!("Couldn't find a services config file. This should be stored in {}. You can create one with `ariadne setup`.", &service_json_path);
        }
    };

    let services_config = parse_services_json(raw_services_json);

    return services_config;
}

pub fn get_user_config() -> UserConfig {
    let user_home_path = get_user_home_path();

    let user_json_path = format!("{}/.ariadne/user.json", user_home_path);
    let raw_user_json_result = fs::read_to_string(&user_json_path);

    let raw_user_json = raw_user_json_result.unwrap_or(String::from("{}"));

    let user_config = parse_user_json(raw_user_json);

    return user_config;
}

pub fn get_log_annotations() -> LogAnnotations {
    let user_home_path = get_user_home_path();

    let log_annotations_json_path = format!("{}/.ariadne/log_annotations.json", user_home_path);
    let raw_log_annotations_json_result = fs::read_to_string(&log_annotations_json_path);

    let raw_log_annotations_json = raw_log_annotations_json_result.unwrap_or(String::from("{}"));
    let log_annotations = parse_log_annotations_json(raw_log_annotations_json);

    return log_annotations;
}
