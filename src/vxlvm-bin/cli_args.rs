use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, name = "vxlvm")]
#[clap(about = "The virtual machine for executing xvl files.")]
pub struct CLIArgs {
    /// The file to execute
    #[clap(min_values = 1)]
    pub input_file: String,
}
