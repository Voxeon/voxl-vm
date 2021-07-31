use super::handler::System;
use paste::paste;
use vxl_iset::instruction_arguments::Register;
use vxlvm::validator::{BulkValidator, Validator};
use vxlvm::vm::VM;

macro_rules! binary_operation_test {
    ($lhs:expr, $rhs:expr, $ins:expr, $name:ident, $op:tt) => {
        paste! {
            #[test]
            fn [<test_$name>]() {
                let mut bytes = vec![
                    0x3 // ldi
                ];
                bytes.append(&mut $lhs.to_le_bytes().to_vec());
                bytes.push(0b0110_0000); // $r0

                bytes.push(0x3);//ldi
                bytes.append(&mut $rhs.to_le_bytes().to_vec());
                bytes.push(0b0111_0000); // $r1

                bytes.push($ins);

                bytes.push(0b0110_0110); // $r0, $r0,
                bytes.push(0b0111_0000); // $r1

                let mut handler = System::new();
                let validator = BulkValidator::with_bytes(bytes);
                let mut vm = VM::new(
                    validator.process_all_instructions().unwrap(),
                );

                // ldi $r0, lhs
                vm.run_next(&mut handler).unwrap();

                // ldi $r1, rhs
                vm.run_next(&mut handler).unwrap();

                assert_eq!(vm.registers().get_value(Register::R0 as u8), $lhs as u64);
                assert_eq!(vm.registers().get_value(Register::R1 as u8), $rhs as u64);

                // $ins $r0, $r0, $r1
                vm.run_next(&mut handler).unwrap();
                assert_eq!(vm.registers().get_value(Register::R0 as u8), ($lhs $op $rhs) as u64);
                assert_eq!(vm.registers().get_value(Register::R1 as u8), $rhs as u64);
            }
        }
    };

    ($lhs:expr, $rhs:expr, $out:expr, $ins:expr, $name:ident) => {
        paste! {
            #[test]
            fn [<test_$name>]() {
                let mut bytes = vec![
                    0x3 // ldi
                ];
                bytes.append(&mut $lhs.to_le_bytes().to_vec());
                bytes.push(0b0110_0000); // $r0

                bytes.push(0x3);//ldi
                bytes.append(&mut $rhs.to_le_bytes().to_vec());
                bytes.push(0b0111_0000); // $r1

                bytes.push($ins);

                bytes.push(0b0110_0110); // $r0, $r0,
                bytes.push(0b0111_0000); // $r1

                let mut handler = System::new();
                let validator = BulkValidator::with_bytes(bytes);
                let mut vm = VM::new(
                    validator.process_all_instructions().unwrap(),
                );

                // ldi $r0, lhs
                vm.run_next(&mut handler).unwrap();

                // ldi $r1, rhs
                vm.run_next(&mut handler).unwrap();

                assert_eq!(vm.registers().get_value(Register::R0 as u8), $lhs as u64);
                assert_eq!(vm.registers().get_value(Register::R1 as u8), $rhs as u64);

                // $ins $r0, $r0, $r1
                vm.run_next(&mut handler).unwrap();
                assert_eq!(vm.registers().get_value(Register::R0 as u8), $out as u64);
                assert_eq!(vm.registers().get_value(Register::R1 as u8), $rhs as u64);
            }
        }
    };
}

mod signed_binary_tests {
    use super::*;

    // Addition
    binary_operation_test!(55i64, 5i64, 0x1a, addi, +);
    binary_operation_test!(55i64, -5i64, 0x1a, addi_2, +);
    binary_operation_test!(-55i64, -5i64, 0x1a, addi_3, +);

    // Subtraction
    binary_operation_test!(55i64, 5i64, 0x1b, subi, -);
    binary_operation_test!(55i64, -5i64, 0x1b, subi_2, -);
    binary_operation_test!(-55i64, -5i64, 0x1b, subi_3, -);

