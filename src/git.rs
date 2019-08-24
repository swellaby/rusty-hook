mod hooks;

pub use hooks::NO_CONFIG_FILE_FOUND_ERROR_CODE;

pub fn get_root_directory_path<F>(run_command: F, target_directory: &str) -> Result<String, String>
where
    F: Fn(&str, Option<&str>) -> Result<String, String>,
{
    run_command("git rev-parse --show-toplevel", Some(&target_directory))
}

fn get_hooks_directory<F>(run_command: F, root_directory: &str) -> Result<String, String>
where
    F: Fn(&str, Option<&str>) -> Result<String, String>,
{
    run_command("git rev-parse --git-path hooks", Some(&root_directory))
}

pub fn setup_hooks<F, G>(
    run_command: F,
    write_file: G,
    root_directory_path: &str,
) -> Result<(), String>
where
    F: Fn(&str, Option<&str>) -> Result<String, String>,
    G: Fn(&str, &str, bool) -> Result<(), String>,
{
    let hooks_directory = match get_hooks_directory(&run_command, &root_directory_path) {
        Ok(path) => path,
        Err(_) => return Err(String::from("Failure determining git hooks directory")),
    };
    hooks::create_hook_files(write_file, &root_directory_path, &hooks_directory)
}

#[cfg(test)]
#[path = "git_test.rs"]
mod git_tests;
