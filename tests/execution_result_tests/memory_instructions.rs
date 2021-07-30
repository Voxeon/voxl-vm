use voxl_instruction_set::instruction_arguments::Register;
use vxlvm::validator::{BulkValidator, Validator};
use vxlvm::vm::VM;

use super::handler::System;

#[test]
fn test_malloc() {
    // ldi 32, $r1
    // malloc $r0, $r1
    // malloc $r0, $r1
    let bytes: Vec<u8> = vec![
        0b0000_0011, // ldi
        0b0010_0000, // 32
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0111_0000, // r1
        0b0000_1001, // malloc
        0b0110_0111, // r0, r1
        0b0000_1001, // malloc
        0b0110_0111, // r0, r1
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes);
    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R1 as u8), 32);
    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);

    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R1 as u8), 32);
    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 32]);

    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R1 as u8), 32);
    assert_eq!(vm.registers().get_value(Register::R0 as u8), 1);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 32]);
    assert_eq!(vm.memory().retrieve(&1).unwrap(), &vec![0u8; 32]);
}

#[test]
fn test_malloci() {
    // malloc $r0, 32
    // malloc $r0, 32
    let bytes: Vec<u8> = vec![
        0b0000_1010, // malloci
        0b0010_0000, // 32
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_0000, // r0
        0b0000_1010, // malloc
        0b0010_0000, // 32
        0x0,
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

    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 32]);

    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 1);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 32]);
    assert_eq!(vm.memory().retrieve(&1).unwrap(), &vec![0u8; 32]);
}

#[test]
fn test_free() {
    // ldi 32, $r1
    // malloc $r0, $r1
    // free $r0
    // malloc $r0, $r1
    // free $r0
    let bytes: Vec<u8> = vec![
        0b0000_0011, // ldi
        0b0010_0000, // 32
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0111_0000, // r1
        0b0000_1001, // malloc
        0b0110_0111, // r0, r1
        0b0000_1011, // free
        0b0110_0000, // r0
        0b0000_1001, // malloc
        0b0110_0111, // r0, r1
        0b0000_1011, // free
        0b0110_0000, // r0
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes);
    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    // ldi
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R1 as u8), 32);
    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);

    // malloc
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R1 as u8), 32);
    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 32]);

    // free
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R1 as u8), 32);
    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.memory().total_allocated(), 0);

    // malloc
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R1 as u8), 32);
    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 32]);

    // free
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R1 as u8), 32);
    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.memory().total_allocated(), 0);
}

#[test]
fn test_freea() {
    // ldi 32, $r1
    // malloc $r0, $r1
    // malloc $r0, $r1
    // free 0
    // free 1
    let bytes: Vec<u8> = vec![
        0b0000_0011, // ldi
        0b0010_0000, // 32
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0111_0000, // r1
        0b0000_1001, // malloc
        0b0110_0111, // r0, r1
        0b0000_1001, // malloc
        0b0110_0111, // r0, r1
        0b0000_1100, // freea
        0x0,         // 0
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0000_1100, // freea
        0b0000_0001, // 1
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes);
    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    // ldi
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R1 as u8), 32);
    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);

    // malloc
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R1 as u8), 32);
    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 32]);

    // malloc
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R1 as u8), 32);
    assert_eq!(vm.registers().get_value(Register::R0 as u8), 1);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 32]);
    assert_eq!(vm.memory().retrieve(&1).unwrap(), &vec![0u8; 32]);

    // freea
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R1 as u8), 32);
    assert_eq!(vm.registers().get_value(Register::R0 as u8), 1);
    assert_eq!(vm.memory().total_allocated(), 32);

    // freea
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R1 as u8), 32);
    assert_eq!(vm.registers().get_value(Register::R0 as u8), 1);
    assert_eq!(vm.memory().total_allocated(), 0);
}

#[test]
fn test_setb() {
    // malloci $r0, 10
    // ldi $r1, 0
    // ldb $r2, 15
    // setb $r0, $r1, $r2
    let bytes: Vec<u8> = vec![
        0b0000_1010, // malloci
        0b0000_1010, // 10
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_0000, // r0
        0b0000_0011, // ldi
        0x0,         // 0
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0111_0000, // r1
        0b0000_0010, // ldb
        0b0000_1111, // 15
        0b0000_0100,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b1000_0000, // r2
        0b0000_1101, // setb
        0b0110_0111, // r0, r1
        0b1000_0000, // r2
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes);
    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    // malloci $r0, 32
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);

    // ldi $r1, 0
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 0);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);

    // ldb $r2, 15
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R2 as u8), 15);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);

    // setb $r0, $r1, $r2
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R2 as u8), 15);
    assert_eq!(
        vm.memory().retrieve(&0).unwrap(),
        &vec![15, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
}

#[test]
fn test_setb_2() {
    // malloci $r0, 10
    // ldi $r1, 2
    // ldb $r2, 15
    // setb $r0, $r1, $r2
    let bytes: Vec<u8> = vec![
        0b0000_1010, // malloci
        0b0000_1010, // 10
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_0000, // r0
        0b0000_0011, // ldi
        0b0000_0010, // 2
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0111_0000, // r1
        0b0000_0010, // ldb
        0b0000_1111, // 15
        0b0000_0100,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b1000_0000, // r2
        0b0000_1101, // setb
        0b0110_0111, // r0, r1
        0b1000_0000, // r2
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes);
    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    // malloci $r0, 32
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);

    // ldi $r1, 2
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 2);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);

    // ldb $r2, 15
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 2);
    assert_eq!(vm.registers().get_value(Register::R2 as u8), 15);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);

    // setb $r0, $r1, $r2
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 2);
    assert_eq!(vm.registers().get_value(Register::R2 as u8), 15);
    assert_eq!(
        vm.memory().retrieve(&0).unwrap(),
        &vec![0, 0, 15, 0, 0, 0, 0, 0, 0, 0]
    );
}

