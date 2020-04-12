mod rusty_hook;

use std::process::exit;

extern crate ci_info;
extern crate getopts;
extern crate nias;
use getopts::Options;
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
        nias::get_command_runner(),
        nias::get_file_writer(),
        nias::get_file_existence_checker(),
    ) {
        eprintln!(
            "Fatal error encountered during initialization. Details: {}",
            err
        );
        exit(1);
    };
}

fn run(args: Vec<String>) {
    let mut opts = Options::new();
    opts.optopt("h", "hook", "the git hook name", "NAME");
    let matches = match opts.parse(&args[2..]) {
        Ok(m) => m,
        Err(_) => {
            eprintln!("Error parsing options");
            exit(1);
        }
    };

    if !matches.opt_present("h") {
        eprintln!("Hook name option missing");
        exit(1);
    }

    let hook_name = matches.opt_str("h").unwrap();

    if let Err(err) = rusty_hook::run(
        nias::get_command_runner(),
        nias::get_file_existence_checker(),
        nias::get_file_reader(),
        nias::get_conditional_logger(),
        &hook_name,
    ) {
        match err {
            Some(e) if e == rusty_hook::NO_CONFIG_FILE_FOUND => {
                exit(rusty_hook::NO_CONFIG_FILE_FOUND_ERROR_CODE);
            }
            Some(e) => {
                eprintln!("{}", e);
                exit(1);
            }
            None => exit(1),
        }
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
