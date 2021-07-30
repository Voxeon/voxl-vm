use crate::handler::OSHandler;

use voxl_instruction_set::instruction::Instruction;
use vxlvm::error::VXLVMError;
use vxlvm::loader::Loader;
use vxlvm::validator::BulkValidator;
use vxlvm::vm::VM;

use std::fs::OpenOptions;
use std::io::Read;

fn load_file(path: &str) -> Result<Vec<Instruction>, String> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(path)
        .map_err(|e| format!("{}", e))?;

    let mut contents = Vec::new();

    file.read_to_end(&mut contents)
        .map_err(|e| format!("Cannot read file {}. OS Error: {}", path, e))?;

    let loader = Loader::load_bytes(&contents).map_err(|e| {
        if cfg!(feature = "detailed_errors") {
            format!("{}", e.specific_description())
        } else {
            format!("{}", e.short_description())
        }
    })?;

    loader.validate().map_err(|e| {
        if cfg!(feature = "detailed_errors") {
            format!("{}", e.specific_description())
        } else {
            format!("{}", e.short_description())
        }
    })?;

    return loader
        .to_instructions(BulkValidator::new())
        .map_err(|e| {
            if cfg!(feature = "detailed_errors") {
                format!("{}", e.specific_description())
            } else {
                format!("{}", e.short_description())
            }
        })
        .map(|(_header, instructions)| instructions);
}

pub fn execute_file(path: &str) -> Result<(), String> {
    let mut handler = OSHandler::new();
    let mut machine = VM::new(load_file(path)?);

    if let Err(e) = machine.run(&mut handler) {
        if cfg!(feature = "detailed_errors") {
            return Err(e.specific_description());
        } else {
            return Err(e.short_description());
        }
    }

    return Ok(());
}