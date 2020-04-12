use super::*;

#[cfg(test)]
mod get_root_directory_path_tests {
    use super::*;

    #[test]
    fn uses_git_rev_parse_top_level_command() {
        let exp = "/usr/me/foo";
        let target_dir = "";
        let run_command = |cmd: &str, dir: Option<&str>, stream_io: bool| {
            if cmd == "git rev-parse --show-toplevel" && dir == Some(target_dir) && !stream_io {
                Ok(Some(String::from(exp)))
            } else {
                Ok(None)
            }
        };
        let act = get_root_directory_path(run_command, Some(&target_dir));
        assert_eq!(act.unwrap(), Some(String::from(exp)));
    }

    #[test]
    fn returns_error_on_command_error() {
        let exp_err = "Ah!";
        let run_command =
            |_cmd: &str, _dir: Option<&str>, _stream_io: bool| Err(Some(String::from(exp_err)));
        let act = get_root_directory_path(run_command, None);
        assert_eq!(act, Err(Some(String::from(exp_err))));
    }
}

#[cfg(test)]
mod get_hooks_directory_tests {
    use super::*;

    #[test]
    fn uses_git_hooks_path_command() {
        let exp = ".git/hooks";
        let target_dir = "";
        let run_command = |cmd: &str, dir: Option<&str>, stream_io: bool| {
            if cmd == "git rev-parse --git-path hooks" && dir == Some(target_dir) && !stream_io {
                Ok(Some(String::from(exp)))
            } else {
                Ok(None)
            }
        };
        let act = get_hooks_directory(run_command, &target_dir);
        assert_eq!(act.unwrap(), Some(String::from(exp)));
    }

    #[test]
    fn returns_error_on_command_error() {
        let exp_err = "failed";
        let run_command =
            |_cmd: &str, _dir: Option<&str>, _stream_io: bool| Err(Some(String::from(exp_err)));
        let act = get_hooks_directory(run_command, "");
        assert_eq!(act, Err(Some(String::from(exp_err))));
    }
}

#[cfg(test)]
mod setup_hooks_tests {
    use super::*;

    #[test]
    fn errors_when_hooks_directory_unknown() {
        let exp_err = "Failure determining git hooks directory";
        let run_command = |_cmd: &str, _dir: Option<&str>, _stream_io: bool| Err(None);
        let write_file = |_path: &str, _contents: &str, _x: bool| Ok(());
        let result = setup_hooks(run_command, write_file, "");
        assert_eq!(result, Err(String::from(exp_err)));
    }

    #[test]
    fn errors_when_hook_write_fails() {
        let run_command = |_cmd: &str, _dir: Option<&str>, _stream_io: bool| {
            Ok(Some(String::from("/usr/repos/foo/.git/hooks")))
        };
        let write_file = |_path: &str, _contents: &str, _x: bool| Err(String::from(""));
        let result = setup_hooks(run_command, write_file, "");
        assert_eq!(result, Err(String::from(hooks::HOOK_CREATION_ERROR)));
    }

    #[test]
    fn creates_all_hooks() {
        let root_dir = "/usr/repos/foo";
        let git_hooks = ".git/hooks";
        let run_command = |_c: &str, _d: Option<&str>, _s: bool| Ok(Some(String::from(git_hooks)));
        let write_file = |_p: &str, _c: &str, _x: bool| Ok(());
        let result = setup_hooks(run_command, write_file, root_dir);
        assert_eq!(result, Ok(()));
    }
}
