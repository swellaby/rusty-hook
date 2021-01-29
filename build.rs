#[allow(dead_code)]
#[path = "src/rusty_hook.rs"]
mod rusty_hook;

use std::env;
use std::process::exit;

fn main() {
    if ci_info::is_ci() {
        exit(0);
    };

    let target_directory = env::var("OUT_DIR").unwrap();
    if let Err(err) = rusty_hook::init_directory(
        nias::get_command_runner(),
        nias::get_file_writer(),
        nias::get_file_existence_checker(),
        Some(&target_directory),
    ) {
        println!(
            "Fatal error encountered during initialization. Details: {}",
            err
        );
        exit(1);
    };
    exit(0);
}