    // Multiplication
    binary_operation_test!(50i64, 5i64, 0x1c, muli, *);
    binary_operation_test!(50i64, -5i64, 0x1c, muli_2, *);
    binary_operation_test!(-50i64, -5i64, 0x1c, muli_3, *);

    // Division
    binary_operation_test!(50i64, 5i64, 0x1d, divi, /);

    //Mod
    binary_operation_test!(50i64, 5i64, 0x1e, modi, %);
    binary_operation_test!(50i64, 6i64, 0x1e, modi_1, %);
    binary_operation_test!(50i64, 7i64, 0x1e, modi_2, %);
}

mod unsigned_binary_tests {
    use super::*;

    // Addition
    binary_operation_test!(0xFFFFFFFFFFFFFFF0u64, 0xFu64, 0x1f, addu, +);
    binary_operation_test!(55u64, 5u64, 0x1f, addu_1, +);

    // Subtraction
    binary_operation_test!(0xFFFFFFFFFFFFFFFFu64, 0xFu64, 0x20, subu, -);
    binary_operation_test!(1u64, 1u64, 0x20, subu_1, -);

    // Multiplication
    binary_operation_test!(1u64, 5u64, 0x21, mulu, *);
    binary_operation_test!(50u64, 5u64, 0x21, mulu_1, *);
    binary_operation_test!(0x1111111111111111u64, 0xFu64, 0x21, mulu_2, *);

    // Division
    binary_operation_test!(5u64, 1u64, 0x22, divu, /);
    binary_operation_test!(0xFFFFFFFFFFFFFFFFu64, 0xFu64, 0x22, divu_1, /);

    //Mod
    binary_operation_test!(50u64, 5u64, 0x23, modu, %);
    binary_operation_test!(50u64, 6u64, 0x23, modu_1, %);
    binary_operation_test!(50u64, 7u64, 0x23, modu_2, %);
    binary_operation_test!(0xFFFFFFFFFFFFFFFFu64, 0xFu64, 0x23, modu_3, %);
    binary_operation_test!(0x1111111111111111u64, 0xFu64, 0x23, modu_4, %);
}

mod floating_binary_tests {
    use super::*;

    macro_rules! float_binary_operation_test {
            ($lhs:expr, $rhs:expr, $out:expr, $ins:expr, $name:ident, $op:tt) => {
                paste! {
                    #[test]
                    fn [<test_$name>]() {
                        let mut bytes = vec![
                            0x4 // ldf
                        ];
                        bytes.append(&mut $lhs.to_le_bytes().to_vec());
                        bytes.push(0b0110_0000); // $r0

                        bytes.push(0x4);//ldf
                        bytes.append(&mut $rhs.to_le_bytes().to_vec());
                        bytes.push(0b0111_0000); // $r1

                        bytes.push($ins);

                        bytes.push(0b0110_0110); // $r0, $r0,
                        bytes.push(0b0111_0000); // $r1

                        let mut handler = System::new();
                        let validator = BulkValidator::with_bytes(bytes);
                        let mut vm = VM::new(

                            validator.process_all_instructions().unwrap(),
                        );

                        // ldf $r0, lhs
                        vm.run_next(&mut handler).unwrap();

                        // ldf $r1, rhs
                        vm.run_next(&mut handler).unwrap();

                        assert_eq!(vm.registers().get_value(Register::R0 as u8), unsafe { std::mem::transmute($lhs) });
                        assert_eq!(vm.registers().get_value(Register::R1 as u8), unsafe { std::mem::transmute($rhs) });

                        // $ins $r0, $r0, $r1
                        vm.run_next(&mut handler).unwrap();
                        assert_eq!(vm.registers().get_value(Register::R0 as u8), unsafe { std::mem::transmute(($lhs $op $rhs)) });
                        assert_eq!(vm.registers().get_value(Register::R1 as u8), unsafe { std::mem::transmute($rhs) });
                    }
                }
            };
        }

