use std::fs::File;
use std::io::prelude::*;
#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn get_command_runner() -> fn(cmd: &str, dir: &str) -> Result<String, String> {
    |cmd: &str, dir: &str| {
        let target_dir = if dir.is_empty() { "." } else { dir };
        let mut program = "sh";
        let mut switch = "-c";
        if cfg!(target_os = "windows") {
            program = "cmd";
            switch = "/C";
        };
        match Command::new(&program)
            .current_dir(target_dir)
            .args(&[&switch, cmd])
            .output()
        {
            Err(details) => panic!(
                "Command runner crashed in unrecoverable manner. Details: {}",
                details
            ),
            Ok(output) => {
                if output.status.success() {
                    Ok(String::from_utf8(output.stdout)
                        .unwrap()
                        .trim_end_matches("\n")
                        .to_string())
                } else {
                    Err(format!(
                        "{}\n{}",
                        String::from_utf8(output.stderr).unwrap(),
                        String::from_utf8(output.stdout).unwrap(),
                    ))
                }
            }
        }
    }
}

#[cfg(target_family = "unix")]
fn create_file(path: PathBuf, make_executable: bool) -> Result<File, ()> {
    let file = match File::create(&path) {
        Ok(file) => file,
        Err(_) => return Err(()),
    };

    if make_executable {
        let metadata = match file.metadata() {
            Ok(metadata) => metadata,
            Err(_) => return Err(()),
        };

        let mut permissions = metadata.permissions();
        permissions.set_mode(0o755);
    };

    Ok(file)
}

#[cfg(target_family = "windows")]
fn create_file(path: PathBuf, _make_executable: bool) -> Result<File, ()> {
    match File::create(&path) {
        Err(_) => Err(()),
        Ok(file) => Ok(file),
    }
}

pub fn get_file_writer(
) -> fn(file_path: &str, contents: &str, make_executable: bool) -> Result<(), String> {
    |file_path: &str, contents: &str, make_executable: bool| {
        let path = PathBuf::from(file_path);
        let mut file = match create_file(path, make_executable) {
            Ok(f) => f,
            Err(_) => return Err(format!("Failed to create file {}", file_path)),
        };

        match file.write_all(contents.as_bytes()) {
            Ok(_) => Ok(()),
            Err(_) => Err(format!("Failed to write contents to {}", file_path)),
        }
    }
}

pub fn get_file_existence_checker() -> fn(file_path: &str) -> Result<bool, ()> {
    |file_path: &str| Ok(Path::new(file_path).exists())
}

pub fn get_file_reader() -> fn(file_path: &str) -> Result<String, ()> {
    |file_path: &str| {
        let mut file = match File::open(&Path::new(file_path)) {
            Err(_) => return Err(()),
            Ok(file) => file,
        };
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Err(_) => Err(()),
            Ok(_) => Ok(contents),
        }
    }
}

pub fn get_logger() -> fn(message: &str) {
    |message: &str| println!("{}", message)
}
