use voxl_instruction_set::instruction_arguments::Register;
use vxlvm::validator::{BulkValidator, Validator};
use vxlvm::vm::VM;

use super::handler::System;
use paste::paste;

mod cmp {
    use super::*;

    #[test]
    fn test_cmp_eq() {
        let bytes = vec![
            0x3,  // ldi
            0x22, // 34
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0b0110_0000, // r0
            0x3,         // ldi
            0x22,        // 34
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0b0111_0000, // r1
            0x34,        // cmp
            0b0110_0111, // r0, r1
        ];

        let mut handler = System::new();
        let validator = BulkValidator::with_bytes(bytes);
        let mut vm = VM::new(validator.process_all_instructions().unwrap());

        // ldi $r0, 34
        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), 0x22);

        // ldi $r1, 34
        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), 0x22);
        assert_eq!(vm.registers().get_value(Register::R1 as u8), 0x22);

        // cmp $r0, $r1
        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), 0x22);
        assert_eq!(vm.registers().get_value(Register::R1 as u8), 0x22);
        assert_eq!(vm.registers().get_value(Register::RFL as u8), 1);
    }

    #[test]
    fn test_cmp_lt() {
        let bytes = vec![
            0x3,  // ldi
            0x21, // 33
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0b0110_0000, // r0
            0x3,         // ldi
            0x22,        // 34
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0b0111_0000, // r1
            0x34,        // cmp
            0b0110_0111, // r0, r1
        ];

        let mut handler = System::new();
        let validator = BulkValidator::with_bytes(bytes);
        let mut vm = VM::new(validator.process_all_instructions().unwrap());

        // ldi $r0, 33
        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), 0x21);

        // ldi $r1, 34
        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), 0x21);
        assert_eq!(vm.registers().get_value(Register::R1 as u8), 0x22);

        // cmp $r0, $r1
        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), 0x21);
        assert_eq!(vm.registers().get_value(Register::R1 as u8), 0x22);
        assert_eq!(vm.registers().get_value(Register::RFL as u8), 0b010);
    }

    #[test]
    fn test_cmp_gt() {
        let bytes = vec![
            0x3,  // ldi
            0x21, // 33
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0b0110_0000, // r0
            0x3,         // ldi
            0x22,        // 34
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0b0111_0000, // r1
            0x34,        // cmp
            0b0111_0110, // r1, r0
        ];

        let mut handler = System::new();
        let validator = BulkValidator::with_bytes(bytes);
        let mut vm = VM::new(validator.process_all_instructions().unwrap());

        // ldi $r0, 33
        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), 0x21);

        // ldi $r1, 34
        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), 0x21);
        assert_eq!(vm.registers().get_value(Register::R1 as u8), 0x22);

        // cmp $r0, $r1
        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), 0x21);
        assert_eq!(vm.registers().get_value(Register::R1 as u8), 0x22);
        assert_eq!(vm.registers().get_value(Register::RFL as u8), 0b100);
    }
}

mod cmpi {
    use super::*;

