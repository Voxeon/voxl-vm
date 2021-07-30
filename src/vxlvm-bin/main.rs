mod handler;
mod file_operations;

use clap::{App, Arg};
use file_operations::execute_file;

fn main() {
    let matches = App::new("vxlvm")
        .version("0.1.0")
        .about("The virtual machine for executing xvl files.")
        .arg(
            Arg::with_name("input-file")
                .takes_value(true)
                .multiple(false)
                .required(true),
        )
        .get_matches();

    let file = matches
        .value_of("input-file")
        .expect("An executable file to run is required!");

    match execute_file(file) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
