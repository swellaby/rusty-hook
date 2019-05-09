#[allow(dead_code)]
#[path = "src/rusty_hook.rs"]
mod rusty_hook;

#[allow(dead_code)]
#[path = "src/closures.rs"]
mod closures;

use std::env;
use std::process::exit;

fn main() {
    if ci_info::is_ci() {
        exit(0);
    };

    let target_directory = env::var("OUT_DIR").unwrap();
    if let Err(err) = rusty_hook::init_directory(
        &closures::get_command_runner(),
        &closures::get_file_writer(),
        &closures::get_file_existence_checker(),
        &target_directory,
    ) {
        println!(
            "Fatal error encountered during initialization. Details: {}",
            err
        );
    };
    exit(0);
}
