const CONFIG_FILE_TEMPLATE: &str = "[hooks]
pre-commit = \"cargo test\"";

const DEFAULT_CONFIG_FILE_NAME: &str = ".rusty-hook.toml";

const CONFIG_FILE_NAMES: [&str; 2] = [DEFAULT_CONFIG_FILE_NAME, "rusty-hook.toml"];

pub fn create_default_config_file<F, G>(
    write_file: F,
    file_exists: G,
    root_directory_path: &str,
) -> Result<(), String>
where
    F: Fn(&str, &str) -> Result<(), String>,
    G: Fn(&str) -> Result<bool, ()>,
{
    create_config_file(
        &write_file,
        &file_exists,
        root_directory_path,
        DEFAULT_CONFIG_FILE_NAME,
    )
}

pub fn create_config_file<F, G>(
    write_file: F,
    file_exists: G,
    root_directory_path: &str,
    desired_config_file_name: &str,
) -> Result<(), String>
where
    F: Fn(&str, &str) -> Result<(), String>,
    G: Fn(&str) -> Result<bool, ()>,
{
    let mut config_file = DEFAULT_CONFIG_FILE_NAME;
    for &config_file_name in CONFIG_FILE_NAMES.iter() {
        if desired_config_file_name == config_file_name {
            config_file = desired_config_file_name;
        };
        match file_exists(&format!("{}/{}", root_directory_path, config_file_name)) {
            Ok(found) => {
                if found {
                    return Ok(());
                }
            }
            Err(_) => {
                return Err(String::from(
                    "Fatal error encountered while looking for existing config",
                ))
            }
        };
    }

    if let Err(_) = write_file(
        &format!("{}/{}", root_directory_path, config_file),
        CONFIG_FILE_TEMPLATE,
    ) {
        return Err(String::from(""));
    };
    Ok(())
}
