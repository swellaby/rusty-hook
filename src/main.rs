mod closures;
mod rusty_hook;

use std::process::exit;

extern crate ci_info;
// extern crate getopts;
// use getopts::Options;
use std::env;

fn print_version() {
    println!(env!("CARGO_PKG_VERSION"));
}

fn init(_args: Vec<String>) {
    if ci_info::is_ci() {
        println!("CI Environment detected. Skipping hook install");
        exit(0);
    }

    if let Err(err) = rusty_hook::init(
        &closures::get_command_runner(),
        &closures::get_file_writer(),
        &closures::get_file_existence_checker(),
    ) {
        eprintln!(
            "Fatal error encountered during initialization. Details: {}",
            err
        );
        exit(1);
    };
}

fn run(_args: Vec<String>) {
    if let Err(err) = rusty_hook::run(
        &closures::get_command_runner(),
        &closures::get_file_existence_checker(),
        &closures::get_file_reader(),
        &closures::get_logger(),
        "",
    ) {
        eprintln!(
            "Fatal error encountered during initialization. Details: {}",
            err
        );
        exit(1);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let first_opt = args[1].clone();
    match first_opt.as_ref() {
        "-v" => print_version(),
        "--version" => print_version(),
        "init" => init(args),
        "run" => run(args),
        _ => panic!("Unknown command or option: {}", first_opt),
    };
}