    // Addition tests
    float_binary_operation_test!(12.3f64, 14.4f64, 26.7f64, 0x24, addf, +);
    float_binary_operation_test!(-12.3f64, 14.4f64, 2.1f64, 0x24, addf_1, +);
    float_binary_operation_test!(-12.3f64, -14.4f64, -26.7f64, 0x24, addf_2, +);

    // Subtraction tests
    float_binary_operation_test!(12.3f64, 14.4f64, -2.1f64, 0x25, subf, -);
    float_binary_operation_test!(-12.3f64, 14.4f64, -26.7f64, 0x25, subf_1, -);
    float_binary_operation_test!(-12.3f64, -14.4f64, 2.1f64, 0x25, subf_2, -);

    // Multiplication tests
    float_binary_operation_test!(12.3f64, 2.2f64, 27.06f64, 0x26, mulf, *);
    float_binary_operation_test!(-12.3f64, 2.2f64, -27.06f64, 0x26, mulf_1, *);
    float_binary_operation_test!(12.3f64, -2.2f64, -27.06f64, 0x26, mulf_2, *);
    float_binary_operation_test!(-12.3f64, -2.2f64, 27.06f64, 0x26, mulf_3, *);

    // Division tests
    float_binary_operation_test!(12.4f64, 3.2f64, 3.875f64, 0x27, divf, /);
    float_binary_operation_test!(-12.4f64, 3.2f64, -3.875f64, 0x27, divf_1, /);
    float_binary_operation_test!(12.4f64, -3.2f64, -3.875f64, 0x27, divf_2, /);
    float_binary_operation_test!(-12.4f64, -3.2f64, 3.875f64, 0x27, divf_3, /);
}

mod bitwise_operation_tests {
    use super::*;

    macro_rules! bitwise_operation_test {
        ($lhs:expr, $rhs:expr, $out:expr, $ins:expr, $name:ident) => {
            paste! {
                #[test]
                fn [<test_$name>]() {
                    let mut bytes = vec![
                        0x3 // ldi
                    ];
                    bytes.append(&mut $lhs.to_le_bytes().to_vec());
                    bytes.push(0b0110_0000); // $r0

                    bytes.push(0x3);//ldi
                    bytes.append(&mut $rhs.to_le_bytes().to_vec());
                    bytes.push(0b0111_0000); // $r1

                    bytes.push($ins);

                    bytes.push(0b0110_0111); // $r0, $r1,

                    let mut handler = System::new();
                    let validator = BulkValidator::with_bytes(bytes);
                    let mut vm = VM::new(
                        validator.process_all_instructions().unwrap(),
                    );

                    // ldi $r0, lhs
                    vm.run_next(&mut handler).unwrap();

                    // ldi $r1, rhs
                    vm.run_next(&mut handler).unwrap();

                    assert_eq!(vm.registers().get_value(Register::R0 as u8), $lhs as u64);
                    assert_eq!(vm.registers().get_value(Register::R1 as u8), $rhs as u64);

                    // $ins $r0, $r1
                    vm.run_next(&mut handler).unwrap();
                    assert_eq!(vm.registers().get_value(Register::R0 as u8), $out as u64);
                    assert_eq!(vm.registers().get_value(Register::R1 as u8), $rhs as u64);
                }
            }
        };

        (imm $lhs:expr, $rhs:expr, $out:expr, $ins:expr, $name:ident) => {
            paste! {
                #[test]
                fn [<test_$name>]() {
                    let mut bytes = vec![
                        0x3 // ldi
                    ];
                    bytes.append(&mut $lhs.to_le_bytes().to_vec());
                    bytes.push(0b0110_0000); // $r0

                    bytes.push($ins);
                    bytes.append(&mut $rhs.to_le_bytes().to_vec());

                    bytes.push(0b0110_0000); // $r0

                    let mut handler = System::new();
                    let validator = BulkValidator::with_bytes(bytes);
                    let mut vm = VM::new(
                        validator.process_all_instructions().unwrap(),
                    );

                    // ldi $r0, lhs
                    vm.run_next(&mut handler).unwrap();

                    assert_eq!(vm.registers().get_value(Register::R0 as u8), $lhs as u64);

                    // $ins $rhs, $r0
                    vm.run_next(&mut handler).unwrap();
                    assert_eq!(vm.registers().get_value(Register::R0 as u8), $out as u64);
                }
            }
        };
    }

