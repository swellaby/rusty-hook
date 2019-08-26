pub const HOOK_CREATION_ERROR: &str =
    "Fatal error encountered while trying to create git hook files";
pub const NO_CONFIG_FILE_FOUND_ERROR_CODE: i32 = 3;
const MINIMUM_CLI_MAJOR_VERSION: i32 = 0;
const MINIMUM_CLI_MINOR_VERSION: i32 = 10;
const MINIMUM_CLI_PATCH_VERSION: i32 = 0;
const MINIMUM_CLI_VERSION_ALLOW_PRERELEASE: bool = false;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const HOOK_FILE_TEMPLATE: &str = include_str!("hook_files/hook_script.sh");
const HOOK_CLI_SCRIPT_FILE_TEMPLATE: &str = include_str!("hook_files/cli.sh");
const HOOK_SEMVER_SCRIPT_FILE_TEMPLATE: &str = include_str!("hook_files/semver.sh");

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
const CLI_SCRIPT_NAME: &str = "cli.sh";
const SEMVER_SCRIPT_NAME: &str = "semver.sh";

// For unknown reasons, kcov is reporting an uncovered line for the closing `}`
// when using the idiomatic expression.
#[allow(clippy::needless_return)]
fn get_hook_file_contents() -> String {
    return String::from(HOOK_FILE_TEMPLATE).replace("{{VERSION}}", VERSION);
}

fn get_cli_script_file_contents() -> String {
    let exit_code = &NO_CONFIG_FILE_FOUND_ERROR_CODE.to_string();
    let minimum_major = &MINIMUM_CLI_MAJOR_VERSION.to_string();
    let minimum_minor = &MINIMUM_CLI_MINOR_VERSION.to_string();
    let minimum_patch = &MINIMUM_CLI_PATCH_VERSION.to_string();
    let minimum_allow_pre = &MINIMUM_CLI_VERSION_ALLOW_PRERELEASE.to_string();
    String::from(HOOK_CLI_SCRIPT_FILE_TEMPLATE)
        .replace("{{VERSION}}", VERSION)
        .replace("\n# shellcheck disable=SC2170,SC1083", "")
        .replace("{{NO_CONFIG_FILE_EXIT_CODE}}", exit_code)
        .replace("{{MINIMUM_MAJOR}}", minimum_major)
        .replace("{{MINIMUM_MINOR}}", minimum_minor)
        .replace("{{MINIMUM_PATCH}}", minimum_patch)
        .replace("{{MINIMUM_ALLOW_PRE}}", minimum_allow_pre)
}

// For unknown reasons, kcov is reporting an uncovered line for the closing `}`
// when using the idiomatic expression.
#[allow(clippy::needless_return)]
fn get_semver_script_file_contents() -> String {
    return String::from(HOOK_SEMVER_SCRIPT_FILE_TEMPLATE).replace("{{VERSION}}", VERSION);
}

fn get_file_path(root_directory_path: &str, hooks_directory: &str, file: &str) -> String {
    format!("{}/{}/{}", root_directory_path, hooks_directory, file)
}

pub fn create_hook_files<F>(
    write_file: F,
    root_directory_path: &str,
    hooks_directory: &str,
) -> Result<(), String>
where
    F: Fn(&str, &str, bool) -> Result<(), String>,
{
    let hook_file_contents = get_hook_file_contents();
    for hook in HOOK_NAMES.iter() {
        let path = get_file_path(root_directory_path, hooks_directory, hook);
        if write_file(&path, &hook_file_contents, true).is_err() {
            return Err(String::from(HOOK_CREATION_ERROR));
        };
    }

    let cli_file_contents = get_cli_script_file_contents();
    let cli_file_path = get_file_path(root_directory_path, hooks_directory, CLI_SCRIPT_NAME);
    if write_file(&cli_file_path, &cli_file_contents, true).is_err() {
        return Err(String::from(HOOK_CREATION_ERROR));
    };

    let semver_file_contents = get_semver_script_file_contents();
    let semver_file_path = get_file_path(root_directory_path, hooks_directory, SEMVER_SCRIPT_NAME);
    if write_file(&semver_file_path, &semver_file_contents, true).is_err() {
        return Err(String::from(HOOK_CREATION_ERROR));
    };

    Ok(())
}

#[cfg(test)]
#[path = "hooks_test.rs"]
mod hooks_tests;
