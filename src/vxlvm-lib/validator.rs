use core::usize;

use alloc::vec::Vec;
use vxl_iset::instruction::Instruction;
use vxl_iset::instruction_arguments::{Address, Immediate, InstructionArgument, Register};

use crate::error::ValidatorError;

pub trait Validator {
    fn append_bytes(&mut self, bytes: Vec<u8>);
    fn append_byte(&mut self, byte: u8);

    fn next_byte(&mut self) -> Option<u8>;
    fn has_next_byte(&self) -> bool;

    fn process_all_instructions(mut self) -> Result<Vec<Instruction>, ValidatorError>
    where
        Self: Sized,
    {
        let mut instructions = Vec::new();

        while self.has_next_byte() {
            instructions.push(self.take_next_instruction()?);
        }

        return Ok(instructions);
    }

    fn take_next_instruction(&mut self) -> Result<Instruction, ValidatorError> {
        let opcode = self
            .next_byte()
            .ok_or(ValidatorError::UnexpectedEndOfBytes)?;

        let register_count = Instruction::register_count(opcode)
            .ok_or(ValidatorError::UnknownRegisterCountForOpcode)?;
        let address_count = Instruction::address_count(opcode)
            .ok_or(ValidatorError::UnknownAddressCountForOpcode)?;
        let immediate_count = Instruction::immediate_count(opcode)
            .ok_or(ValidatorError::UnknownImmediateCountForOpcode)?;

        let mut registers = Vec::new();
        let mut addresses = Vec::new();
        let mut immediates = Vec::new();

        // immediates | addresses | registers

        for _ in 0..immediate_count {
            let mut bytes = [0u8; Immediate::BYTES];

            for b in 0..Immediate::BYTES {
                bytes[b] = self
                    .next_byte()
                    .ok_or(ValidatorError::UnexpectedEndOfBytes)?;
            }

            immediates.push(Immediate::from(bytes));
        }

        for _ in 0..address_count {
            let mut bytes = [0u8; Address::BYTES];

            for b in 0..Immediate::BYTES {
                bytes[b] = self
                    .next_byte()
                    .ok_or(ValidatorError::UnexpectedEndOfBytes)?;
            }

            addresses.push(Address::from(bytes));
        }

        // Registers are always read from upper 4 bits to lower 4 bits.
        // i.e for the byte 0001 1111 register 1 would be 0001 and register 2 would be 1111

        let mut current_byte = None;

        for _ in 0..register_count {
            if current_byte.is_none() {
                current_byte = Some(
                    self.next_byte()
                        .ok_or(ValidatorError::UnexpectedEndOfBytes)?,
                );

                registers.push(Register::from_bits(current_byte.unwrap() >> 4));
            } else {
                registers.push(Register::from_bits(current_byte.unwrap() & 0xF));

                current_byte = None;
            }
        }

        return Instruction::new(opcode, registers, addresses, immediates)
            .ok_or(ValidatorError::InvalidInstructionFormat);
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BulkValidator {
    current_location: usize,
    bytes: Vec<u8>,
}

impl BulkValidator {
    pub fn new() -> Self {
        return Self {
            current_location: 0,
            bytes: Vec::new(),
        };
    }

    pub fn with_bytes(bytes: Vec<u8>) -> Self {
        return Self {
            current_location: 0,
            bytes,
        };
    }
}

impl Validator for BulkValidator {
    fn append_bytes(&mut self, mut bytes: Vec<u8>) {
        if self.bytes.is_empty() {
            self.bytes = bytes;
            return;
        }

        self.bytes.append(&mut bytes);
    }

    fn append_byte(&mut self, byte: u8) {
        self.bytes.push(byte);
    }

    fn next_byte(&mut self) -> Option<u8> {
        if self.current_location >= self.bytes.len() {
            return None;
        } else {
            self.current_location += 1;
            return Some(self.bytes[self.current_location - 1]);
        }
    }

    fn has_next_byte(&self) -> bool {
        return self.current_location < self.bytes.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod instruction_tests {
        use super::*;

        // These don't test every instruction just some ones with distinguishable properties.

        #[test]
        fn test_noop() {
            let bytes: [u8; 1] = [0b0000_0000];

            let mut validator = BulkValidator::with_bytes(bytes.to_vec());

            assert_eq!(validator.take_next_instruction(), Ok(Instruction::Nop));
        }

        #[test]
        fn test_ldi() {
            // ldi 63, $r0
            let bytes: [u8; 10] = [
                0b0000_0011, // ldi
                0b0011_1111, // 63
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0110_0000, // r0
            ];

            let mut validator = BulkValidator::with_bytes(bytes.to_vec());

            assert_eq!(
                validator.take_next_instruction(),
                Ok(Instruction::Ldi(Immediate::from(63i64), Register::R0))
            );
        }

        #[test]
        fn test_ldf() {
            // ldf 63.24, $r0
            let bytes: [u8; 10] = [
                0b0000_0011, // ldi
                0b0001_1111,
                0b1000_0101,
                0b1110_1011,
                0b0101_0001,
                0b1011_1000,
                0b1001_1110,
                0b0100_1111,
                0b0100_0000,
                0b0110_0000, // r0
            ];

            let mut validator = BulkValidator::with_bytes(bytes.to_vec());

            assert_eq!(
                validator.take_next_instruction(),
                Ok(Instruction::Ldi(Immediate::from(63.24f64), Register::R0))
            );
        }

        #[test]
        fn test_ldb() {
            // ldb 63, $r0
            let bytes: [u8; 10] = [
                0b0000_0010, // ldi
                0b0011_1111, // 63
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0110_0000, // r0
            ];

            let mut validator = BulkValidator::with_bytes(bytes.to_vec());

            assert_eq!(
                validator.take_next_instruction(),
                Ok(Instruction::Ldb(Immediate::from(63u8), Register::R0))
            );
        }

        #[test]
        fn test_mov() {
            // mov $r0, $rou (move rou -> r0)
            let bytes: [u8; 2] = [
                0b0000_0101, // mov
                0b0110_0010, // r0, rou
            ];

            let mut validator = BulkValidator::with_bytes(bytes.to_vec());

            assert_eq!(
                validator.take_next_instruction(),
                Ok(Instruction::Mov(Register::R0, Register::ROU))
            );
        }

        #[test]
        fn test_movra() {
            // mov $r0, $rou (move rou -> r0)
            let bytes: [u8; 2] = [
                0b0000_0101, // mov
                0b0110_0010, // r0, rou
            ];

            let mut validator = BulkValidator::with_bytes(bytes.to_vec());

            assert_eq!(
                validator.take_next_instruction(),
                Ok(Instruction::Mov(Register::R0, Register::ROU))
            );
        }

        #[test]
        fn test_addi() {
            // addi $r2, $r0, $r1 (r0 + r1 -> r2)
            let bytes: [u8; 3] = [
                0b0001_1010, // addi
                0b1000_0110, // r2, r0
                0b0111_0000, // r1
            ];

            let mut validator = BulkValidator::with_bytes(bytes.to_vec());

            assert_eq!(
                validator.take_next_instruction(),
                Ok(Instruction::Addi(Register::R2, Register::R0, Register::R1))
            );
        }
    }
}