    #[test]
    fn test_cmpi_eq() {
        let mut bytes = vec![
            0x3, // ldi
        ];

        bytes.extend_from_slice(&(-33i64).to_le_bytes());
        bytes.push(0b0110_0000);
        bytes.push(0x3);
        bytes.extend_from_slice(&(-33i64).to_le_bytes());
        bytes.push(0b0111_0000);

        bytes.push(0x35); // cmpi
        bytes.push(0b0110_0111);

        let mut handler = System::new();
        let validator = BulkValidator::with_bytes(bytes);
        let mut vm = VM::new(validator.process_all_instructions().unwrap());

        // ldi $r0, -33
        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), -33i64 as u64);

        // ldi $r1, -33
        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), -33i64 as u64);
        assert_eq!(vm.registers().get_value(Register::R1 as u8), -33i64 as u64);

        // cmp $r0, $r1
        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), -33i64 as u64);
        assert_eq!(vm.registers().get_value(Register::R1 as u8), -33i64 as u64);
        assert_eq!(vm.registers().get_value(Register::RFL as u8), 1);
    }

    #[test]
    fn test_cmpi_eq_2() {
        let mut bytes = vec![
            0x3, // ldi
        ];

        bytes.extend_from_slice(&(33i64).to_le_bytes());
        bytes.push(0b0110_0000);
        bytes.push(0x3);
        bytes.extend_from_slice(&(33i64).to_le_bytes());
        bytes.push(0b0111_0000);

        bytes.push(0x35); // cmpi
        bytes.push(0b0110_0111);

        let mut handler = System::new();
        let validator = BulkValidator::with_bytes(bytes);
        let mut vm = VM::new(validator.process_all_instructions().unwrap());

        // ldi $r0, 33
        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), 33i64 as u64);

        // ldi $r1, 33
        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), 33i64 as u64);
        assert_eq!(vm.registers().get_value(Register::R1 as u8), 33i64 as u64);

        // cmp $r0, $r1
        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), 33i64 as u64);
        assert_eq!(vm.registers().get_value(Register::R1 as u8), 33i64 as u64);
        assert_eq!(vm.registers().get_value(Register::RFL as u8), 1);
    }

    #[test]
    fn test_cmpi_lt() {
        let mut bytes = vec![
            0x3, // ldi
        ];

        bytes.extend_from_slice(&(-33i64).to_le_bytes());
        bytes.push(0b0110_0000);
        bytes.push(0x3);
        bytes.extend_from_slice(&(33i64).to_le_bytes());
        bytes.push(0b0111_0000);

        bytes.push(0x35); // cmpi
        bytes.push(0b0110_0111);

        let mut handler = System::new();
        let validator = BulkValidator::with_bytes(bytes);
        let mut vm = VM::new(validator.process_all_instructions().unwrap());

        // ldi $r0, -33
        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), -33i64 as u64);

        // ldi $r1, 33
        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), -33i64 as u64);
        assert_eq!(vm.registers().get_value(Register::R1 as u8), 33i64 as u64);

        // cmp $r0, $r1
        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), -33i64 as u64);
        assert_eq!(vm.registers().get_value(Register::R1 as u8), 33i64 as u64);
        assert_eq!(vm.registers().get_value(Register::RFL as u8), 0b010);
    }

    #[test]
    fn test_cmpi_lt_2() {
        let mut bytes = vec![
            0x3, // ldi
        ];

        bytes.extend_from_slice(&(-33i64).to_le_bytes());
        bytes.push(0b0110_0000);
        bytes.push(0x3);
        bytes.extend_from_slice(&(-32i64).to_le_bytes());
        bytes.push(0b0111_0000);

        bytes.push(0x35); // cmpi
        bytes.push(0b0110_0111);

        let mut handler = System::new();
        let validator = BulkValidator::with_bytes(bytes);
        let mut vm = VM::new(validator.process_all_instructions().unwrap());

        // ldi $r0, -33
        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), -33i64 as u64);

        // ldi $r1, -32
        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), -33i64 as u64);
        assert_eq!(vm.registers().get_value(Register::R1 as u8), -32i64 as u64);

        // cmp $r0, $r1
        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), -33i64 as u64);
        assert_eq!(vm.registers().get_value(Register::R1 as u8), -32i64 as u64);
        assert_eq!(vm.registers().get_value(Register::RFL as u8), 0b010);
    }

    #[test]
    fn test_cmpi_lt_3() {
        let mut bytes = vec![
            0x3, // ldi
        ];

        bytes.extend_from_slice(&(32i64).to_le_bytes());
        bytes.push(0b0110_0000);
        bytes.push(0x3);
        bytes.extend_from_slice(&(33i64).to_le_bytes());
        bytes.push(0b0111_0000);

        bytes.push(0x35); // cmpi
        bytes.push(0b0110_0111);

        let mut handler = System::new();
        let validator = BulkValidator::with_bytes(bytes);
        let mut vm = VM::new(validator.process_all_instructions().unwrap());

        // ldi $r0, 32
        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), 32i64 as u64);

        // ldi $r1, 33
        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), 32i64 as u64);
        assert_eq!(vm.registers().get_value(Register::R1 as u8), 33i64 as u64);

        // cmp $r0, $r1
        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), 32i64 as u64);
        assert_eq!(vm.registers().get_value(Register::R1 as u8), 33i64 as u64);
        assert_eq!(vm.registers().get_value(Register::RFL as u8), 0b010);
    }

    #[test]
    fn test_cmpi_gt() {
        let mut bytes = vec![
            0x3, // ldi
        ];

        bytes.extend_from_slice(&(-33i64).to_le_bytes());
        bytes.push(0b0110_0000);
        bytes.push(0x3);
        bytes.extend_from_slice(&(33i64).to_le_bytes());
        bytes.push(0b0111_0000);

        bytes.push(0x35); // cmpi
        bytes.push(0b0111_0110);

        let mut handler = System::new();
        let validator = BulkValidator::with_bytes(bytes);
        let mut vm = VM::new(validator.process_all_instructions().unwrap());

        // ldi $r0, -33
        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), -33i64 as u64);

        // ldi $r1, 33
        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), -33i64 as u64);
        assert_eq!(vm.registers().get_value(Register::R1 as u8), 33i64 as u64);

        // cmp $r1, $r0
        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), -33i64 as u64);
        assert_eq!(vm.registers().get_value(Register::R1 as u8), 33i64 as u64);
        assert_eq!(vm.registers().get_value(Register::RFL as u8), 0b100);
    }

    #[test]
    fn test_cmpi_gt_2() {
        let mut bytes = vec![
            0x3, // ldi
        ];

        bytes.extend_from_slice(&(-33i64).to_le_bytes());
        bytes.push(0b0110_0000);
        bytes.push(0x3);
        bytes.extend_from_slice(&(-32i64).to_le_bytes());
        bytes.push(0b0111_0000);

        bytes.push(0x35); // cmpi
        bytes.push(0b0111_0110);

        let mut handler = System::new();
        let validator = BulkValidator::with_bytes(bytes);
        let mut vm = VM::new(validator.process_all_instructions().unwrap());

        // ldi $r0, -33
        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), -33i64 as u64);

        // ldi $r1, -32
        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), -33i64 as u64);
        assert_eq!(vm.registers().get_value(Register::R1 as u8), -32i64 as u64);

        // cmp $r1, $r0
        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), -33i64 as u64);
        assert_eq!(vm.registers().get_value(Register::R1 as u8), -32i64 as u64);
        assert_eq!(vm.registers().get_value(Register::RFL as u8), 0b100);
    }

    #[test]
    fn test_cmpi_gt_3() {
        let mut bytes = vec![
            0x3, // ldi
        ];

        bytes.extend_from_slice(&(32i64).to_le_bytes());
        bytes.push(0b0110_0000);
        bytes.push(0x3);
        bytes.extend_from_slice(&(33i64).to_le_bytes());
        bytes.push(0b0111_0000);

        bytes.push(0x35); // cmpi
        bytes.push(0b0111_0110);

        let mut handler = System::new();
        let validator = BulkValidator::with_bytes(bytes);
        let mut vm = VM::new(validator.process_all_instructions().unwrap());

        // ldi $r0, 32
        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), 32i64 as u64);

        // ldi $r1, 33
        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), 32i64 as u64);
        assert_eq!(vm.registers().get_value(Register::R1 as u8), 33i64 as u64);

        // cmp $r1, $r0
        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), 32i64 as u64);
        assert_eq!(vm.registers().get_value(Register::R1 as u8), 33i64 as u64);
        assert_eq!(vm.registers().get_value(Register::RFL as u8), 0b100);
    }
}