#[test]
fn test_seti() {
    // malloci $r0, 10
    // ldi $r1, 1
    // ldi $r2, 15782
    // seti $r0, $r1, $r2
    let bytes: Vec<u8> = vec![
        0b0000_1010, // malloci
        0b0000_1010, // 10
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_0000, // r0
        0b0000_0011, // ldi
        0b0000_0001, // 1
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0111_0000, // r1
        0b0000_0011, // ldi
        0b1010_0110, // 15782
        0b0011_1101,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b1000_0000, // r2
        0b0000_1110, // seti
        0b0110_0111, // r0, r1
        0b1000_0000, // r2
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes);
    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    // malloci $r0, 32
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);

    // ldi $r1, 1
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 1);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);

    // ldi $r2, 15782
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 1);
    assert_eq!(vm.registers().get_value(Register::R2 as u8), 15782);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);

    // seti $r0, $r1, $r2
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 1);
    assert_eq!(vm.registers().get_value(Register::R2 as u8), 15782);
    assert_eq!(
        vm.memory().retrieve(&0).unwrap(),
        &vec![0, 0xa6, 0x3d, 0, 0, 0, 0, 0, 0, 0]
    );
}

#[test]
fn test_isetb() {
    // malloci $r0, 10
    // ldb $r1, 15
    // isetb 2, $r0, $r1
    let bytes: Vec<u8> = vec![
        0b0000_1010, // malloci
        0b0000_1010, // 10
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_0000, // r0
        0b0000_0010, // ldb
        0b0000_1111, // 15
        0b0000_0100,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0111_0000, // r1
        0b0000_1111, // isetb
        0b0000_0010, // 2
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_0111, // r0, r1
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes);
    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    // malloci $r0, 32
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);

    // ldb $r1, 15
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 15);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);

    // isetb 2, $r0, $r1
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 15);
    assert_eq!(
        vm.memory().retrieve(&0).unwrap(),
        &vec![0, 0, 15, 0, 0, 0, 0, 0, 0, 0]
    );
}

#[test]
fn test_iseti() {
    // malloci $r0, 10
    // ldi $r1, 15782
    // iseti 1, $r0, $r1
    let bytes: Vec<u8> = vec![
        0b0000_1010, // malloci
        0b0000_1010, // 10
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_0000, // r0
        0b0000_0011, // ldi
        0b1010_0110, // 15782
        0b0011_1101,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0111_0000, // r1
        0b0001_0000, // iseti
        0b0000_0001, // 1
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_0111, // r0, r1
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes);
    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    // malloci $r0, 32
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);

    // ldi $r1, 15782
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 15782);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);

    // iseti 1, $r0, $r1
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 15782);
    assert_eq!(
        vm.memory().retrieve(&0).unwrap(),
        &vec![0, 0xa6, 0x3d, 0, 0, 0, 0, 0, 0, 0]
    );
}

#[test]
fn test_getb() {
    // malloci $r0, 10
    // ldi $r1, 2
    // ldb $r2, 15
    // setb $r0, $r1, $r2
    // getb $r0, $r0, $r1
    let bytes: Vec<u8> = vec![
        0b0000_1010, // malloci
        0b0000_1010, // 10
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_0000, // r0
        0b0000_0011, // ldi
        0b0000_0010, // 2
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0111_0000, // r1
        0b0000_0010, // ldb
        0b0000_1111, // 15
        0b0000_0100,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b1000_0000, // r2
        0b0000_1101, // setb
        0b0110_0111, // r0, r1
        0b1000_0000, // r2
        0b0001_0001, // getb
        0b0110_0110, // r0, r0
        0b0111_0000, // r1
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes);
    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    // malloci $r0, 32
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);

    // ldi $r1, 2
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 2);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);

    // ldb $r2, 15
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 2);
    assert_eq!(vm.registers().get_value(Register::R2 as u8), 15);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);

    // setb $r0, $r1, $r2
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 2);
    assert_eq!(vm.registers().get_value(Register::R2 as u8), 15);
    assert_eq!(
        vm.memory().retrieve(&0).unwrap(),
        &vec![0, 0, 15, 0, 0, 0, 0, 0, 0, 0]
    );

    // getb $r0, $r0, $r1
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 15);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 2);
    assert_eq!(vm.registers().get_value(Register::R2 as u8), 15);
    assert_eq!(
        vm.memory().retrieve(&0).unwrap(),
        &vec![0, 0, 15, 0, 0, 0, 0, 0, 0, 0]
    );
}

