use super::*;
use std::collections::HashMap;

#[cfg(test)]
pub(crate) mod utils {
    use std::collections::HashMap;

    pub(crate) const GIT_REV_PARSE_CMD: &str = "git rev-parse --show-toplevel";

    #[allow(clippy::type_complexity)]
    pub(crate) fn build_simple_command_runner(
        outcome: Result<Option<String>, Option<String>>,
    ) -> Box<
        dyn Fn(
            &str,
            Option<&str>,
            bool,
            Option<&HashMap<String, String>>,
        ) -> Result<Option<String>, Option<String>>,
    > {
        Box::new(
            move |_: &str, _: Option<&str>, _: bool, _: Option<&HashMap<String, String>>| {
                outcome.to_owned()
            },
        )
    }
}

#[cfg(test)]
mod init_directory_tests {
    use super::utils::build_simple_command_runner;
    use super::*;

    #[test]
    fn returns_error_when_root_directory_detect_fails() {
        let exp_err = "Failure determining git repo root directory";
        let run_command = build_simple_command_runner(Err(Some(String::from(exp_err))));
        let write_file = |_file_path: &str, _contents: &str, _x: bool| {
            panic!("Should not get here");
        };
        let file_exists = |_path: &str| panic!("Should not get here");
        let result = init(run_command, write_file, file_exists, vec![]);
        assert_eq!(result, Err(String::from(exp_err)));
    }

    #[test]
    fn should_return_error_when_hook_creation_fails() {
        let run_command = build_simple_command_runner(Ok(Some(String::from(""))));
        let write_file = |_file_path: &str, _contents: &str, _x: bool| Err(String::from(""));
        let file_exists = |_path: &str| panic!("Should not get here");
        let result = init(run_command, write_file, file_exists, vec![]);
        assert_eq!(result, Err(String::from("Unable to create git hooks")));
    }

    #[test]
    fn should_return_error_when_config_creation_fails() {
        let run_command = build_simple_command_runner(Ok(Some(String::from(""))));
        let write_file = |_file_path: &str, _contents: &str, _x: bool| Ok(());
        let file_exists = |_path: &str| Err(());
        let result = init(run_command, write_file, file_exists, vec![]);
        assert_eq!(result, Err(String::from("Unable to create config file")));
    }

    #[test]
    fn should_return_ok_on_success() {
        let run_command = build_simple_command_runner(Ok(Some(String::from(""))));
        let write_file = |_file_path: &str, _contents: &str, _x: bool| Ok(());
        let file_exists = |_path: &str| Ok(false);
        let result = init(run_command, write_file, file_exists, vec![]);
        assert_eq!(result, Ok(()));
    }
}

mod init_tests {
    use super::*;

    #[test]
    fn invokes_init_directory_with_cwd() {
        let run_command = |_cmd: &str,
                           dir: Option<&str>,
                           _stream_io: bool,
                           _env: Option<&HashMap<String, String>>| {
            if let Some(target_dir) = dir {
                if target_dir != "." {
                    return Err(None);
                }
                Ok(Some(String::from("")))
            } else {
                Ok(Some(String::from(".")))
            }
        };
        let write_file = |_file_path: &str, _contents: &str, _x: bool| Ok(());
        let file_exists = |_path: &str| Ok(false);
        let result = init(run_command, write_file, file_exists, vec![]);
        assert_eq!(result, Ok(()));
    }
}

#[cfg(test)]
mod run_tests {
    use super::utils::build_simple_command_runner;
    use super::*;

    #[test]
    fn returns_error_when_root_directory_detect_fails() {
        let exp_err = "Failure determining git repo root directory";
        let run_command = build_simple_command_runner(Err(Some(String::from(exp_err))));
        let read_file = |_file_path: &str| panic!("");
        let file_exists = |_path: &str| panic!("");
        let log = |_path: &str, _should_log: bool| panic!("");
        let result = run(run_command, file_exists, read_file, log, "", None);
        assert_eq!(result, Err(Some(String::from(exp_err))));
    }

    #[test]
    fn returns_error_when_config_file_missing() {
        let exp_err = config::NO_CONFIG_FILE_FOUND;
        let run_command = build_simple_command_runner(Ok(Some(String::from(""))));
        let read_file = |_file_path: &str| Err(());
        let file_exists = |_path: &str| Ok(false);
        let log = |_path: &str, _should_log: bool| panic!("");
        let result = run(run_command, file_exists, read_file, log, "", None);
        assert_eq!(result, Err(Some(String::from(exp_err))));
    }

    #[test]
    fn returns_error_when_config_contents_unloadable() {
        let exp_err = "Failed to parse config file";
        let run_command = build_simple_command_runner(Ok(Some(String::from(""))));
        let read_file = |_file_path: &str| Err(());
        let file_exists = |_path: &str| Ok(true);
        let log = |_path: &str, _should_log: bool| panic!("");
        let result = run(run_command, file_exists, read_file, log, "", None);
        assert_eq!(result, Err(Some(String::from(exp_err))));
    }

