mod commands;
use commands::index::Index;
use commands::init::Init;
use commands::traits::Command;
mod consts;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    //let _ = commands::init::init();
    let args: Vec<String> = env::args().collect();
    let rest = if args.len() > 2 { &args[2..] } else { &[] };

    let result = match args.get(1) {
        Some(cmd) if cmd == Init.get_command() => Init.process_command(rest),
        Some(cmd) if cmd == Index.get_command() => Index.process_command(rest),
        Some(_) => return ExitCode::FAILURE, //IN FUTURE call help
        None => return ExitCode::FAILURE,    //same here
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {}", err);
            ExitCode::FAILURE
        }
    }
}