mod cmpf {
    use super::*;

    macro_rules! cmpf_test {
        ($name:tt, $v:expr, $out:expr) => {
            cmpf_test!($name, $v, $v, $out);
        };

        ($name:tt, $a:expr, $b:expr, $out:expr) => {
            paste! {
                #[test]
                fn [<test_ $name>]() {
                    let mut bytes = vec![
                        0x4, // ldf
                    ];

                    let f_a: f64 = $a as f64;
                    let f_b: f64 = $b as f64;

                    let f_a_u: u64 = unsafe { std::mem::transmute(f_a) };
                    let f_b_u: u64 = unsafe { std::mem::transmute(f_b) };

                    bytes.extend_from_slice(&(f_a).to_le_bytes());
                    bytes.push(0b0110_0000);

                    bytes.push(0x4);
                    bytes.extend_from_slice(&(f_b).to_le_bytes());
                    bytes.push(0b0111_0000);

                    bytes.push(0x36); // cmpf
                    bytes.push(0b0110_0111);

                    let mut handler = System::new();
                    let validator = BulkValidator::with_bytes(bytes);
                    let mut vm = VM::new(

                        validator.process_all_instructions().unwrap(),
                    );

                    // ldf $r0, $a
                    vm.run_next(&mut handler).unwrap();

                    assert_eq!(vm.registers().get_value(Register::R0 as u8), f_a_u);

                    // ldf $r1, $b
                    vm.run_next(&mut handler).unwrap();

                    assert_eq!(vm.registers().get_value(Register::R0 as u8), f_a_u);
                    assert_eq!(vm.registers().get_value(Register::R1 as u8), f_b_u);

                    // cmp $r0, $r1
                    vm.run_next(&mut handler).unwrap();

                    assert_eq!(vm.registers().get_value(Register::R0 as u8), f_a_u);
                    assert_eq!(vm.registers().get_value(Register::R1 as u8), f_b_u);
                    assert_eq!(vm.registers().get_value(Register::RFL as u8), $out);
                }
            }
        };
    }

