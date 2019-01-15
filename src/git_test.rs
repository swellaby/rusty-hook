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
