use crate::types::*;

/// Parses the user.json file and retrieves its values. Consumes any errors that can happen while parsing, and in
/// case of any error just returns an empty UserConfig object.
pub fn parse_user_json(raw_user_json: String) -> UserConfig {
    let user_config_result = serde_json::from_str(&raw_user_json);
    match user_config_result {
        Ok(user_config) => user_config,
        Err(_) => UserConfig {
            node_base_path: "".to_string(),
            java_8_home: "".to_string(),
            java_11_home: "".to_string(),
        },
    }
}

/// Parses the services.json file and retrieves its values. Consumes any errors that can happen while parsing, and in
/// case of any error just returns an empty ServicesConfig object.
pub fn parse_services_json(raw_services_json: String) -> ServicesConfig {
    let services_config_result = serde_json::from_str(&raw_services_json);
    match services_config_result {
        Ok(services_config) => services_config,
        Err(_) => ServicesConfig {
            services: Vec::new(),
            recipes: Vec::new(),
        },
    }
}

/// Parses the log_annotations.json file and retrieves its values. Consumes any errors that can happen while parsing, and in
/// case of any error just returns an empty LogAnnotations object.
pub fn parse_log_annotations_json(raw_log_annotations_json: String) -> LogAnnotations {
    let log_annotations_result = serde_json::from_str(&raw_log_annotations_json);
    match log_annotations_result {
        Ok(log_annotations) => log_annotations,
        Err(_) => LogAnnotations {
            annotations: Vec::new(),
        },
    }
}