    // Equality
    cmpf_test!(cmpf_eq, -33.33, 0b1);
    cmpf_test!(cmpf_eq_1, 33.33, 0b1);
    cmpf_test!(cmpf_eq_2, -33, 0b1);
    cmpf_test!(cmpf_eq_3, 33, 0b1);

    // Less than
    cmpf_test!(cmpf_lt, -33.33, 34, 0b010);
    cmpf_test!(cmpf_lt_1, 33.33, 34, 0b010);
    cmpf_test!(cmpf_lt_2, -33, 34, 0b010);
    cmpf_test!(cmpf_lt_3, 33, 34, 0b010);
    cmpf_test!(cmpf_lt_4, -33.33, -32, 0b010);
    cmpf_test!(cmpf_lt_5, -33, -32, 0b010);

    // Greater than
    cmpf_test!(cmpf_gt, 34, -33.33, 0b100);
    cmpf_test!(cmpf_gt_1, 34, 33.33, 0b100);
    cmpf_test!(cmpf_gt_2, 34, -33, 0b100);
    cmpf_test!(cmpf_gt_3, 34, 33, 0b100);
    cmpf_test!(cmpf_gt_4, -32, -33.33, 0b100);
    cmpf_test!(cmpf_gt_5, -32, -33, 0b100);
}

mod jump_tests {
    use super::*;

