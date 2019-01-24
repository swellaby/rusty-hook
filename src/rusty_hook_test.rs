use super::*;

#[cfg(test)]
mod init_tests {
    use super::*;

    #[test]
    fn returns_error_when_root_directory_detect_fails() {
        let exp_err = "Failure determining git repo root directory";
        let run_command = |_cmd: &str| Err(String::from(exp_err));
        let write_file = |_file_path: &str, _contents: &str, _x: bool| {
            panic!("Should not get here");
        };
        let file_exists = |_path: &str| panic!("Should not get here");
        let result = init(run_command, write_file, file_exists);
        assert_eq!(result, Err(String::from(exp_err)));
    }

    #[test]
    fn should_return_error_when_hook_creation_fails() {
        let run_command = |_cmd: &str| Ok(String::from(""));
        let write_file = |_file_path: &str, _contents: &str, _x: bool| Err(String::from(""));
        let file_exists = |_path: &str| panic!("Should not get here");
        let result = init(run_command, write_file, file_exists);
        assert_eq!(result, Err(String::from("Unable to create git hooks")));
    }

    #[test]
    fn should_return_error_when_config_creation_fails() {
        let run_command = |_cmd: &str| Ok(String::from(""));
        let write_file = |_file_path: &str, _contents: &str, _x: bool| Ok(());
        let file_exists = |_path: &str| Err(());
        let result = init(run_command, write_file, file_exists);
        assert_eq!(result, Err(String::from("Unable to create config file")));
    }

    #[test]
    fn should_return_ok_on_success() {
        let run_command = |_cmd: &str| Ok(String::from(""));
        let write_file = |_file_path: &str, _contents: &str, _x: bool| Ok(());
        let file_exists = |_path: &str| Ok(false);
        let result = init(run_command, write_file, file_exists);
        assert_eq!(result, Ok(()));
    }
}
