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
mod create_config_file_tests {
    // use super::*;

}

#[cfg(test)]
mod create_default_config_file_tests {
    // use super::*;

}