    macro_rules! test_conditional_jump {
        ($a:literal, $b:literal, $output:literal, $instruction:literal, $name:ident) => {
            paste! {
                #[test]
                fn [<test_ $name>]() {
                    let a = ($a as u64).to_le_bytes();
                    let b = ($b as u64).to_le_bytes();
                    let output = $output as u64;
                    let instruction = $instruction as u8;

                    // 0. ldi, $r0, a
                    let mut bytes = vec![0x3]; // ldi
                    bytes.extend_from_slice(&a);
                    bytes.push(0b0110_0000); // $r0

                    // 1. ldi $r1, b
                    bytes.push(0x3);
                    bytes.extend_from_slice(&b);
                    bytes.push(0b0111_0000); // $r1

                    bytes.extend_from_slice(&[
                        0x34, // 2. cmp
                        0b0110_0111, // $r0, $r1
                        instruction, // 3.
                        0x5, // target address
                        0x0,
                        0x0,
                        0x0,
                        0x0,
                        0x0,
                        0x0,
                        0x0,
                        0x3, // 4.ldi
                    ]);

                    bytes.extend_from_slice(&0u64.to_le_bytes());
                    bytes.push(0b1000_0000); // $r2

                    // 5. ldi $r2, b
                    bytes.push(0x3);
                    bytes.extend_from_slice(&1u64.to_le_bytes());
                    bytes.push(0b1000_0000); // $r2

                    let mut handler = System::new();
                    let validator = BulkValidator::with_bytes(bytes);
                    let mut vm = VM::new(

                        validator.process_all_instructions().unwrap(),
                    );


                    vm.run_next(&mut handler).unwrap();
                    vm.run_next(&mut handler).unwrap();
                    vm.run_next(&mut handler).unwrap();
                    vm.run_next(&mut handler).unwrap();
                    vm.run_next(&mut handler).unwrap();

                    assert_eq!(vm.registers().get_value(Register::R2 as u8), output);
                }
            }
        };
    }

    #[test]
    fn test_jmp() {
        let bytes = vec![
            0x37, // jmp
            0x2,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x44, // halt
            0x3,  //ldi
            0x56,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0b0110_0000, // $r0
        ];

        let mut handler = System::new();
        let validator = BulkValidator::with_bytes(bytes);
        let mut vm = VM::new(validator.process_all_instructions().unwrap());

        // jmp 2
        vm.run_next(&mut handler).unwrap();

        // ldi $r0, 0x56

        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), 0x56);
    }

    #[test]
    fn test_jmp_2() {
        let bytes = vec![
            0x37, // jmp
            0x12,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x44, // halt
            0x0,  //nop
            0x0,  //nop
            0x0,  //nop
            0x0,  //nop
            0x0,  //nop
            0x0,  //nop
            0x0,  //nop
            0x0,  //nop
            0x0,  //nop
            0x0,  //nop
            0x0,  //nop
            0x0,  //nop
            0x0,  //nop
            0x0,  //nop
            0x0,  //nop
            0x0,  //nop
            0x3,  //ldi
            0x56,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0b0110_0000, // $r0
        ];

        let mut handler = System::new();
        let validator = BulkValidator::with_bytes(bytes);
        let mut vm = VM::new(validator.process_all_instructions().unwrap());

        // jmp 18
        vm.run_next(&mut handler).unwrap();

        // ldi $r0, 0x56
        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), 0x56);
    }

    #[test]
    fn test_jmp_3() {
        let bytes = vec![
            0x37, // jmp
            0x13,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x44, // halt - 2
            0x0,  // nop
            0x0,  // nop
            0x0,  // nop
            0x0,  // nop
            0x0,  // nop
            0x0,  // nop
            0x0,  // nop
            0x0,  // nop
            0x3,  // ldi - 10
            0x56,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0b0110_0000, // $r0
            0x0,         // nop - 11
            0x0,         // nop
            0x0,         // nop
            0x0,         // nop
            0x0,         // nop
            0x0,         // nop
            0x0,         // nop
            0x0,         // nop
            0x37,        // jmp - 19
            0xa,
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

        // jmp 19
        vm.run_next(&mut handler).unwrap();
        // jmp 10
        vm.run_next(&mut handler).unwrap();

        // ldi $r0, 0x56
        vm.run_next(&mut handler).unwrap();

        assert_eq!(vm.registers().get_value(Register::R0 as u8), 0x56);
    }

    test_conditional_jump!(52, 52, 1, 0x38, jump_equal_1);
    test_conditional_jump!(53, 52, 0, 0x38, jump_equal_2);

    test_conditional_jump!(52, 52, 0, 0x39, jump_not_equal_1);
    test_conditional_jump!(53, 52, 1, 0x39, jump_not_equal_2);

    test_conditional_jump!(52, 53, 0, 0x3a, jump_greater_than_equal_1);
    test_conditional_jump!(53, 52, 1, 0x3a, jump_greater_than_equal_2);
    test_conditional_jump!(52, 52, 1, 0x3a, jump_greater_than_equal_3);

    test_conditional_jump!(52, 53, 0, 0x3b, jump_greater_than_1);
    test_conditional_jump!(53, 52, 1, 0x3b, jump_greater_than_2);
    test_conditional_jump!(52, 52, 0, 0x3b, jump_greater_than_3);

    test_conditional_jump!(52, 53, 1, 0x3c, jump_less_than_equal_1);
    test_conditional_jump!(53, 52, 0, 0x3c, jump_less_than_equal_2);
    test_conditional_jump!(52, 52, 1, 0x3c, jump_less_than_equal_3);

    test_conditional_jump!(52, 53, 1, 0x3d, jump_less_than_1);
    test_conditional_jump!(53, 52, 0, 0x3d, jump_less_than_2);
    test_conditional_jump!(52, 52, 0, 0x3d, jump_less_than_3);
}

