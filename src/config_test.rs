use super::*;

#[cfg(test)]
mod find_config_file_tests {
    use super::*;

    #[test]
    fn returns_error_on_io_error() {
        let exp_error = "Fatal error encountered while looking for existing config";
        let file_exists = |_path: &str| Err(());
        let act = find_config_file("", file_exists);
        assert_eq!(act, Err(String::from(exp_error)));
    }

    #[test]
    fn returns_correct_message_on_no_file_found() {
        let exp_message = "No config file found";
        let file_exists = |_path: &str| Ok(false);
        let act = find_config_file("", file_exists);
        assert_eq!(act.unwrap(), String::from(exp_message));
    }

    #[test]
    fn returns_correct_path_when_file_found() {
        let root_dir = "/usr/me/repos/foo";
        let found_file = "rusty-hook.toml";
        let exp_path = format!("{}/{}", root_dir, found_file);
        let file_exists = |path: &str| {
            if path == exp_path {
                return Ok(true);
            }
            Ok(false)
        };
        let act = find_config_file(root_dir, file_exists);
        assert_eq!(act.unwrap(), exp_path);
    }
}

#[cfg(test)]
mod create_default_config_file_tests {
    use super::*;

    #[test]
    fn creates_config_with_default_name() {
        let root_dir = "/usr/mine/foo";
        let exp_path = format!("{}/{}", root_dir, DEFAULT_CONFIG_FILE_NAME);
        let write_file = |file_path: &str, contents: &str, make_executable: bool| {
            assert_eq!(&exp_path, file_path);
            assert_eq!(CONFIG_FILE_TEMPLATE, contents);
            assert!(!make_executable);
            Ok(())
        };
        let file_exists = |_path: &str| Ok(false);
        let result = create_default_config_file(write_file, file_exists, root_dir);
        assert_eq!(result, Ok(()));
    }
}

#[cfg(test)]
mod create_config_file_tests {
    use super::*;

