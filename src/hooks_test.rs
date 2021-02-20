use super::*;

const EXP_NO_CONFIG_FILE_FOUND_ERROR_CODE: i32 = 3;

const EXP_VERSION: &str = env!("CARGO_PKG_VERSION");
const EXP_HOOK_FILE_TEMPLATE: &str = include_str!("hook_files/hook_script.sh");
const EXP_HOOK_CLI_SCRIPT_FILE_TEMPLATE: &str = include_str!("hook_files/cli.sh");
const EXP_HOOK_SEMVER_SCRIPT_FILE_TEMPLATE: &str = include_str!("hook_files/semver.sh");

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

const EXP_CLI_SCRIPT_NAME: &str = "cli.sh";
const EXP_SEMVER_SCRIPT_NAME: &str = "semver.sh";

const EXP_MINIMUM_CLI_MAJOR_VERSION: i32 = 0;
const EXP_MINIMUM_CLI_MINOR_VERSION: i32 = 12;
const EXP_MINIMUM_CLI_PATCH_VERSION: i32 = 0;
const EXP_MINIMUM_CLI_VERSION_ALLOW_PRERELEASE: bool = false;

const EXP_HOOK_CREATION_ERROR: &str =
    "Fatal error encountered while trying to create git hook files";

fn get_expected_hook_file_contents() -> String {
    String::from(EXP_HOOK_FILE_TEMPLATE).replace("{{VERSION}}", EXP_VERSION)
}

fn get_expected_cli_script_file_contents() -> String {
    let exit_code = &EXP_NO_CONFIG_FILE_FOUND_ERROR_CODE.to_string();
    let minimum_major = &EXP_MINIMUM_CLI_MAJOR_VERSION.to_string();
    let minimum_minor = &EXP_MINIMUM_CLI_MINOR_VERSION.to_string();
    let minimum_patch = &EXP_MINIMUM_CLI_PATCH_VERSION.to_string();
    let minimum_allow_pre = &EXP_MINIMUM_CLI_VERSION_ALLOW_PRERELEASE.to_string();
    String::from(EXP_HOOK_CLI_SCRIPT_FILE_TEMPLATE)
        .replace("{{VERSION}}", VERSION)
        .replace("\n# shellcheck disable=SC2170,SC1083", "")
        .replace("{{NO_CONFIG_FILE_EXIT_CODE}}", exit_code)
        .replace("{{MINIMUM_MAJOR}}", minimum_major)
        .replace("{{MINIMUM_MINOR}}", minimum_minor)
        .replace("{{MINIMUM_PATCH}}", minimum_patch)
        .replace("{{MINIMUM_ALLOW_PRE}}", minimum_allow_pre)
}

fn get_expected_semver_script_file_contents() -> String {
    String::from(EXP_HOOK_SEMVER_SCRIPT_FILE_TEMPLATE).replace("{{VERSION}}", VERSION)
}

#[cfg(test)]
mod constants_tests {
    use super::*;

    #[test]
    fn should_use_correct_hook_names() {
        for (&exp_hook, &act_hook) in EXP_HOOK_NAMES.iter().zip(HOOK_NAMES.iter()) {
            assert_eq!(exp_hook, act_hook);
        }
    }
}

#[cfg(test)]
mod get_hook_file_contents_tests {
    use super::*;

    #[test]
    fn provides_correct_hook_file_contents() {
        let exp = get_expected_hook_file_contents();
        assert_eq!(get_hook_file_contents(), exp);
    }
}

#[cfg(test)]
mod get_cli_script_file_contents_tests {
    use super::*;

    #[test]
    fn provides_correct_cli_script_file_contents() {
        let exp = get_expected_cli_script_file_contents();
        assert_eq!(get_cli_script_file_contents(), exp);
    }
}

#[cfg(test)]
mod get_semver_script_file_contents_tests {
    use super::*;

    #[test]
    fn provides_correct_semver_script_file_contents() {
        let exp = get_expected_semver_script_file_contents();
        assert_eq!(get_semver_script_file_contents(), exp);
    }
}

#[cfg(test)]
mod create_hook_files_tests {
    use super::*;

    #[test]
    fn errors_when_hook_write_fails() {
        let write_file = |path: &str, _contents: &str, _make_executable: bool| {
            let file_name = &&path[(path.rfind('/').unwrap() + 1)..];
            match *file_name {
                EXP_CLI_SCRIPT_NAME => Ok(()),
                EXP_SEMVER_SCRIPT_NAME => Ok(()),
                _ => Err(String::from("")),
            }
        };
        let result = create_hook_files(write_file, "", "", &vec![]);
        assert_eq!(result, Err(String::from(EXP_HOOK_CREATION_ERROR)));
    }

