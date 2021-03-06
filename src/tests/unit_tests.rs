#[cfg(test)]
mod unit_tests {
    // use super::*;
    use crate::types::types::*;
    use crate::yaml_parse::*;

    #[test]
    fn parse_user_yaml_should_correctly_parse() {
        let raw_user_yaml = String::from(
            "---
            node_base_path: \"/Users/username/.nvm/versions/node\"
            java_11_home: \"/Library/Java/JavaVirtualMachines/adoptopenjdk-11.jdk/Contents/Home\"
            java_8_home: \"/Library/Java/JavaVirtualMachines/adoptopenjdk-8.jdk/Contents/Home\"
            ",
        );
        let target_user_config = UserConfig {
            node_base_path: String::from("/Users/username/.nvm/versions/node"),
            java_8_home: String::from(
                "/Library/Java/JavaVirtualMachines/adoptopenjdk-8.jdk/Contents/Home",
            ),
            java_11_home: String::from(
                "/Library/Java/JavaVirtualMachines/adoptopenjdk-11.jdk/Contents/Home",
            ),
        };

        let actual_user_config = parse_user_yaml(raw_user_yaml);
        assert_eq!(
            actual_user_config.java_8_home,
            target_user_config.java_8_home
        );
        assert_eq!(
            actual_user_config.java_11_home,
            target_user_config.java_11_home
        );
        assert_eq!(
            actual_user_config.node_base_path,
            target_user_config.node_base_path
        );
    }
}
