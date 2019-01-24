#[path = "config.rs"]
mod config;

#[path = "git.rs"]
mod git;

pub fn init<F, G, H>(run_command: F, write_file: G, file_exists: H) -> Result<(), String>
where
    F: Fn(&str) -> Result<String, String>,
    G: Fn(&str, &str) -> Result<(), String>,
    H: Fn(&str) -> Result<bool, ()>,
{
    let root_directory_path = match git::get_root_directory_path(&run_command) {
        Ok(path) => path,
        Err(_) => return Err(String::from("Failure determining git repo root directory")),
    };

    if git::create_hook_files(&run_command, &write_file, &root_directory_path).is_err() {
        return Err(String::from("Unable to create git hooks"));
    };

    if config::create_default_config_file(&write_file, &file_exists, &root_directory_path).is_err()
    {
        return Err(String::from(""));
    }

    Ok(())
}
