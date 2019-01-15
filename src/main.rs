mod rusty_hook;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Command;

fn get_command_runner() -> fn(cmd: &str) -> Result<String, String> {
    |cmd: &str| {
        let mut program = "sh";
        let mut switch = "-c";
        if cfg!(target_os = "windows") {
            program = "cmd";
            switch = "/C";
        };
        match Command::new(&program).args(&[&switch, cmd]).output() {
            Err(details) => panic!(
                "Command runner crashed in unrecoverable manner. Details: {}",
                details
            ),
            Ok(output) => {
                if output.status.success() {
                    Ok(String::from_utf8(output.stdout).unwrap().replace("\n", ""))
                } else {
                    Err(String::from_utf8(output.stderr).unwrap())
                }
            }
        }
    }
}

fn get_file_writer() -> fn(file_path: &str, contents: &str) {
    |file_path: &str, contents: &str| {
        let path = PathBuf::from(file_path);
        let display = path.display();
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why.description()),
            Ok(file) => file,
        };

        if let Err(why) = file.write_all(contents.as_bytes()) {
            panic!("couldn't write to {}: {}", display, why.description())
        }
    }
}

fn main() {
    println!("Work in progress!");
    rusty_hook::init(&get_command_runner(), &get_file_writer());
}