    #[test]
    fn errors_when_cli_script_write_fails() {
        let write_file = |path: &str, _contents: &str, _make_executable: bool| {
            let file_name = &&path[(path.rfind('/').unwrap() + 1)..];
            match *file_name {
                EXP_CLI_SCRIPT_NAME => Err(String::from("")),
                EXP_SEMVER_SCRIPT_NAME => Ok(()),
                _ => Ok(()),
            }
        };
        let result = create_hook_files(write_file, "", "", &vec![]);
        assert_eq!(result, Err(String::from(EXP_HOOK_CREATION_ERROR)));
    }

    #[test]
    fn errors_when_semver_script_write_fails() {
        let write_file = |path: &str, _contents: &str, _make_executable: bool| {
            let file_name = &&path[(path.rfind('/').unwrap() + 1)..];
            match *file_name {
                EXP_CLI_SCRIPT_NAME => Ok(()),
                EXP_SEMVER_SCRIPT_NAME => Err(String::from("")),
                _ => Ok(()),
            }
        };
        let result = create_hook_files(write_file, "", "", &vec![]);
        assert_eq!(result, Err(String::from(EXP_HOOK_CREATION_ERROR)));
    }

    #[test]
    fn creates_all_hooks() {
        let root_dir = "/usr/repos/foo";
        let git_hooks = ".git/hooks";
        let exp_contents = get_expected_hook_file_contents();
        let exp_cli_contents = get_expected_cli_script_file_contents();
        let exp_cli_path = &format!("{}/{}/{}", root_dir, git_hooks, EXP_CLI_SCRIPT_NAME);
        let exp_semver_path = &format!("{}/{}/{}", root_dir, git_hooks, EXP_SEMVER_SCRIPT_NAME);
        let exp_semver_contents = get_expected_semver_script_file_contents();
        let write_file = |path: &str, contents: &str, make_executable: bool| {
            let file_name = &&path[(path.rfind('/').unwrap() + 1)..];
            match *file_name {
                EXP_CLI_SCRIPT_NAME => {
                    assert_eq!(exp_cli_path, path);
                    assert_eq!(exp_cli_contents, contents);
                }
                EXP_SEMVER_SCRIPT_NAME => {
                    assert_eq!(exp_semver_path, path);
                    assert_eq!(exp_semver_contents, contents);
                }
                _ => {
                    let exp_hook = EXP_HOOK_NAMES.iter().find(|&n| n == file_name).unwrap();
                    let exp_path = &format!("{}/{}/{}", root_dir, git_hooks, exp_hook);
                    assert_eq!(exp_path, path);
                    assert_eq!(exp_contents, contents);
                }
            }
            assert_eq!(true, make_executable);
            Ok(())
        };
        let result = create_hook_files(write_file, root_dir, git_hooks, &vec![]);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn creates_skipped_hook() {
        let root_dir = "/usr/repos/foo";
        let git_hooks = ".git/hooks";
        let exp_contents = get_expected_hook_file_contents();
        let exp_cli_contents = get_expected_cli_script_file_contents();
        let exp_cli_path = &format!("{}/{}/{}", root_dir, git_hooks, EXP_CLI_SCRIPT_NAME);
        let exp_semver_path = &format!("{}/{}/{}", root_dir, git_hooks, EXP_SEMVER_SCRIPT_NAME);
        let exp_semver_contents = get_expected_semver_script_file_contents();
        let skipped_hook = "commit-msg";
        let write_file = |path: &str, contents: &str, make_executable: bool| {
            let file_name = &&path[(path.rfind('/').unwrap() + 1)..];
            match *file_name {
                EXP_CLI_SCRIPT_NAME => {
                    assert_eq!(exp_cli_path, path);
                    assert_eq!(exp_cli_contents, contents);
                }
                EXP_SEMVER_SCRIPT_NAME => {
                    assert_eq!(exp_semver_path, path);
                    assert_eq!(exp_semver_contents, contents);
                }
                _ => {
                    let exp_hook = EXP_HOOK_NAMES
                        .iter()
                        .find(|&n| n == file_name && n != &skipped_hook)
                        .unwrap();
                    let exp_path = &format!("{}/{}/{}", root_dir, git_hooks, exp_hook);
                    assert_eq!(exp_path, path);
                    assert_eq!(exp_contents, contents);
                }
            }
            assert_eq!(true, make_executable);
            Ok(())
        };
        let result = create_hook_files(write_file, root_dir, git_hooks, &vec![skipped_hook]);
        assert_eq!(result, Ok(()));
    }
}