    #[test]
    fn returns_empty_when_config_exists() {
        let write_file = |_file_path: &str, _contents: &str, _x: bool| {
            panic!("Should not get here");
        };
        let file_exists = |_path: &str| Ok(true);
        let result = create_config_file(write_file, file_exists, "", "");
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn returns_error_on_io_error() {
        let write_file = |_file_path: &str, _contents: &str, _x: bool| {
            panic!("Should not get here");
        };
        let file_exists = |_path: &str| Err(());
        let result = create_config_file(write_file, file_exists, "", "");
        assert_eq!(
            result,
            Err(String::from(
                "Fatal error encountered while looking for existing config"
            ))
        );
    }

    #[test]
    fn creates_default_when_specified_file_invalid() {
        let root_dir = "/usr/mine/bar";
        let exp_path = format!("{}/{}", root_dir, DEFAULT_CONFIG_FILE_NAME);
        let write_file = |file_path: &str, contents: &str, make_executable: bool| {
            assert_eq!(&exp_path, file_path);
            assert_eq!(CONFIG_FILE_TEMPLATE, contents);
            assert!(!make_executable);
            Ok(())
        };
        let file_exists = |_path: &str| Ok(false);
        let result = create_config_file(write_file, file_exists, root_dir, "not-valid");
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn creates_specified_config_when_valid() {
        let root_dir = "/usr/mine/bar";
        let desired_config = "rusty-hook.toml";
        let exp_path = format!("{}/{}", root_dir, desired_config);
        let write_file = |file_path: &str, contents: &str, make_executable: bool| {
            assert_eq!(&exp_path, file_path);
            assert_eq!(CONFIG_FILE_TEMPLATE, contents);
            assert!(!make_executable);
            Ok(())
        };
        let file_exists = |_path: &str| Ok(false);
        let result = create_config_file(write_file, file_exists, root_dir, desired_config);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn returns_error_when_write_fails() {
        let exp_err = "Failed to create config file";
        let write_file = |_file_path: &str, _contents: &str, _x: bool| Err(String::from(""));
        let file_exists = |_path: &str| Ok(false);
        let result = create_config_file(write_file, file_exists, "", "");
        assert_eq!(result, Err(String::from(exp_err)));
    }
}

#[cfg(test)]
mod get_config_file_contents_tests {
    use super::*;

    #[test]
    fn fails_on_config_file_search_error() {
        let file_exists = |_path: &str| Err(());
        let read_file = |_path: &str| panic!("Should not call here");
        let result = get_config_file_contents(read_file, file_exists, "");
        assert_eq!(result, Err(String::from(NO_CONFIG_FILE_FOUND)));
    }

    #[test]
    fn fails_on_config_file_not_found() {
        let file_exists = |_path: &str| Ok(false);
        let read_file = |_path: &str| panic!("Should not call here");
        let result = get_config_file_contents(read_file, file_exists, "");
        assert_eq!(result, Err(String::from(NO_CONFIG_FILE_FOUND)));
    }

    #[test]
    fn fails_on_config_file_read_error() {
        let exp_err = "Failure reading file";
        let file_exists = |_path: &str| Ok(true);
        let read_file = |_path: &str| Err(());
        let result = get_config_file_contents(read_file, file_exists, "/var/foo");
        assert_eq!(result, Err(String::from(exp_err)));
    }

    #[test]
    fn returns_contents_on_success() {
        let exp_contents = "[hooks]
            pre-commit = 'cargo test'
        ";
        let file_exists = |_path: &str| Ok(true);
        let read_file = |_path: &str| Ok(String::from(exp_contents));
        let result = get_config_file_contents(read_file, file_exists, "/var/foo");
        assert_eq!(result.unwrap(), String::from(exp_contents));
    }
}

#[cfg(test)]
mod get_table_key_value_from_config {
    use super::*;

    #[test]
    fn handles_invalid_toml() {
        let invalid_contents = "90827342089734";
        let result = get_table_key_value_from_config(invalid_contents, "", "");
        assert_eq!(result, Err(String::from("Error parsing config file")));
    }

    #[test]
    fn handles_missing_table() {
        let contents = "[hooks]";
        let result = get_table_key_value_from_config(contents, "foo", "");
        assert_eq!(result, Err(String::from("Missing config table")));
    }

    #[test]
    fn handles_missing_table_key() {
        let contents = r#"[hooks]
            pre-commit = "cargo test"
        "#;
        let result = get_table_key_value_from_config(contents, "hooks", "pre-push");
        assert_eq!(result, Err(String::from("Missing config key")));
    }

    #[test]
    fn parses_hook_value() {
        let contents = r#"[hooks]
            pre-commit = "cargo test"
        "#;
        let table = "hooks";
        let key = "pre-commit";
        let exp_value = "cargo test";
        let result = get_table_key_value_from_config(contents, table, key).unwrap();
        assert_eq!(
            String::from(result.as_str().unwrap()),
            String::from(exp_value)
        );
    }
}

#[cfg(test)]
mod get_log_setting_tests {
    use super::*;

    #[test]
    fn returns_true_when_content_not_found() {
        let invalid_contents = "90827342089734";
        let result = get_log_setting(invalid_contents);
        assert!(result);
    }

    #[test]
    fn returns_true_when_log_not_boolean() {
        let contents = r#"[logging]
            verbose = "cargo test"
        "#;
        let result = get_log_setting(contents);
        assert!(result);
    }

    #[test]
    fn returns_result_when_value_valid() {
        let contents = "[logging]
            verbose = false
        ";
        let result = get_log_setting(contents);
        assert!(!result);
    }
}

#[cfg(test)]
mod get_hook_script_tests {
    use super::*;

    #[test]
    fn returns_err_when_content_not_found() {
        let invalid_contents = "abc";
        let result = get_hook_script(invalid_contents, "");
        assert_eq!(result, Err(String::from("Error parsing config file")));
    }

    #[test]
    fn returns_err_when_hook_not_string() {
        let contents = "[hooks]
            pre-push = false
        ";
        let result = get_hook_script(contents, "pre-push");
        assert_eq!(result, Err(String::from("Invalid hook config")));
    }

    #[test]
    fn returns_result_when_value_valid() {
        let contents = r#"[hooks]
            pre-commit = "cargo test"
        "#;
        let result = get_hook_script(contents, "pre-commit");
        assert_eq!(result.unwrap(), "cargo test");
    }

    #[test]
    fn returns_result_when_value_array() {
        let contents = r#"[hooks]
            pre-commit = [
                "cargo test",
                "cargo fmt"
            ]
        "#;
        let result = get_hook_script(contents, "pre-commit");
        assert_eq!(result.unwrap(), "cargo test && cargo fmt");
    }

    #[test]
    fn returns_error_when_wrong_value_array() {
        let contents = r#"[hooks]
            pre-commit = [
                "cargo test",
                8
            ]
        "#;
        let result = get_hook_script(contents, "pre-commit");
        assert_eq!(
            result,
            Err(String::from(
                "Invalid hook config for pre-commit. An element in the array is not a string"
            ))
        );
    }
}