    #[test]
    fn returns_ok_when_hook_missing() {
        let contents = "[hooks]
            pre-commit = 'cargo test'
        ";
        let run_command = build_simple_command_runner(Ok(Some(String::from(""))));
        let read_file = |_file_path: &str| Ok(String::from(contents));
        let file_exists = |_path: &str| Ok(true);
        let log = |_path: &str, _should_log: bool| panic!("");
        let result = run(run_command, file_exists, read_file, log, "pre-push", None);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn returns_error_on_invalid_config() {
        let exp_err = "Invalid rusty-hook config file";
        let contents = "abc";
        let run_command = build_simple_command_runner(Ok(Some(String::from(""))));
        let read_file = |_file_path: &str| Ok(String::from(contents));
        let file_exists = |_path: &str| Ok(true);
        let log = |_path: &str, _should_log: bool| panic!("");
        let result = run(run_command, file_exists, read_file, log, "pre-push", None);
        assert_eq!(result, Err(Some(String::from(exp_err))));
    }

    #[test]
    fn does_not_log_details_when_disabled() {
        let contents = r#"[hooks]
            pre-commit = "cargo test"

            [logging]
            verbose = false
        "#;
        let run_command = |cmd: &str,
                           _dir: Option<&str>,
                           stream_io: bool,
                           _env: Option<&HashMap<String, String>>| {
            if cmd == "cargo test" && stream_io {
                panic!("")
            }
            Ok(Some(String::from("")))
        };
        let read_file = |_file_path: &str| Ok(String::from(contents));
        let file_exists = |_path: &str| Ok(true);
        let log = |_path: &str, should_log: bool| {
            if should_log {
                panic!("")
            }
        };
        let result = run(run_command, file_exists, read_file, log, "pre-commit", None);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn logs_details_when_enabled() {
        let contents = r#"[hooks]
            pre-commit = "cargo test"

            [logging]
            verbose = true
        "#;
        let run_command = |cmd: &str,
                           _dir: Option<&str>,
                           stream_io: bool,
                           _env: Option<&HashMap<String, String>>| {
            if cmd == "cargo test" && !stream_io {
                panic!("")
            }
            Ok(Some(String::from("")))
        };
        let read_file = |_file_path: &str| Ok(String::from(contents));
        let file_exists = |_path: &str| Ok(true);
        let log = |_path: &str, should_log: bool| {
            if !should_log {
                panic!("")
            }
        };
        let result = run(run_command, file_exists, read_file, log, "pre-commit", None);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn returns_ok_when_script_succeeds() {
        let contents = r#"[hooks]
            pre-commit = "cargo test"

            [logging]
            verbose = false
        "#;
        let run_command = build_simple_command_runner(Ok(Some(String::from(""))));
        let read_file = |_file_path: &str| Ok(String::from(contents));
        let file_exists = |_path: &str| Ok(true);
        let log = |_path: &str, _should_log: bool| ();
        let result = run(run_command, file_exists, read_file, log, "pre-commit", None);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn returns_err_when_script_fails() {
        let exp_err = "crashed";
        let contents = r#"[hooks]
            pre-commit = "cargo test"

            [logging]
            verbose = false
        "#;
        let run_command = |cmd: &str,
                           _dir: Option<&str>,
                           _stream_io: bool,
                           _env: Option<&HashMap<String, String>>| {
            if cmd == "cargo test" {
                return Err(Some(String::from(exp_err)));
            }
            Ok(Some(String::from("")))
        };
        let read_file = |_file_path: &str| Ok(String::from(contents));
        let file_exists = |_path: &str| Ok(true);
        let log = |_path: &str, _should_log: bool| ();
        let result = run(run_command, file_exists, read_file, log, "pre-commit", None);
        assert_eq!(result, Err(Some(String::from(exp_err))));
    }

    mod git_params {
        use super::super::utils::GIT_REV_PARSE_CMD;
        use super::*;

        #[test]
        fn handles_no_params_correctly() {
            let contents = r#"[hooks]
                pre-push = "echo %rh!"

                [logging]
                verbose = false
            "#;
            let run_command =
                |c: &str, _: Option<&str>, _: bool, env: Option<&HashMap<String, String>>| {
                    assert!(env.is_none());
                    if c != GIT_REV_PARSE_CMD {
                        assert_eq!(c, "echo %rh!");
                    }

                    Ok(Some(String::new()))
                };
            let read_file = |_file_path: &str| Ok(String::from(contents));
            let file_exists = |_path: &str| Ok(true);
            let log = |_path: &str, _should_log: bool| ();
            let result = run(run_command, file_exists, read_file, log, "pre-push", None);
            assert!(result.is_ok());
        }

        #[test]
        fn handles_params_correctly() {
            let params = ".git/COMMIT_EDITMSG";
            let hook_script = "echo %rh!";
            let contents = format!(
                r#"[hooks]
                commit-msg = "{}"

                [logging]
                verbose = false
            "#,
                &hook_script
            );
            let run_command =
                |c: &str, _: Option<&str>, _: bool, env: Option<&HashMap<String, String>>| {
                    if c != GIT_REV_PARSE_CMD {
                        assert!(env.is_some());
                        assert_eq!(c, &format!("echo {}", &params));
                        let env = env.unwrap();
                        assert_eq!(env.len(), 1);
                        assert_eq!(env.get("RUSTY_HOOK_GIT_PARAMS").unwrap(), &params);
                    }

                    Ok(Some(hook_script.to_owned()))
                };
            let read_file = |_file_path: &str| Ok(contents.to_owned());
            let file_exists = |_path: &str| Ok(true);
            let log = |_path: &str, _should_log: bool| ();
            let result = run(
                run_command,
                file_exists,
                read_file,
                log,
                "commit-msg",
                Some(params.to_owned()),
            );
            assert!(result.is_ok());
        }
    }
}
