use crate::command::find_selected_binary;
use crate::command::find_selected_version;
use crate::config;
use crate::definitions::Definitions;
use std::env;
use std::os::unix::process::CommandExt;
use std::process;
use std::process::Command;

pub fn run(command: &str, command_args: &[String]) {
    let definitions =
        Definitions::load_or_default(&config::definitions_file()).expect("TODO: better errors");

    match find_selected_binary(&definitions, command) {
        Some(bin) => {
            let err = Command::new(&bin).args(command_args).exec();

            let pretty_command_version = match find_selected_version(command) {
                Some(version) => version,
                None => "(not set, falling back on system version)".to_string(),
            };

            // Since we're calling exec, either our process will be replaced
            // (and this code will never be called) or something's wrong and
            // we get this error
            eprintln!(
                "🔥 alt failed to run {} version {}!",
                command, pretty_command_version
            );
            eprintln!("error: {:?}", err);
            eprintln!("command: {}", command);
            eprintln!("command version: {}", pretty_command_version);
            eprintln!("args: {:?}", command_args);
            eprintln!("bin: {}", bin.display());
            eprintln!("current dir: {:?}", env::current_dir());
            panic!();
        }
        None => {
            println!("command not found: {}", command);
            process::exit(1)
        }
    }
}
