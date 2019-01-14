use super::*;

#[test]
fn get_root_directory_path_works() {
    let exp = "/usr/me/foo";
    let run_command = |cmd: &str| {
        if cmd == "git rev-parse --show-toplevel" {
            String::from(exp)
        } else {
            String::from("")
        }
    };
    let act = get_root_directory_path(run_command);
    assert_eq!(act, exp);
}

#[test]
fn get_hooks_directory_works() {
    let exp = "/.git/hooks";
    let run_command = |cmd: &str| {
        if cmd == "git rev-parse --git-path hooks" {
            String::from(exp)
        } else {
            String::from("")
        }
    };
    let act = get_hooks_directory(run_command);
    assert_eq!(act, exp);
}