    bitwise_operation_test!(0b1101u64, 1u64, 0b11010, 0x28, rotl);
    bitwise_operation_test!(
        0b01110000_00000000_00000000_00000000_00000000_00000000_00000000_00000000u64,
        3u64,
        0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000011u64,
        0x28,
        rotl_1
    );

    bitwise_operation_test!(imm 0b1101u64, 1u64, 0b11010, 0x29, rotli);

    bitwise_operation_test!(imm
        0b01110000_00000000_00000000_00000000_00000000_00000000_00000000_00000000u64,
        3u64,
        0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000011u64,
        0x29,
        rotli_1
    );

    bitwise_operation_test!(0b11010u64, 1u64, 0b1101u64, 0x2a, rotr);

    bitwise_operation_test!(
        0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000011u64,
        3u64,
        0b01110000_00000000_00000000_00000000_00000000_00000000_00000000_00000000u64,
        0x2a,
        rotr_1
    );

    bitwise_operation_test!(imm 0b11010u64, 1u64, 0b1101u64, 0x2b, rotri);

    bitwise_operation_test!(imm
        0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000011u64,
        3u64,
        0b01110000_00000000_00000000_00000000_00000000_00000000_00000000_00000000u64,
        0x2b,
        rotri_1
    );

    bitwise_operation_test!(0b1101u64, 1u64, 0b11010, 0x2c, sll);
    bitwise_operation_test!(
        0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000011u64,
        2u64,
        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001100u64,
        0x2c,
        sll_1
    );

    bitwise_operation_test!(imm 0b1101u64, 1u64, 0b11010, 0x2d, slli);

    bitwise_operation_test!(imm
        0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000011u64,
        2u64,
        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001100u64,
        0x2d,
        slli_1
    );

    bitwise_operation_test!(imm
        0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000011u64,
        63u64,
        0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000u64,
        0x2d,
        slli_2
    );

    bitwise_operation_test!(0b1101u64, 1u64, 0b110, 0x2e, srl);

    bitwise_operation_test!(
        0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000011u64,
        2u64,
        0b00100000_00000000_00000000_00000000_00000000_00000000_00000000_00000000u64,
        0x2e,
        srl_1
    );

    bitwise_operation_test!(imm 0b1101u64, 1u64, 0b110, 0x2f, srli);

    bitwise_operation_test!(imm
        0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000011u64,
        2u64,
        0b00100000_00000000_00000000_00000000_00000000_00000000_00000000_00000000u64,
        0x2f,
        srli_1
    );

    #[test]
    fn test_not() {
        let mut bytes = vec![
            0x3, // ldi
        ];

        bytes.append(&mut 0b1100110011u64.to_le_bytes().to_vec());
        bytes.push(0b0110_0000); // $r0

        bytes.push(0x30);
        bytes.push(0b0110_0000); // $r0

        let mut handler = System::new();
        let validator = BulkValidator::with_bytes(bytes);
        let mut vm = VM::new(validator.process_all_instructions().unwrap());

        // ldi $r0, lhs
        vm.run_next(&mut handler).unwrap();

        assert_eq!(
            vm.registers().get_value(Register::R0 as u8),
            0b1100110011u64
        );

        // not $r0
        vm.run_next(&mut handler).unwrap();
        assert_eq!(
            vm.registers().get_value(Register::R0 as u8),
            !0b1100110011u64,
        );
    }

    // Test And

    binary_operation_test!(0u64, 0u64, 0u64, 0x31, and);
    binary_operation_test!(0b1101u64, 0u64, 0u64, 0x31, and_1);
    binary_operation_test!(0b1101u64, 0b11u64, 0b1u64, 0x31, and_2);
    binary_operation_test!(0b1101u64, 0b1101u64, 0b1101u64, 0x31, and_3);

