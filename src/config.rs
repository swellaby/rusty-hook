const CONFIG_FILE_TEMPLATE: &str = "[hooks]
pre-commit = \"cargo test\"";

const DEFAULT_CONFIG_FILE_NAME: &str = ".rusty-hook.toml";
const CONFIG_FILE_NAMES: [&str; 2] = [DEFAULT_CONFIG_FILE_NAME, "rusty-hook.toml"];
const NO_CONFIG_FILE_FOUND: &str = "No config file found";

fn find_config_file<F>(root_directory_path: &str, file_exists: F) -> Result<String, String>
where
    F: Fn(&str) -> Result<bool, ()>,
{
    for &config_file_name in CONFIG_FILE_NAMES.iter() {
        let path = format!("{}/{}", root_directory_path, config_file_name);
        match file_exists(&path) {
            Ok(found) => {
                if found {
                    return Ok(path);
                }
            }
            Err(_) => {
                return Err(String::from(
                    "Fatal error encountered while looking for existing config",
                ));
            }
        };
    }

    Ok(String::from(NO_CONFIG_FILE_FOUND))
}

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
    match find_config_file(root_directory_path, &file_exists) {
        Ok(path) => {
            if path != NO_CONFIG_FILE_FOUND {
                return Ok(());
            }
        }
        Err(_) => {
            return Err(String::from(
                "Fatal error encountered while looking for existing config",
            ));
        }
    };

    let config_file = if CONFIG_FILE_NAMES
        .iter()
        .any(|n| n == &desired_config_file_name)
    {
        desired_config_file_name
    } else {
        DEFAULT_CONFIG_FILE_NAME
    };

    if write_file(
        &format!("{}/{}", root_directory_path, config_file),
        CONFIG_FILE_TEMPLATE,
    )
    .is_err()
    {
        return Err(String::from(""));
    };
    Ok(())
}

#[cfg(test)]
#[path = "config_test.rs"]
mod config_tests;
