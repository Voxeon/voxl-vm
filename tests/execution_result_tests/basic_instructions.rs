use std::cell::RefCell;
use std::rc::Rc;

use voxl_instruction_set::instruction_arguments::Register;
use vxlvm::error::VMError;
use vxlvm::validator::{BulkValidator, Validator};
use vxlvm::vm::VM;

use super::handler::System;

#[test]
fn test_single_load_byte() {
    // ldb 63, $r0
    let bytes: [u8; 10] = [
        0b0000_0010, // ldb
        0b0011_1111, // 63
        0b0000_0100,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_0000, // r0
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes.to_vec());

    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 63);
}

#[test]
fn test_single_load_integer() {
    // ldi 663, $r0
    let bytes: [u8; 10] = [
        0b0000_0011, // ldi
        0b0110_0011, // 663
        0b0000_0110,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_0000, // r0
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes.to_vec());

    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0x663);
}

#[test]
fn test_load_move_byte() {
    let bytes: Vec<u8> = vec![
        // ldb 63, $r0
        0b0000_0010, // ldb
        0b0011_1111, // 63
        0b0000_0100,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_0000, // r0
        // mov $r1, $0 (move r0 -> r1)
        0b0000_0101, // mov
        0b0111_0110, // r1, r0
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes);
    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 63);

    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 63);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 63);
}

#[test]
fn test_push_integer() {
    // ldi 663, $r0
    // push $r0
    let bytes: Vec<u8> = vec![
        0b0000_0011, // ldi
        0b0110_0011, // 663
        0b0000_0110,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_0000, // r0
        0b0000_0110, // push
        0b0110_0000,
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes);
    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    vm.run_next(&mut handler).unwrap();
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0x663);
    assert_eq!(vm.registers().get_value(Register::RFP as u8), 8);
    assert_eq!(vm.stack().get_top_u64(8).unwrap(), 0x663);
}

#[test]
fn test_pop_integer() {
    // ldi 0x663, $r0
    // push $r0
    // pop $r1
    let bytes: Vec<u8> = vec![
        0b0000_0011, // ldi
        0b0110_0011, // 663
        0b0000_0110,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_0000, // r0
        0b0000_0110, // push
        0b0110_0000,
        0b0000_0111, // pop
        0b0111_0000,
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes);

    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    vm.run_next(&mut handler).unwrap();
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0x663);
    assert_eq!(vm.registers().get_value(Register::RFP as u8), 8);
    assert_eq!(vm.stack().get_top_u64(8).unwrap(), 0x663);

    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0x663);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 0x663);
    assert_eq!(vm.registers().get_value(Register::RFP as u8), 0);
    assert_eq!(vm.stack().get_top_u64(8).unwrap(), 0x663);
}

#[test]
fn test_sget() {
    // ldi 0x663, $r0
    // push $r0
    // ldi 0x92, $r0
    // push $r0
    // ldi 8, $r1
    // sget $r0, $r1
    let bytes: Vec<u8> = vec![
        0b0000_0011, // ldi
        0b0110_0011, // 663
        0b0000_0110,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_0000, // r0
        0b0000_0110, // push
        0b0110_0000,
        0b0000_0011, // ldi
        0b1001_0010, // 92
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_0000, // r0
        0b0000_0110, // push
        0b0110_0000,
        0b0000_0011, // ldi
        0b0000_1000, // 8
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0111_0000, // r1
        0b0000_1000, //sget
        0b0110_0111, // r0, r1
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes);
    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    vm.run_next(&mut handler).unwrap();
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0x663);
    assert_eq!(vm.registers().get_value(Register::RFP as u8), 8);
    assert_eq!(vm.stack().get_top_u64(8).unwrap(), 0x663);

    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0x92);
    assert_eq!(vm.registers().get_value(Register::RFP as u8), 8);
    assert_eq!(vm.stack().get_top_u64(8).unwrap(), 0x663);

    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0x92);
    assert_eq!(vm.registers().get_value(Register::RFP as u8), 16);
    assert_eq!(vm.stack().get_top_u64(8).unwrap(), 0x663);
    assert_eq!(vm.stack().get_top_u64(16).unwrap(), 0x92);

    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R1 as u8), 8);

    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0x663);
    assert_eq!(vm.registers().get_value(Register::RFP as u8), 16);
    assert_eq!(vm.stack().get_top_u64(8).unwrap(), 0x663);
}

#[test]
fn test_syscall() {
    // ldi 0x663, $r0
    // syscall 2
    let bytes: Vec<u8> = vec![
        0b0000_0011, // ldi
        0b0110_0011, // 663
        0b0000_0110,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_0000, // r0
        0b0000_0001, // syscall
        0x2,         // 2
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
    ];

    let buffer = Rc::new(RefCell::new(Vec::new()));

    let mut handler = System::with_buffer(buffer.clone());
    let validator = BulkValidator::with_bytes(bytes);
    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0x663);

    vm.run_next(&mut handler).unwrap();

    let cp = buffer.borrow().clone();
    assert_eq!(cp, vec!["1635\n".to_string()]);
    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0x663);
    assert_eq!(vm.registers().get_value(Register::ROU as u8), 0);
}

#[test]
fn test_halt() {
    // halt
    // ldi $r0, 0x663
    let bytes: Vec<u8> = vec![
        0x45,        // halt
        0b0000_0011, // ldi
        0b0110_0011, // 663
        0b0000_0110,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_0000, // r0
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes);
    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    // halt
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);

    assert_eq!(
        vm.run_next(&mut handler).unwrap_err(),
        VMError::SystemHalted
    );
}