mod call_return {
    use super::*;

    #[test]
    fn test_call() {
        let bytes = vec![
            0x43, // call
            0x2,  // 2
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,  // nop
            0x3,  // ldi
            0x10, // 16
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0b0110_1000, // r0
        ];

        let mut handler = System::new();
        let validator = BulkValidator::with_bytes(bytes);
        let mut vm = VM::new(validator.process_all_instructions().unwrap());

        // call
        vm.run_next(&mut handler).unwrap();
        assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
        assert_eq!(vm.registers().get_value(Register::RSP as u8), 3 * 8);
        assert_eq!(vm.registers().get_value(Register::RFP as u8), 3 * 8);

        // ldi
        vm.run_next(&mut handler).unwrap();
        assert_eq!(vm.registers().get_value(Register::R0 as u8), 0x10);
        assert_eq!(vm.registers().get_value(Register::RSP as u8), 3 * 8);
        assert_eq!(vm.registers().get_value(Register::RFP as u8), 3 * 8);
    }

    #[test]
    fn test_call_2() {
        let bytes = vec![
            0x43, // call
            0x1,  // 1
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x43, // call
            0x2,  // 2
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x3,  // ldi
            0x10, // 16
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0b0110_1000, // r0
        ];

        let mut handler = System::new();
        let validator = BulkValidator::with_bytes(bytes);
        let mut vm = VM::new(validator.process_all_instructions().unwrap());

        // call
        vm.run_next(&mut handler).unwrap();
        assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
        assert_eq!(vm.registers().get_value(Register::RSP as u8), 3 * 8);
        assert_eq!(vm.registers().get_value(Register::RFP as u8), 3 * 8);

        // call
        vm.run_next(&mut handler).unwrap();
        assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
        assert_eq!(vm.registers().get_value(Register::RSP as u8), 6 * 8);
        assert_eq!(vm.registers().get_value(Register::RFP as u8), 6 * 8);

        // ldi
        vm.run_next(&mut handler).unwrap();
        assert_eq!(vm.registers().get_value(Register::R0 as u8), 0x10);
        assert_eq!(vm.registers().get_value(Register::RSP as u8), 6 * 8);
        assert_eq!(vm.registers().get_value(Register::RFP as u8), 6 * 8);
    }

