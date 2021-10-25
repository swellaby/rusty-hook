use std::env;
use std::process::exit;

use getopts::{Matches, Options};

mod rusty_hook;

const INIT_BRIEF: &str = r#"
Usage:
    rusty-hook init
"#;
const RUN_BRIEF: &str = r#"
Usage:
    rusty-hook run --hook [NAME] [git_args]
    
Args:
    <git_args>      Arguments that will replace "%rh!" token in hook
"#;
const DEFAULT_BRIEF: &str = r#"
Usage:
    rusty-hook <SUBCOMMAND>

Subcommands:
    run     Run a git hook using the current directory's configuration. Ran automatically by rusty-hook's git hooks
    init    Initialize rusty-hook's git hooks in the current directory
"#;

fn print_version() {
    println!(env!("CARGO_PKG_VERSION"));
}

fn print_usage(brief: &str, opts: &Options) {
    print!("{}", opts.usage(brief));
}

fn parse_list_args(opts: &Options, args: &[String]) -> Matches {
    match opts.parse(args) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}

fn init(matches: Matches) {
    if ci_info::is_ci() {
        println!("[rusty-hook] CI Environment detected. Skipping hook install");
        exit(0);
    }

    let skip_hook_list = matches.free;

    if let Err(err) = rusty_hook::init(
        nias::get_command_runner(),
        nias::get_file_writer(),
        nias::get_file_existence_checker(),
        &skip_hook_list,
    ) {
        eprintln!(
            "[rusty] Fatal error encountered during initialization. Details: {}",
            err
        );
        exit(1);
    };
}

fn run(matches: Matches) {
    if !matches.opt_present("hook") {
        eprintln!("Hook name option missing");
        exit(1);
    }

    let hook = matches.opt_str("hook").unwrap();

    let args = if matches.free.is_empty() {
        None
    } else {
        Some(matches.free.join(" "))
    };

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
                eprintln!("[rusty-hook] {}", e);
                exit(1);
            }
            None => exit(1),
        }
    }
}

fn parse_args(args: &[String], opts: &mut Options) -> (Matches, String) {
    let first_opt = &args[1];

    let (arg_slice, brief_ref) = match first_opt.as_ref() {
        "init" => {
            opts.optopt(
                "",
                "skip_hook_list",
                "A hook that should be skipped",
                "SKIP_HOOK_LIST",
            );

            (&args[2..], INIT_BRIEF)
        }
        "run" => {
            opts.optopt("", "hook", "The git hook name", "NAME");

            (&args[2..], RUN_BRIEF)
        }
        _ => (&args[1..], DEFAULT_BRIEF),
    };

    let matches = parse_list_args(opts, arg_slice);

    (matches, brief_ref.to_string())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("h", "help", "Print help information");
    opts.optflag("V", "version", "Print version information");

    let (matches, brief) = parse_args(&args, &mut opts);

    if matches.opt_present("h") {
        print_usage(&brief, &opts);
        exit(0)
    };

    if matches.opt_present("V") {
        print_version();
        exit(0)
    };

    match args[1].as_ref() {
        "init" => init(matches),
        "run" => run(matches),
        cmd => eprintln!("Unknown command: {}", cmd),
    }
}
