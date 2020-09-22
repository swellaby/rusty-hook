use toml::Value;

const CONFIG_FILE_TEMPLATE: &str = "[hooks]
pre-commit = \"cargo test\"

[logging]
verbose = true
";

const DEFAULT_CONFIG_FILE_NAME: &str = ".rusty-hook.toml";
const CONFIG_FILE_NAMES: [&str; 2] = [DEFAULT_CONFIG_FILE_NAME, "rusty-hook.toml"];
pub const NO_CONFIG_FILE_FOUND: &str = "No config file found";
pub const MISSING_CONFIG_KEY: &str = "Missing config key";
pub const FATAL_ERROR_DURING_CONFIG_LOOKUP: &str =
    "Fatal error encountered while looking for existing config";

fn find_config_file<F>(root_directory_path: &str, file_exists: F) -> Result<String, String>
where
    F: Fn(&str) -> Result<bool, ()>,
{
    for &config_file_name in CONFIG_FILE_NAMES.iter() {
        let path = format!("{}/{}", root_directory_path, config_file_name);
        match file_exists(&path) {
            Err(_) => {
                return Err(String::from(FATAL_ERROR_DURING_CONFIG_LOOKUP));
            }
            Ok(found) => {
                if found {
                    return Ok(path);
                }
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
    F: Fn(&str, &str, bool) -> Result<(), String>,
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
    F: Fn(&str, &str, bool) -> Result<(), String>,
    G: Fn(&str) -> Result<bool, ()>,
{
    match find_config_file(root_directory_path, &file_exists) {
        Err(_) => {
            return Err(String::from(FATAL_ERROR_DURING_CONFIG_LOOKUP));
        }
        Ok(path) => {
            if path != NO_CONFIG_FILE_FOUND {
                return Ok(());
            }
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
        false,
    )
    .is_err()
    {
        return Err(String::from("Failed to create config file"));
    };
    Ok(())
}

pub fn get_config_file_contents<F, G>(
    read_file: F,
    file_exists: G,
    root_directory_path: &str,
) -> Result<String, String>
where
    F: Fn(&str) -> Result<String, ()>,
    G: Fn(&str) -> Result<bool, ()>,
{
    let path = match find_config_file(root_directory_path, &file_exists) {
        Ok(path) => {
            if path == NO_CONFIG_FILE_FOUND {
                return Err(String::from(NO_CONFIG_FILE_FOUND));
            } else {
                path
            }
        }
        Err(_) => return Err(String::from(NO_CONFIG_FILE_FOUND)),
    };

    match read_file(&path) {
        Ok(contents) => Ok(contents),
        Err(_) => Err(String::from("Failure reading file")),
    }
}

fn get_table_key_value_from_config(
    config_contents: &str,
    table: &str,
    key: &str,
) -> Result<Value, String> {
    let value = match config_contents.parse::<Value>() {
        Ok(val) => val,
        Err(_) => return Err(String::from("Error parsing config file")),
    };

    let config = value.as_table().unwrap();
    if !config.contains_key(table) {
        return Err(String::from("Missing config table"));
    };

    if !value[table].as_table().unwrap().contains_key(key) {
        return Err(String::from(MISSING_CONFIG_KEY));
    };

    Ok(value[table][key].clone())
}

pub fn get_log_setting(config_contents: &str) -> bool {
    match get_table_key_value_from_config(config_contents, "logging", "verbose") {
        Err(_) => true,
        Ok(value) => match value.as_bool() {
            Some(setting) => setting,
            None => true,
        },
    }
}

pub fn get_hook_script(config_contents: &str, hook_name: &str) -> Result<String, String> {
    match get_table_key_value_from_config(config_contents, "hooks", hook_name) {
        Err(err) => Err(err),
        Ok(value) => match value {
            Value::String(script) => Ok(script),
            Value::Array(script) => {
                let mut script = script.iter().map(|f| f.as_str());
                match script.any(|f| f.is_none()) {
                    true => Err(String::from("An element in the array is not a string")),
                    false => Ok(script
                        .map(|f| f.unwrap())
                        .collect::<Vec<&str>>()
                        .join(" && ")),
                }
            }
            _ => Err(String::from("Invalid hook config")),
        },
    }
}

#[cfg(test)]
#[path = "config_test.rs"]
mod config_tests;