    #[test]
    fn test_ret() {
        let bytes = vec![
            0x43, // call
            0x3,  // 3
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x3,  // ldi
            0x20, // 32
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0b0110_1000, // r0
            0x45,        // halt
            0x3,         // ldi
            0x10,        // 16
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0b0110_1000, // r0
            0x44,        //ret
        ];

        let mut handler = System::new();
        let validator = BulkValidator::with_bytes(bytes);
        let instructions = validator.process_all_instructions().unwrap();
        let mut vm = VM::new(instructions);

        // call
        vm.run_next(&mut handler).unwrap();
        assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
        assert_eq!(vm.registers().get_value(Register::RSP as u8), 3 * 8);
        assert_eq!(vm.registers().get_value(Register::RFP as u8), 3 * 8);

        // ldi
        vm.run_next(&mut handler).unwrap();
        assert_eq!(vm.registers().get_value(Register::R0 as u8), 0x10);
        assert_eq!(vm.registers().get_value(Register::RSP as u8), 3 * 8);
        assert_eq!(vm.registers().get_value(Register::RFP as u8), 3 * 8);

        // ret
        vm.run_next(&mut handler).unwrap();
        assert_eq!(vm.registers().get_value(Register::R0 as u8), 0x10);
        assert_eq!(vm.registers().get_value(Register::RSP as u8), 0);
        assert_eq!(vm.registers().get_value(Register::RFP as u8), 0);

        // ldi
        vm.run_next(&mut handler).unwrap();
        assert_eq!(vm.registers().get_value(Register::R0 as u8), 0x20);
        assert_eq!(vm.registers().get_value(Register::RSP as u8), 0);
        assert_eq!(vm.registers().get_value(Register::RFP as u8), 0);

        // halt
        vm.run_next(&mut handler).unwrap();
        assert_eq!(vm.registers().get_value(Register::R0 as u8), 0x20);
        assert_eq!(vm.registers().get_value(Register::RSP as u8), 0);
        assert_eq!(vm.registers().get_value(Register::RFP as u8), 0);
    }

    #[test]
    fn test_ret_2() {
        let bytes = vec![
            0x43, // 0. call
            0x3,  // 3
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x3,  // 1. ldi
            0x20, // 32
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0b0110_1000, // r0
            0x44,        // 2. ret
            0x3,         // 3. ldi
            0x10,        // 16
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0b0110_1000, // r0
            0x43,        // 4. call
            0x1,         // 1
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x44, // 5. ret
        ];

        let mut handler = System::new();
        let validator = BulkValidator::with_bytes(bytes);
        let instructions = validator.process_all_instructions().unwrap();
        let mut vm = VM::new(instructions);

        // call
        vm.run_next(&mut handler).unwrap();
        assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
        assert_eq!(vm.registers().get_value(Register::RSP as u8), 3 * 8);
        assert_eq!(vm.registers().get_value(Register::RFP as u8), 3 * 8);

        // ldi
        vm.run_next(&mut handler).unwrap();
        assert_eq!(vm.registers().get_value(Register::R0 as u8), 0x10);
        assert_eq!(vm.registers().get_value(Register::RSP as u8), 3 * 8);
        assert_eq!(vm.registers().get_value(Register::RFP as u8), 3 * 8);

        // call
        vm.run_next(&mut handler).unwrap();
        assert_eq!(vm.registers().get_value(Register::R0 as u8), 0x10);
        assert_eq!(vm.registers().get_value(Register::RSP as u8), 6 * 8);
        assert_eq!(vm.registers().get_value(Register::RFP as u8), 6 * 8);

        // ldi
        vm.run_next(&mut handler).unwrap();
        assert_eq!(vm.registers().get_value(Register::R0 as u8), 0x20);
        assert_eq!(vm.registers().get_value(Register::RSP as u8), 6 * 8);
        assert_eq!(vm.registers().get_value(Register::RFP as u8), 6 * 8);

        // ret
        vm.run_next(&mut handler).unwrap();
        assert_eq!(vm.registers().get_value(Register::R0 as u8), 0x20);
        assert_eq!(vm.registers().get_value(Register::RSP as u8), 3 * 8);
        assert_eq!(vm.registers().get_value(Register::RFP as u8), 3 * 8);

        // ret
        vm.run_next(&mut handler).unwrap();
        assert_eq!(vm.registers().get_value(Register::R0 as u8), 0x20);
        assert_eq!(vm.registers().get_value(Register::RSP as u8), 0);
        assert_eq!(vm.registers().get_value(Register::RFP as u8), 0);
    }
}
