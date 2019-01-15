#[path = "git.rs"]
mod git;

pub fn init<F, G>(run_command: F, write_file: G) -> Result<(), String>
where
    F: Fn(&str) -> Result<String, String>,
    G: Fn(&str, &str),
{
    if git::create_hook_files(&run_command, &write_file).is_err() {
        return Err(String::from("Unable to create git hooks"));
    };

    // create config file
    Ok(())
}
