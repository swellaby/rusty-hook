use std::env;
use std::process::exit;

use clap::{crate_authors, Clap};

mod rusty_hook;

#[derive(Clap)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = crate_authors!())]
struct RustyHookOpts {
    #[clap(subcommand)]
    subcmd: RustyHookSubCommand,
}

#[derive(Clap)]
enum RustyHookSubCommand {
    /// Initialize rusty-hook's git hooks in the current directory.
    #[clap(version = env!("CARGO_PKG_VERSION"), author = crate_authors!())]
    Init,
    /// Run a git hook using the current directory's configuration.
    /// Ran automatically by rusty-hook's git hooks.
    #[clap(version = env!("CARGO_PKG_VERSION"), author = crate_authors!())]
    Run(RustyHookRun),
}

#[derive(Clap)]
struct RustyHookRun {
    #[clap(long)]
    hook: String,
    #[clap(name = "git args", raw(true))]
    args: Vec<String>,
}

fn init() {
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

fn run(hook: String, args: Vec<String>) {
    if let Err(err) = rusty_hook::run(
        nias::get_command_runner(),
        nias::get_file_existence_checker(),
        nias::get_file_reader(),
        nias::get_conditional_logger(),
        &hook,
        args,
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
    let opts = RustyHookOpts::parse();
    match opts.subcmd {
        RustyHookSubCommand::Init => init(),
        RustyHookSubCommand::Run(run_cmd) => run(run_cmd.hook, run_cmd.args),
    }
}
