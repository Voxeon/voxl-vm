mod cli_args;
mod file_operations;
mod handler;

use clap::StructOpt;
use cli_args::CLIArgs;
use file_operations::execute_file;

fn main() {
    let cli_args = CLIArgs::parse();

    match execute_file(&cli_args.input_file) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