#[test]
fn test_geti() {
    // malloci $r0, 10
    // ldi $r1, 1
    // ldi $r2, 15782
    // seti $r0, $r1, $r2
    // geti $r0, $r0, $r1
    let bytes: Vec<u8> = vec![
        0b0000_1010, // malloci
        0b0000_1010, // 10
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_0000, // r0
        0b0000_0011, // ldi
        0b0000_0001, // 1
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0111_0000, // r1
        0b0000_0011, // ldi
        0b1010_0110, // 15782
        0b0011_1101,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b1000_0000, // r2
        0b0000_1110, // seti
        0b0110_0111, // r0, r1
        0b1000_0000, // r2
        0b0001_0010, // geti
        0b0110_0110, // r0, r0
        0b0111_0000, // r1
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes);
    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    // malloci $r0, 32
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);

    // ldi $r1, 1
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 1);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);

    // ldi $r2, 15782
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 1);
    assert_eq!(vm.registers().get_value(Register::R2 as u8), 15782);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);

    // seti $r0, $r1, $r2
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 1);
    assert_eq!(vm.registers().get_value(Register::R2 as u8), 15782);
    assert_eq!(
        vm.memory().retrieve(&0).unwrap(),
        &vec![0, 0xa6, 0x3d, 0, 0, 0, 0, 0, 0, 0]
    );

    // geti $r0, $r0, $r1
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 15782);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 1);
    assert_eq!(vm.registers().get_value(Register::R2 as u8), 15782);
    assert_eq!(
        vm.memory().retrieve(&0).unwrap(),
        &vec![0, 0xa6, 0x3d, 0, 0, 0, 0, 0, 0, 0]
    );
}

#[test]
fn test_last_1() {
    // malloci $r0, 10
    // ldi $r1, 9
    // ldb $r2, 15
    // setb $r0, $r1, $r2
    let bytes: Vec<u8> = vec![
        0b0000_1010, // malloci
        0b0000_1010, // 10
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_0000, // r0
        0b0000_0011, // ldi
        0b0000_1001, // 9
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0111_0000, // r1
        0b0000_0010, // ldb
        0b0000_1111, // 15
        0b0000_0100,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b1000_0000, // r2
        0b0000_1101, // setb
        0b0110_0111, // r0, r1
        0b1000_0000, // r2
        0b0001_0101, // last
        0b0110_0110, // r0, r0
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes);
    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    // malloci $r0, 32
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);

    // ldi $r1, 9
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 9);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);

    // ldb $r2, 15
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 9);
    assert_eq!(vm.registers().get_value(Register::R2 as u8), 15);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);

    // setb $r0, $r1, $r2
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 9);
    assert_eq!(vm.registers().get_value(Register::R2 as u8), 15);
    assert_eq!(
        vm.memory().retrieve(&0).unwrap(),
        &vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 15]
    );

    // last $r0, $r0
    vm.run_next(&mut handler).unwrap();

    assert_eq!(
        vm.registers().get_value(Register::R0 as u8),
        u64::from_le_bytes([0, 0, 0, 0, 0, 0, 0, 15])
    );
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 9);
    assert_eq!(vm.registers().get_value(Register::R2 as u8), 15);
    assert_eq!(
        vm.memory().retrieve(&0).unwrap(),
        &vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 15]
    );
}

#[test]
fn test_last_2() {
    // malloci $r0, 10
    // ldi $r1, 2
    // ldb $r2, 15
    // setb $r0, $r1, $r2
    let bytes: Vec<u8> = vec![
        0b0000_1010, // malloci
        0b0000_1010, // 10
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_0000, // r0
        0b0000_0011, // ldi
        0b0000_0010, // 2
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0111_0000, // r1
        0b0000_0010, // ldb
        0b0000_1111, // 15
        0b0000_0100,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b1000_0000, // r2
        0b0000_1101, // setb
        0b0110_0111, // r0, r1
        0b1000_0000, // r2
        0b0001_0101, // last
        0b0110_0110, // r0, r0
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes);
    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    // malloci $r0, 32
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);

    // ldi $r1, 9
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 2);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);

    // ldb $r2, 15
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 2);
    assert_eq!(vm.registers().get_value(Register::R2 as u8), 15);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);

    // setb $r0, $r1, $r2
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 2);
    assert_eq!(vm.registers().get_value(Register::R2 as u8), 15);
    assert_eq!(
        vm.memory().retrieve(&0).unwrap(),
        &vec![0, 0, 15, 0, 0, 0, 0, 0, 0, 0]
    );

    // last $r0, $r0
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 15);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 2);
    assert_eq!(vm.registers().get_value(Register::R2 as u8), 15);
    assert_eq!(
        vm.memory().retrieve(&0).unwrap(),
        &vec![0, 0, 15, 0, 0, 0, 0, 0, 0, 0]
    );
}

