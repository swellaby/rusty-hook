pub use hooks::NO_CONFIG_FILE_FOUND_ERROR_CODE;

mod hooks;

pub fn get_root_directory_path<F>(
    run_command: F,
    target_directory: Option<&str>,
) -> Result<Option<String>, Option<String>>
where
    F: Fn(&str, Option<&str>, bool) -> Result<Option<String>, Option<String>>,
{
    run_command("git rev-parse --show-toplevel", target_directory, false)
}

fn get_hooks_directory<F>(
    run_command: F,
    root_directory: &str,
) -> Result<Option<String>, Option<String>>
where
    F: Fn(&str, Option<&str>, bool) -> Result<Option<String>, Option<String>>,
{
    run_command(
        "git rev-parse --git-path hooks",
        Some(&root_directory),
        false,
    )
}

pub fn setup_hooks<F, G>(
    run_command: F,
    write_file: G,
    root_directory_path: &str,
) -> Result<(), String>
where
    F: Fn(&str, Option<&str>, bool) -> Result<Option<String>, Option<String>>,
    G: Fn(&str, &str, bool) -> Result<(), String>,
{
    let hooks_directory = match get_hooks_directory(&run_command, &root_directory_path) {
        Ok(Some(path)) => path,
        _ => return Err(String::from("Failure determining git hooks directory")),
    };
    hooks::create_hook_files(write_file, &root_directory_path, &hooks_directory)
}

#[cfg(test)]
#[path = "git_test.rs"]
mod git_tests;
