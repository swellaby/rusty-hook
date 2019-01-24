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
            if path == &exp_path {
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
            assert_eq!(false, make_executable);
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
    fn returns_empty_result_when_config_already_exists() {
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
    fn creates_default_config_file_when_specified_file_invalid() {
        let root_dir = "/usr/mine/bar";
        let exp_path = format!("{}/{}", root_dir, DEFAULT_CONFIG_FILE_NAME);
        let write_file = |file_path: &str, contents: &str, make_executable: bool| {
            assert_eq!(&exp_path, file_path);
            assert_eq!(CONFIG_FILE_TEMPLATE, contents);
            assert_eq!(false, make_executable);
            Ok(())
        };
        let file_exists = |_path: &str| Ok(false);
        let result = create_config_file(write_file, file_exists, root_dir, "not-valid");
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn creates_specified_config_file_when_specified_file_valid() {
        let root_dir = "/usr/mine/bar";
        let desired_config = "rusty-hook.toml";
        let exp_path = format!("{}/{}", root_dir, desired_config);
        let write_file = |file_path: &str, contents: &str, make_executable: bool| {
            assert_eq!(&exp_path, file_path);
            assert_eq!(CONFIG_FILE_TEMPLATE, contents);
            assert_eq!(false, make_executable);
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