#[test]
fn test_last_3() {
    // malloci $r0, 4
    // ldi $r1, 3
    // ldb $r2, 15
    // setb $r0, $r1, $r2
    // last $r0, $r0
    let bytes: Vec<u8> = vec![
        0b0000_1010, // malloci
        0b0000_0100, // 4
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_0000, // r0
        0b0000_0011, // ldi
        0b0000_0011, // 3
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0111_0000, // r1
        0b0000_0010, // ldb
        0b0000_1111, // 15
        0b0000_0100,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b1000_0000, // r2
        0b0000_1101, // setb
        0b0110_0111, // r0, r1
        0b1000_0000, // r2
        0b0001_0101, // last
        0b0110_0110, // r0, r0
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes);
    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    // malloci $r0, 4
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 4]);

    // ldi $r1, 3
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 3);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 4]);

    // ldb $r2, 15
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 3);
    assert_eq!(vm.registers().get_value(Register::R2 as u8), 15);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 4]);

    // setb $r0, $r1, $r2
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 3);
    assert_eq!(vm.registers().get_value(Register::R2 as u8), 15);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0, 0, 0, 15]);

    // last $r0, $r0
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 15);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 3);
    assert_eq!(vm.registers().get_value(Register::R2 as u8), 15);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0, 0, 0, 15]);
}

#[test]
fn test_length() {
    // malloci $r0, 4
    // length $r1, $r0
    let bytes: Vec<u8> = vec![
        0b0000_1010, // malloci
        0x4,         // 4
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_0000, // r0
        0x16,        // length
        0b0111_0110, // r1, r0
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes);
    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    // malloci $r0, 4
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 4]);

    // length $r1, $r0
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 4);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 4]);
}

