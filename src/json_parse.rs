use crate::types::*;

/// Parses the user.json file and retrieves its values
pub fn parse_user_json(raw_user_json: String) -> UserConfig {
    let user_config: UserConfig = serde_json::from_str(&raw_user_json).unwrap();
    return user_config;
}

/// Parses the services.json file and retrieves its values
pub fn parse_services_json(raw_services_json: String) -> ServicesConfig {
    let services_config: ServicesConfig = serde_json::from_str(&raw_services_json).unwrap();
    return services_config;
}

/// Parses the log_annotations.json file and retrieves its values
pub fn parse_log_annotations_json(raw_log_annotations_json: String) -> LogAnnotations {
    let log_annotations: LogAnnotations = serde_json::from_str(&raw_log_annotations_json).unwrap();
    return log_annotations;
}
