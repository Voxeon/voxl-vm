use super::handler::System;
use voxl_instruction_set::instruction_arguments::Register;
use vxlvm::loader::Loader;
use vxlvm::validator::BulkValidator;
use vxlvm::vm::VM;

#[test]
fn test_full_file_single_load() {
    // See the examples in the vxasm repository, this is the ld.vsm
    let bytes = hex::decode("6558564c000a00000000000000000000000000000001b27acab0c6ccc799475b2b9dd8e5eb8689dff72b4e044bf3c6582727aa033f0000000000000060").unwrap();

    let mut handler = System::new();
    let (_header, instructions) = Loader::load_bytes(&bytes)
        .unwrap()
        .to_instructions(BulkValidator::new())
        .unwrap();

    let mut vm = VM::new(instructions);

    vm.run(&mut handler).unwrap();
    assert_eq!(vm.registers().get_value(Register::R0 as u8), 63);
}

#[test]
fn test_power() {
    // See the examples in the vxasm repository, this is the ld.vsm
    let bytes = hex::decode("6558564c005b00000000000000000000000000000001c58043c2580e23d9465f0b1558d09b9bfc1ee1105a08417c6f6839e7aa030400000000000000600304000000000000007043040000000000000045020100000000000000200586056702000000000000000070020100000000000000f034763a0e000000000000002122801f77f037090000000000000044").unwrap();

    let mut handler = System::new();
    let (_header, instructions) = Loader::load_bytes(&bytes)
        .unwrap()
        .to_instructions(BulkValidator::new())
        .unwrap();

    let mut vm = VM::new(instructions);

    vm.run(&mut handler).unwrap();
    assert_eq!(vm.registers().get_value(Register::ROU as u8), 256);
}