#[test]
fn test_clone() {
    // malloci $r1, 10
    // ldi $r2, 1
    // ldi $r3, 44
    // seti $r1, $r2, $r3
    // clone $r0, $r1
    let bytes: Vec<u8> = vec![
        0b0000_1010, // malloci
        0xa,         // 10
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0111_0000, // r1
        0b0000_0011, // ldi
        0b0000_0001, // 1
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b1000_0000, // r2
        0b0000_0011, // ldi
        44,          // 44
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b1001_0000, // r3
        0b0000_1110, // seti
        0b0111_1000, // r1, r2
        0b1001_0000, // r3
        0x17,        // clone
        0b0110_0111, // r0, r1
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes);
    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    // malloci $r1, 10
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R1 as u8), 0);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);

    // ldi $r2, 1
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R1 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R2 as u8), 1);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);

    // ldi $r3, 44
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R1 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R2 as u8), 1);
    assert_eq!(vm.registers().get_value(Register::R3 as u8), 44);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);

    // seti $r1, $r2, $r3
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R1 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R2 as u8), 1);
    assert_eq!(vm.registers().get_value(Register::R3 as u8), 44);
    assert_eq!(
        vm.memory().retrieve(&0).unwrap(),
        &vec![0, 44, 0, 0, 0, 0, 0, 0, 0, 0]
    );

    // clone $r0, $r1
    vm.run_next(&mut handler).unwrap();
    assert_eq!(vm.registers().get_value(Register::R0 as u8), 1);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R2 as u8), 1);
    assert_eq!(vm.registers().get_value(Register::R3 as u8), 44);
    assert_eq!(
        vm.memory().retrieve(&0).unwrap(),
        &vec![0, 44, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(
        vm.memory().retrieve(&1).unwrap(),
        &vec![0, 44, 0, 0, 0, 0, 0, 0, 0, 0]
    );
}

#[test]
fn test_swpa() {
    // malloci $r0, 10
    // malloci $r1, 10
    // ldi $r2, 1
    // ldi $r3, 44
    // ldi $r4, 16
    // seti $r1, $r2, $r3
    // seti $r2, $r2, $r4
    // swpa 0, 1
    let bytes: Vec<u8> = vec![
        0b0000_1010, // malloci
        0xa,         // 10
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_0000, // r0
        0b0000_1010, // malloci
        0xa,         // 10
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0111_0000, // r1
        0b0000_0011, // ldi
        0b0000_0001, // 1
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b1000_0000, // r2
        0b0000_0011, // ldi
        44,          // 44
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b1001_0000, // r3
        0b0000_0011, // ldi
        16,          // 16
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b1010_0000, // r4
        0b0000_1110, // seti
        0b0110_1000, // r0, r2
        0b1001_0000, // r3
        0b0000_1110, // seti
        0b0111_1000, // r1, r2
        0b1010_0000, // r4
        0x40,        // swpa
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0, // 0
        0x1,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0, // 1
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes);
    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    // malloci $r0, 10
    vm.run_next(&mut handler).unwrap();
    // malloci $r1, 10
    vm.run_next(&mut handler).unwrap();
    // ldi $r2, 1
    vm.run_next(&mut handler).unwrap();
    // ldi $r3, 44
    vm.run_next(&mut handler).unwrap();
    // ldi $r4, 16
    vm.run_next(&mut handler).unwrap();
    // seti $r1, $r2, $r3
    vm.run_next(&mut handler).unwrap();
    // seti $r2, $r2, $r4
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 1);
    assert_eq!(vm.registers().get_value(Register::R2 as u8), 1);
    assert_eq!(vm.registers().get_value(Register::R3 as u8), 44);
    assert_eq!(vm.registers().get_value(Register::R4 as u8), 16);
    assert_eq!(
        vm.memory().retrieve(&0).unwrap(),
        &vec![0, 44, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(
        vm.memory().retrieve(&1).unwrap(),
        &vec![0, 16, 0, 0, 0, 0, 0, 0, 0, 0]
    );

    // swpa 0, 1
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 1);
    assert_eq!(vm.registers().get_value(Register::R2 as u8), 1);
    assert_eq!(vm.registers().get_value(Register::R3 as u8), 44);
    assert_eq!(vm.registers().get_value(Register::R4 as u8), 16);
    assert_eq!(
        vm.memory().retrieve(&0).unwrap(),
        &vec![0, 16, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(
        vm.memory().retrieve(&1).unwrap(),
        &vec![0, 44, 0, 0, 0, 0, 0, 0, 0, 0]
    );
}

#[test]
fn test_swpa_2() {
    // malloci $r0, 10
    // malloci $r1, 10
    // ldi $r2, 1
    // ldi $r3, 44
    // ldi $r4, 16
    // seti $r1, $r2, $r3
    // seti $r2, $r2, $r4
    // swpa 1, 0
    let bytes: Vec<u8> = vec![
        0b0000_1010, // malloci
        0xa,         // 10
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_0000, // r0
        0b0000_1010, // malloci
        0xa,         // 10
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0111_0000, // r1
        0b0000_0011, // ldi
        0b0000_0001, // 1
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b1000_0000, // r2
        0b0000_0011, // ldi
        44,          // 44
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b1001_0000, // r3
        0b0000_0011, // ldi
        16,          // 16
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b1010_0000, // r4
        0b0000_1110, // seti
        0b0110_1000, // r0, r2
        0b1001_0000, // r3
        0b0000_1110, // seti
        0b0111_1000, // r1, r2
        0b1010_0000, // r4
        0x40,        // swpa
        0x1,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0, // 1
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0, // 0
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes);
    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    // malloci $r0, 10
    vm.run_next(&mut handler).unwrap();
    // malloci $r1, 10
    vm.run_next(&mut handler).unwrap();
    // ldi $r2, 1
    vm.run_next(&mut handler).unwrap();
    // ldi $r3, 44
    vm.run_next(&mut handler).unwrap();
    // ldi $r4, 16
    vm.run_next(&mut handler).unwrap();
    // seti $r1, $r2, $r3
    vm.run_next(&mut handler).unwrap();
    // seti $r2, $r2, $r4
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 1);
    assert_eq!(vm.registers().get_value(Register::R2 as u8), 1);
    assert_eq!(vm.registers().get_value(Register::R3 as u8), 44);
    assert_eq!(vm.registers().get_value(Register::R4 as u8), 16);
    assert_eq!(
        vm.memory().retrieve(&0).unwrap(),
        &vec![0, 44, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(
        vm.memory().retrieve(&1).unwrap(),
        &vec![0, 16, 0, 0, 0, 0, 0, 0, 0, 0]
    );

    // swpa 0, 1
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 1);
    assert_eq!(vm.registers().get_value(Register::R2 as u8), 1);
    assert_eq!(vm.registers().get_value(Register::R3 as u8), 44);
    assert_eq!(vm.registers().get_value(Register::R4 as u8), 16);
    assert_eq!(
        vm.memory().retrieve(&0).unwrap(),
        &vec![0, 16, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(
        vm.memory().retrieve(&1).unwrap(),
        &vec![0, 44, 0, 0, 0, 0, 0, 0, 0, 0]
    );
}

#[test]
fn test_swpar() {
    // malloci $r0, 10
    // malloci $r1, 10
    // ldi $r2, 1
    // ldi $r3, 44
    // ldi $r4, 16
    // seti $r1, $r2, $r3
    // seti $r2, $r2, $r4
    // swpar $r0, $r1
    let bytes: Vec<u8> = vec![
        0b0000_1010, // malloci
        0xa,         // 10
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_0000, // r0
        0b0000_1010, // malloci
        0xa,         // 10
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0111_0000, // r1
        0b0000_0011, // ldi
        0b0000_0001, // 1
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b1000_0000, // r2
        0b0000_0011, // ldi
        44,          // 44
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b1001_0000, // r3
        0b0000_0011, // ldi
        16,          // 16
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b1010_0000, // r4
        0b0000_1110, // seti
        0b0110_1000, // r0, r2
        0b1001_0000, // r3
        0b0000_1110, // seti
        0b0111_1000, // r1, r2
        0b1010_0000, // r4
        0x41,        // swpar
        0b0110_0111, // r0, r1
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes);
    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    // malloci $r0, 10
    vm.run_next(&mut handler).unwrap();
    // malloci $r1, 10
    vm.run_next(&mut handler).unwrap();
    // ldi $r2, 1
    vm.run_next(&mut handler).unwrap();
    // ldi $r3, 44
    vm.run_next(&mut handler).unwrap();
    // ldi $r4, 16
    vm.run_next(&mut handler).unwrap();
    // seti $r1, $r2, $r3
    vm.run_next(&mut handler).unwrap();
    // seti $r2, $r2, $r4
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 1);
    assert_eq!(vm.registers().get_value(Register::R2 as u8), 1);
    assert_eq!(vm.registers().get_value(Register::R3 as u8), 44);
    assert_eq!(vm.registers().get_value(Register::R4 as u8), 16);
    assert_eq!(
        vm.memory().retrieve(&0).unwrap(),
        &vec![0, 44, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(
        vm.memory().retrieve(&1).unwrap(),
        &vec![0, 16, 0, 0, 0, 0, 0, 0, 0, 0]
    );

    // swpar $r0, $r1
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 1);
    assert_eq!(vm.registers().get_value(Register::R2 as u8), 1);
    assert_eq!(vm.registers().get_value(Register::R3 as u8), 44);
    assert_eq!(vm.registers().get_value(Register::R4 as u8), 16);
    assert_eq!(
        vm.memory().retrieve(&0).unwrap(),
        &vec![0, 16, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(
        vm.memory().retrieve(&1).unwrap(),
        &vec![0, 44, 0, 0, 0, 0, 0, 0, 0, 0]
    );
}

#[test]
fn test_swpar_2() {
    // malloci $r0, 10
    // malloci $r1, 10
    // ldi $r2, 1
    // ldi $r3, 44
    // ldi $r4, 16
    // seti $r1, $r2, $r3
    // seti $r2, $r2, $r4
    // swpar $r1, $r0
    let bytes: Vec<u8> = vec![
        0b0000_1010, // malloci
        0xa,         // 10
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_0000, // r0
        0b0000_1010, // malloci
        0xa,         // 10
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0111_0000, // r1
        0b0000_0011, // ldi
        0b0000_0001, // 1
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b1000_0000, // r2
        0b0000_0011, // ldi
        44,          // 44
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b1001_0000, // r3
        0b0000_0011, // ldi
        16,          // 16
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b1010_0000, // r4
        0b0000_1110, // seti
        0b0110_1000, // r0, r2
        0b1001_0000, // r3
        0b0000_1110, // seti
        0b0111_1000, // r1, r2
        0b1010_0000, // r4
        0x41,        // swpar
        0b0111_0110, // r1, r0
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes);
    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    // malloci $r0, 10
    vm.run_next(&mut handler).unwrap();
    // malloci $r1, 10
    vm.run_next(&mut handler).unwrap();
    // ldi $r2, 1
    vm.run_next(&mut handler).unwrap();
    // ldi $r3, 44
    vm.run_next(&mut handler).unwrap();
    // ldi $r4, 16
    vm.run_next(&mut handler).unwrap();
    // seti $r1, $r2, $r3
    vm.run_next(&mut handler).unwrap();
    // seti $r2, $r2, $r4
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 1);
    assert_eq!(vm.registers().get_value(Register::R2 as u8), 1);
    assert_eq!(vm.registers().get_value(Register::R3 as u8), 44);
    assert_eq!(vm.registers().get_value(Register::R4 as u8), 16);
    assert_eq!(
        vm.memory().retrieve(&0).unwrap(),
        &vec![0, 44, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(
        vm.memory().retrieve(&1).unwrap(),
        &vec![0, 16, 0, 0, 0, 0, 0, 0, 0, 0]
    );

    // swpar $r0, $r1
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 1);
    assert_eq!(vm.registers().get_value(Register::R2 as u8), 1);
    assert_eq!(vm.registers().get_value(Register::R3 as u8), 44);
    assert_eq!(vm.registers().get_value(Register::R4 as u8), 16);
    assert_eq!(
        vm.memory().retrieve(&0).unwrap(),
        &vec![0, 16, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(
        vm.memory().retrieve(&1).unwrap(),
        &vec![0, 44, 0, 0, 0, 0, 0, 0, 0, 0]
    );
}

#[test]
fn test_swpr() {
    // ldi $r0, 1
    // ldi $r1, 0
    // swpr $r0, $r1
    let bytes: Vec<u8> = vec![
        0b0000_0011, // ldi
        0b0000_0001, // 1
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_0000, // r0
        0b0000_0011, // ldi
        0x0,         // 0
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0111_0000, // r1
        0x42,        // swpr
        0b0110_0111, // r0, r1
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes);
    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    // ldi $r0, 1
    vm.run_next(&mut handler).unwrap();
    // ldi $r1, 0
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 1);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 0);

    // swpr $r0, $r1
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 1);
}

#[test]
fn test_copy() {
    // malloci $r0, 10
    // malloci $r1, 10
    // ldi $r2,
    // iseti 1, $r0, $r2
    // ldi $r2, 0
    // ldi $r3, 4
    // copy r1, r2, r0, r2, r3

    // r = destination address
    // r1 = destination offset
    // r2 = src address
    // r3 = src offset
    // r4 = number of bytes
    let bytes: Vec<u8> = vec![
        0b0000_1010, // malloci
        0xa,         // 10
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_0000, // r0
        0b0000_1010, // malloci
        0xa,         // 10
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0111_0000, // r1
        0b0000_0011, // ldi
        0x12,
        0x34,
        0x56,
        0x78,
        0x9a,
        0xbc,
        0xde,
        0xf0,
        0b1000_0000, // r2
        0b0001_0000, // iseti
        0b0000_0001, // 1
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_1000, // r0, r2
        0b0000_0011, // ldi
        0x0,         // 0
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b1000_0000, // r2
        0b0000_0011, // ldi
        0x4,         // 4
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b1001_0000, // r3
        0x18,        // copy
        0b0111_1000, // r1, r2
        0b0110_1000, // r0, r2
        0b1001_0000, // r3
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes);
    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    // malloci $r0, 10
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);

    // malloci $r1, 10
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 1);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);
    assert_eq!(vm.memory().retrieve(&1).unwrap(), &vec![0u8; 10]);

    // ldi $r2, 0x12 0x23 0x45 0x67 0x78 0x9a 0xbc 0xde 0xf0
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 1);
    assert_eq!(
        vm.registers().get_value(Register::R2 as u8),
        0xf0debc9a78563412
    );
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);
    assert_eq!(vm.memory().retrieve(&1).unwrap(), &vec![0u8; 10]);

    // iseti 1, $r0, $r2
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 1);
    assert_eq!(
        vm.registers().get_value(Register::R2 as u8),
        0xf0debc9a78563412
    );
    assert_eq!(
        vm.memory().retrieve(&0).unwrap(),
        &vec![0x0, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x0]
    );
    assert_eq!(vm.memory().retrieve(&1).unwrap(), &vec![0u8; 10]);

    // ldi $r2, 0
    vm.run_next(&mut handler).unwrap();

    // ldi $r3, 4
    vm.run_next(&mut handler).unwrap();

    // copy r1, r2, r0, r2, r3
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 1);
    assert_eq!(vm.registers().get_value(Register::R2 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R3 as u8), 4);
    assert_eq!(
        vm.memory().retrieve(&0).unwrap(),
        &vec![0x0, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x0]
    );
    assert_eq!(
        vm.memory().retrieve(&1).unwrap(),
        &vec![0x0, 0x12, 0x34, 0x56, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,]
    );
}

