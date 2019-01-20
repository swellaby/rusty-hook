use super::*;

#[test]
fn get_root_directory_path_returns_path() {
    let exp = "/usr/me/foo";
    let run_command = |cmd: &str| {
        if cmd == "git rev-parse --show-toplevel" {
            Ok(String::from(exp))
        } else {
            Ok(String::from(""))
        }
    };
    let act = get_root_directory_path(run_command);
    assert_eq!(act.unwrap(), exp);
}

#[test]
fn get_root_directory_path_returns_err() {
    let exp_err = "Ah!";
    let run_command = |_cmd: &str| Err(String::from(exp_err));
    let act = get_root_directory_path(run_command);
    assert_eq!(act, Err(String::from(exp_err)));
}

#[test]
fn get_hooks_directory_returns_path() {
    let exp = "/.git/hooks";
    let run_command = |cmd: &str| {
        if cmd == "git rev-parse --git-path hooks" {
            Ok(String::from(exp))
        } else {
            Ok(String::from(""))
        }
    };
    let act = get_hooks_directory(run_command);
    assert_eq!(act.unwrap(), exp);
}

#[test]
fn get_hooks_directory_returns_err() {
    let exp_err = "failed";
    let run_command = |_cmd: &str| Err(String::from(exp_err));
    let act = get_hooks_directory(run_command);
    assert_eq!(act, Err(String::from(exp_err)));
}

#[test]
fn create_hook_files_fails_when_hooks_directory_unknown() {
    let exp_err = "Failure determining git hooks directory";
    let run_command = |_cmd: &str| Err(String::from(""));
    let write_file = |_path: &str, _contents: &str| Ok(());
    let result = create_hook_files(run_command, write_file, "");
    assert_eq!(result, Err(String::from(exp_err)));
}

#[test]
fn create_hook_files_fails_when_hook_write_fails() {
    let exp_err = "Fatal error encountered while trying to create git hook files";
    let run_command = |_cmd: &str| Ok(String::from("/usr/repos/foo/.git/hooks"));
    let write_file = |_path: &str, _contents: &str| Err(String::from(""));
    let result = create_hook_files(run_command, write_file, "");
    assert_eq!(result, Err(String::from(exp_err)));
}