    // Test Or
    binary_operation_test!(0u64, 0u64, 0u64, 0x32, or);
    binary_operation_test!(0b1101u64, 0u64, 0b1101u64, 0x32, or_1);
    binary_operation_test!(0b1101u64, 0b11u64, 0b1111u64, 0x32, or_2);
    binary_operation_test!(0b1101u64, 0b1101u64, 0b1101u64, 0x32, or_3);

    // Test Xor
    binary_operation_test!(0u64, 0u64, 0u64, 0x33, xor);
    binary_operation_test!(0b1101u64, 0u64, 0b1101u64, 0x33, xor_1);
    binary_operation_test!(0b1101u64, 0b1111u64, 0b10u64, 0x33, xor_2);
    binary_operation_test!(0b1101u64, 0b1101u64, 0u64, 0x33, xor_3);
    binary_operation_test!(0b0101u64, 0b1010u64, 0b1111u64, 0x33, xor_4);
}

#[test]
fn test_i2f() {
    let bytes = vec![
        0x3,  // ldi
        0x45, // 0x45
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_0000, // $r0
        0x3e,        // i2f
        0b0110_0000, // $r0
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes);
    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    // ldi $r0, 0x45
    vm.run_next(&mut handler).unwrap();

    //i2f $r0
    vm.run_next(&mut handler).unwrap();

    let cmp = unsafe { core::mem::transmute(69.0f64) };

    assert_eq!(vm.registers().get_value(Register::R0 as u8), cmp);
}

#[test]
fn test_i2f_2() {
    let bytes = vec![
        0x3, // ldi
        0x0, // 0x0
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0b0110_0000, // $r0
        0x3e,        // i2f
        0b0110_0000, // $r0
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes);
    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    // ldi $r0, 0x45
    vm.run_next(&mut handler).unwrap();

    //i2f $r0
    vm.run_next(&mut handler).unwrap();

    let cmp = unsafe { core::mem::transmute(0f64) };

    assert_eq!(vm.registers().get_value(Register::R0 as u8), cmp);
}

#[test]
fn test_f2i_1() {
    let bytes = vec![
        0x3,  // ldi
        0x9a, // 0.2
        0x99,
        0x99,
        0x99,
        0x99,
        0x99,
        0xc9,
        0x3f,
        0b0110_0000, // $r0
        0x3f,        // f2i
        0b0110_0000, // $r0
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes);
    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    // ldi $r0, 0x45
    vm.run_next(&mut handler).unwrap();

    //i2f $r0
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
}

#[test]
fn test_f2i_2() {
    let bytes = vec![
        0x3,  // ldi
        0x9a, // -0.2
        0x99,
        0x99,
        0x99,
        0x99,
        0x99,
        0xc9,
        0xbf,
        0b0110_0000, // $r0
        0x3f,        // f2i
        0b0110_0000, // $r0
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes);
    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    // ldi $r0, 0x45
    vm.run_next(&mut handler).unwrap();

    //i2f $r0
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), 0);
}

#[test]
fn test_f2i_3() {
    let bytes = vec![
        0x3,  // ldi
        0x9a, // -2.2
        0x99,
        0x99,
        0x99,
        0x99,
        0x99,
        0x01,
        0xc0,
        0b0110_0000, // $r0
        0x3f,        // f2i
        0b0110_0000, // $r0
    ];

    let mut handler = System::new();
    let validator = BulkValidator::with_bytes(bytes);
    let mut vm = VM::new(validator.process_all_instructions().unwrap());

    // ldi $r0, 0x45
    vm.run_next(&mut handler).unwrap();

    //i2f $r0
    vm.run_next(&mut handler).unwrap();

    assert_eq!(vm.registers().get_value(Register::R0 as u8), -2i64 as u64);
}