#[test]
fn test_copy_2() {
    // malloci $r0, 10
    // malloci $r1, 10
    // ldi $r2,
    // iseti 1, $r0, $r2
    // ldi $r2, 0
    // ldi $r3, 4
    // ldi $r4, 2
    // copy r1, r2, r0, r3, r4

    // r = destination address
    // r1 = destination offset
    // r2 = src address
    // r3 = src offset
    // r4 = number of bytes
    let bytes: Vec<u8> = vec![
        0b0000_1010, // malloci
        0xa,         // 10
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_0000, // r0
        0b0000_1010, // malloci
        0xa,         // 10
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0111_0000, // r1
        0b0000_0011, // ldi
        0x12,
        0x34,
        0x56,
        0x78,
        0x9a,
        0xbc,
        0xde,
        0xf0,
        0b1000_0000, // r2
        0b0001_0000, // iseti
        0b0000_0001, // 1
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_1000, // r0, r2
        0b0000_0011, // ldi
        0x0,         // 0
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b1000_0000, // r2
        0b0000_0011, // ldi
        0x4,         // 4
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b1001_0000, // r3
        0b0000_0011, // ldi
        0x2,         // 2
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b1010_0000, // r4
        0x18,        // copy
        0b0111_1000, // r1, r2
        0b0110_1001, // r0, r3
        0b1010_0000, // r4
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes);
    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    // malloci $r0, 10
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);

    // malloci $r1, 10
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 1);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);
    assert_eq!(vm.memory().retrieve(&1).unwrap(), &vec![0u8; 10]);

    // ldi $r2, 0x12 0x23 0x45 0x67 0x78 0x9a 0xbc 0xde 0xf0
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 1);
    assert_eq!(
        vm.registers().get_value(Register::R2 as u8),
        0xf0debc9a78563412
    );
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);
    assert_eq!(vm.memory().retrieve(&1).unwrap(), &vec![0u8; 10]);

    // iseti 1, $r0, $r2
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 1);
    assert_eq!(
        vm.registers().get_value(Register::R2 as u8),
        0xf0debc9a78563412
    );
    assert_eq!(
        vm.memory().retrieve(&0).unwrap(),
        &vec![0x0, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x0]
    );
    assert_eq!(vm.memory().retrieve(&1).unwrap(), &vec![0u8; 10]);

    // ldi $r2, 0
    vm.run_next(&mut handler).unwrap();

    // ldi $r3, 4
    vm.run_next(&mut handler).unwrap();

    // ldi $r4, 2
    vm.run_next(&mut handler).unwrap();

    // copy r1, r2, r0, r3, r4
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 1);
    assert_eq!(vm.registers().get_value(Register::R2 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R3 as u8), 4);
    assert_eq!(vm.registers().get_value(Register::R4 as u8), 2);
    assert_eq!(
        vm.memory().retrieve(&0).unwrap(),
        &vec![0x0, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x0]
    );
    assert_eq!(
        vm.memory().retrieve(&1).unwrap(),
        &vec![0x78, 0x9a, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,]
    );
}

