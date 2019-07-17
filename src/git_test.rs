use super::*;

fn get_expected_hook_file_contents() -> String {
    let version = env!("CARGO_PKG_VERSION");
    let exit_code = &NO_CONFIG_FILE_FOUND_ERROR_CODE.to_string();
    let minimum_major = &MINIMUM_CLI_MAJOR_VERSION.to_string();
    let minimum_minor = &MINIMUM_CLI_MINOR_VERSION.to_string();
    let minimum_patch = &MINIMUM_CLI_PATCH_VERSION.to_string();
    let minimum_allow_pre = &MINIMUM_CLI_VERSION_ALLOW_PRERELEASE.to_string();
    String::from(HOOK_FILE_TEMPLATE)
        .replace("{{VERSION}}", version)
        .replace("\n# shellcheck disable=SC2170,SC1083", "")
        .replace("{{NO_CONFIG_FILE_EXIT_CODE}}", exit_code)
        .replace("{{MINIMUM_MAJOR}}", minimum_major)
        .replace("{{MINIMUM_MINOR}}", minimum_minor)
        .replace("{{MINIMUM_PATCH}}", minimum_patch)
        .replace("{{MINIMUM_ALLOW_PRE}}", minimum_allow_pre)
}

#[cfg(test)]
mod get_root_directory_path_tests {
    use super::*;

    #[test]
    fn uses_git_rev_parse_top_level_command() {
        let exp = "/usr/me/foo";
        let target_dir = "";
        let run_command = |cmd: &str, dir: &str| {
            if cmd == "git rev-parse --show-toplevel" && dir == target_dir {
                Ok(String::from(exp))
            } else {
                Ok(String::from(""))
            }
        };
        let act = get_root_directory_path(run_command, &target_dir);
        assert_eq!(act.unwrap(), exp);
    }

    #[test]
    fn returns_error_on_command_error() {
        let exp_err = "Ah!";
        let run_command = |_cmd: &str, _dir: &str| Err(String::from(exp_err));
        let act = get_root_directory_path(run_command, "");
        assert_eq!(act, Err(String::from(exp_err)));
    }
}

#[cfg(test)]
mod get_hooks_directory_tests {
    use super::*;

    #[test]
    fn uses_git_hooks_path_command() {
        let exp = ".git/hooks";
        let target_dir = "";
        let run_command = |cmd: &str, dir: &str| {
            if cmd == "git rev-parse --git-path hooks" && dir == target_dir {
                Ok(String::from(exp))
            } else {
                Ok(String::from(""))
            }
        };
        let act = get_hooks_directory(run_command, &target_dir);
        assert_eq!(act.unwrap(), exp);
    }

    #[test]
    fn returns_error_on_command_error() {
        let exp_err = "failed";
        let run_command = |_cmd: &str, _dir: &str| Err(String::from(exp_err));
        let act = get_hooks_directory(run_command, "");
        assert_eq!(act, Err(String::from(exp_err)));
    }
}

#[cfg(test)]
mod get_hook_contents_tests {
    use super::*;

    #[test]
    fn provides_correct_contents() {
        let exp = get_expected_hook_file_contents();
        assert_eq!(get_hook_file_contents(), exp);
    }
}

#[cfg(test)]
mod create_hook_files_tests {
    use super::*;

    #[test]
    fn errors_when_hooks_directory_unknown() {
        let exp_err = "Failure determining git hooks directory";
        let run_command = |_cmd: &str, _dir: &str| Err(String::from(""));
        let write_file = |_path: &str, _contents: &str, _x: bool| Ok(());
        let result = create_hook_files(run_command, write_file, "");
        assert_eq!(result, Err(String::from(exp_err)));
    }

    #[test]
    fn errors_when_hook_write_fails() {
        let exp_err = "Fatal error encountered while trying to create git hook files";
        let run_command = |_cmd: &str, _dir: &str| Ok(String::from("/usr/repos/foo/.git/hooks"));
        let write_file = |_path: &str, _contents: &str, _x: bool| Err(String::from(""));
        let result = create_hook_files(run_command, write_file, "");
        assert_eq!(result, Err(String::from(exp_err)));
    }

    const EXP_HOOK_NAMES: [&str; 19] = [
        "applypatch-msg",
        "pre-applypatch",
        "post-applypatch",
        "pre-commit",
        "prepare-commit-msg",
        "commit-msg",
        "post-commit",
        "pre-rebase",
        "post-checkout",
        "post-merge",
        "pre-push",
        "pre-receive",
        "update",
        "post-receive",
        "post-update",
        "push-to-checkout",
        "pre-auto-gc",
        "post-rewrite",
        "sendemail-validate",
    ];

    #[test]
    fn should_use_correct_hook_names() {
        for (&exp_hook, &act_hook) in EXP_HOOK_NAMES.iter().zip(HOOK_NAMES.iter()) {
            assert_eq!(exp_hook, act_hook);
        }
    }

    #[test]
    fn creates_all_hooks() {
        let root_dir = "/usr/repos/foo";
        let git_hooks = ".git/hooks";
        let exp_contents = get_expected_hook_file_contents();
        let run_command = |_cmd: &str, _dir: &str| Ok(String::from(git_hooks));
        let write_file = |path: &str, contents: &str, make_executable: bool| {
            let act_hook = &&path[(path.rfind('/').unwrap() + 1)..];
            let exp_hook = EXP_HOOK_NAMES.iter().find(|&n| n == act_hook).unwrap();
            let exp_path = &format!("{}/{}/{}", root_dir, git_hooks, exp_hook);
            assert_eq!(exp_path, path);
            assert_eq!(exp_contents, contents);
            assert_eq!(true, make_executable);
            Ok(())
        };
        let result = create_hook_files(run_command, write_file, root_dir);
        assert_eq!(result, Ok(()));
    }
}
