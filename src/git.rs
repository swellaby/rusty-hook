pub const NO_CONFIG_FILE_FOUND_ERROR_CODE: i32 = 3;

pub fn get_root_directory_path<F>(run_command: F, target_directory: &str) -> Result<String, String>
where
    F: Fn(&str, &str) -> Result<String, String>,
{
    run_command("git rev-parse --show-toplevel", &target_directory)
}

fn get_hooks_directory<F>(run_command: F, root_directory: &str) -> Result<String, String>
where
    F: Fn(&str, &str) -> Result<String, String>,
{
    run_command("git rev-parse --git-path hooks", &root_directory)
}

const HOOK_FILE_TEMPLATE: &str = include_str!("hook_script.sh");
const HOOK_NAMES: [&str; 19] = [
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
const MINIMUM_CLI_MAJOR_VERSION: i32 = 0;
const MINIMUM_CLI_MINOR_VERSION: i32 = 9;
const MINIMUM_CLI_PATCH_VERSION: i32 = 0;
const MINIMUM_CLI_VERSION_ALLOW_PRERELEASE: bool = false;

fn get_hook_file_contents() -> String {
    let version = env!("CARGO_PKG_VERSION");
    let exit_code = &NO_CONFIG_FILE_FOUND_ERROR_CODE.to_string();
    let minimum_major = &MINIMUM_CLI_MAJOR_VERSION.to_string();
    let minimum_minor = &MINIMUM_CLI_MINOR_VERSION.to_string();
    let minimum_patch = &MINIMUM_CLI_PATCH_VERSION.to_string();
    let minimum_allow_pre = &MINIMUM_CLI_VERSION_ALLOW_PRERELEASE.to_string();
    let hook_file_contents = String::from(HOOK_FILE_TEMPLATE)
        .replace("{{VERSION}}", version)
        .replace("\n# shellcheck disable=SC2170,SC1083", "")
        .replace("{{NO_CONFIG_FILE_EXIT_CODE}}", exit_code)
        .replace("{{MINIMUM_MAJOR}}", minimum_major)
        .replace("{{MINIMUM_MINOR}}", minimum_minor)
        .replace("{{MINIMUM_PATCH}}", minimum_patch)
        .replace("{{MINIMUM_ALLOW_PRE}}", minimum_allow_pre);

    hook_file_contents
}

pub fn create_hook_files<F, G>(
    run_command: F,
    write_file: G,
    root_directory_path: &str,
) -> Result<(), String>
where
    F: Fn(&str, &str) -> Result<String, String>,
    G: Fn(&str, &str, bool) -> Result<(), String>,
{
    let hooks_directory = match get_hooks_directory(&run_command, &root_directory_path) {
        Ok(path) => path,
        Err(_) => return Err(String::from("Failure determining git hooks directory")),
    };
    let hook_file_contents = get_hook_file_contents();

    for hook in HOOK_NAMES.iter() {
        if write_file(
            &format!("{}/{}/{}", root_directory_path, hooks_directory, hook),
            &hook_file_contents,
            true,
        )
        .is_err()
        {
            return Err(String::from(
                "Fatal error encountered while trying to create git hook files",
            ));
        };
    }
    Ok(())
}

#[cfg(test)]
#[path = "git_test.rs"]
mod git_tests;