#[test]
fn test_copyi() {
    // malloci $r0, 10
    // malloci $r1, 10
    // ldi $r2,
    // iseti 1, $r0, $r2
    // copy r1, 0, r0, 0, 4

    // i = destination offset
    // i1 = src offset
    // i2 = number of bytes
    // r = destination address
    // r1 = src address
    let bytes: Vec<u8> = vec![
        0b0000_1010, // malloci
        0xa,         // 10
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_0000, // r0
        0b0000_1010, // malloci
        0xa,         // 10
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0111_0000, // r1
        0b0000_0011, // ldi
        0x12,
        0x34,
        0x56,
        0x78,
        0x9a,
        0xbc,
        0xde,
        0xf0,
        0b1000_0000, // r2
        0b0001_0000, // iseti
        0b0000_0001, // 1
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_1000, // r0, r2
        0x19,        // copyi
        0x0,         //0
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0, //0
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x4, //4
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0111_0110, // r1, r0
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes);
    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    // malloci $r0, 10
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);

    // malloci $r1, 10
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 1);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);
    assert_eq!(vm.memory().retrieve(&1).unwrap(), &vec![0u8; 10]);

    // ldi $r2, 0x12 0x23 0x45 0x67 0x78 0x9a 0xbc 0xde 0xf0
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 1);
    assert_eq!(
        vm.registers().get_value(Register::R2 as u8),
        0xf0debc9a78563412
    );
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);
    assert_eq!(vm.memory().retrieve(&1).unwrap(), &vec![0u8; 10]);

    // iseti 1, $r0, $r2
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 1);
    assert_eq!(
        vm.registers().get_value(Register::R2 as u8),
        0xf0debc9a78563412
    );
    assert_eq!(
        vm.memory().retrieve(&0).unwrap(),
        &vec![0x0, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x0]
    );
    assert_eq!(vm.memory().retrieve(&1).unwrap(), &vec![0u8; 10]);

    // copy r1, r2, r0, r2, r3
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 1);
    assert_eq!(
        vm.memory().retrieve(&0).unwrap(),
        &vec![0x0, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x0]
    );
    assert_eq!(
        vm.memory().retrieve(&1).unwrap(),
        &vec![0x0, 0x12, 0x34, 0x56, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,]
    );
}

