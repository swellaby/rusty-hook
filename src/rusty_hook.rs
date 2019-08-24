#[path = "config.rs"]
mod config;

#[path = "git.rs"]
mod git;

pub use config::NO_CONFIG_FILE_FOUND;
pub use git::NO_CONFIG_FILE_FOUND_ERROR_CODE;

pub fn init_directory<F, G, H>(
    run_command: F,
    write_file: G,
    file_exists: H,
    target_directory: Option<&str>,
) -> Result<(), String>
where
    F: Fn(&str, Option<&str>) -> Result<String, String>,
    G: Fn(&str, &str, bool) -> Result<(), String>,
    H: Fn(&str) -> Result<bool, ()>,
{
    let root_directory_path = match git::get_root_directory_path(&run_command, target_directory) {
        Ok(path) => path,
        Err(_) => return Err(String::from("Failure determining git repo root directory")),
    };

    if git::setup_hooks(&run_command, &write_file, &root_directory_path).is_err() {
        return Err(String::from("Unable to create git hooks"));
    };

    if config::create_default_config_file(&write_file, &file_exists, &root_directory_path).is_err()
    {
        return Err(String::from("Unable to create config file"));
    }

    Ok(())
}

pub fn init<F, G, H>(run_command: F, write_file: G, file_exists: H) -> Result<(), String>
where
    F: Fn(&str, Option<&str>) -> Result<String, String>,
    G: Fn(&str, &str, bool) -> Result<(), String>,
    H: Fn(&str) -> Result<bool, ()>,
{
    init_directory(&run_command, &write_file, &file_exists, None)
}

pub fn run<F, G, H, I>(
    run_command: F,
    file_exists: G,
    read_file: H,
    log: I,
    hook_name: &str,
) -> Result<(), String>
where
    F: Fn(&str, Option<&str>) -> Result<String, String>,
    G: Fn(&str) -> Result<bool, ()>,
    H: Fn(&str) -> Result<String, ()>,
    I: Fn(&str, bool),
{
    let root_directory_path = match git::get_root_directory_path(&run_command, None) {
        Ok(path) => path,
        Err(_) => return Err(String::from("Failure determining git repo root directory")),
    };

    let config_file_contents =
        match config::get_config_file_contents(read_file, file_exists, &root_directory_path) {
            Ok(contents) => contents,
            Err(err) => {
                if err == config::NO_CONFIG_FILE_FOUND {
                    return Err(String::from(config::NO_CONFIG_FILE_FOUND));
                } else {
                    return Err(String::from("Failed to parse config file"));
                }
            }
        };

    let log_details = config::get_log_setting(&config_file_contents);
    let script = match config::get_hook_script(&config_file_contents, &hook_name) {
        Ok(script) => script,
        Err(err) => {
            if err == config::MISSING_CONFIG_KEY {
                return Ok(());
            }
            return Err(String::from("Invalid rusty-hook config file"));
        }
    };

    let message = format!(
        "Found configured hook: {}\nRunning command: {}",
        hook_name, script
    );
    log(&message, log_details);

    match run_command(&script, Some(&root_directory_path)) {
        Ok(stdout) => {
            log(&stdout, log_details);
            Ok(())
        }
        Err(stderr) => Err(stderr),
    }
}

#[cfg(test)]
#[path = "rusty_hook_test.rs"]
mod rusty_hook_tests;