#[test]
fn test_copyi_2() {
    // malloci $r0, 10
    // malloci $r1, 10
    // ldi $r2,
    // iseti 1, $r0, $r2
    // copy r1, 3, r0, 4, 2

    // i = destination offset
    // i1 = src offset
    // i2 = number of bytes
    // r = destination address
    // r1 = src address
    let bytes: Vec<u8> = vec![
        0b0000_1010, // malloci
        0xa,         // 10
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_0000, // r0
        0b0000_1010, // malloci
        0xa,         // 10
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0111_0000, // r1
        0b0000_0011, // ldi
        0x12,
        0x34,
        0x56,
        0x78,
        0x9a,
        0xbc,
        0xde,
        0xf0,
        0b1000_0000, // r2
        0b0001_0000, // iseti
        0b0000_0001, // 1
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_1000, // r0, r2
        0x19,        // copyi
        0x3,         //3
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x4, //4
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x2, //2
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0111_0110, // r1, r0
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes);
    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    // malloci $r0, 10
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);

    // malloci $r1, 10
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 1);
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);
    assert_eq!(vm.memory().retrieve(&1).unwrap(), &vec![0u8; 10]);

    // ldi $r2, 0x12 0x23 0x45 0x67 0x78 0x9a 0xbc 0xde 0xf0
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 1);
    assert_eq!(
        vm.registers().get_value(Register::R2 as u8),
        0xf0debc9a78563412
    );
    assert_eq!(vm.memory().retrieve(&0).unwrap(), &vec![0u8; 10]);
    assert_eq!(vm.memory().retrieve(&1).unwrap(), &vec![0u8; 10]);

    // iseti 1, $r0, $r2
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 1);
    assert_eq!(
        vm.registers().get_value(Register::R2 as u8),
        0xf0debc9a78563412
    );
    assert_eq!(
        vm.memory().retrieve(&0).unwrap(),
        &vec![0x0, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x0]
    );
    assert_eq!(vm.memory().retrieve(&1).unwrap(), &vec![0u8; 10]);

    // copy r1, r2, r0, r2, r3
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
    assert_eq!(vm.registers().get_value(Register::R1 as u8), 1);
    assert_eq!(
        vm.memory().retrieve(&0).unwrap(),
        &vec![0x0, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x0]
    );
    assert_eq!(
        vm.memory().retrieve(&1).unwrap(),
        &vec![0x0, 0x0, 0x0, 0x78, 0x9a, 0x0, 0x0, 0x0, 0x0, 0x0,]
    );
}
