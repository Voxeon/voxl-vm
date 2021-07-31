#[derive(Debug)]
pub struct Registers {
    registers: [u64; 16],
    wrapping: bool,
}

impl Registers {
    pub fn new(wrapping: bool) -> Self {
        return Self {
            registers: [0; 16],
            wrapping,
        };
    }

    pub fn get_value(&self, register: u8) -> u64 {
        if register >= 16 {
            panic!("Invalid register {}", register);
        }

        return self.registers[register as usize];
    }

    pub fn set_value(&mut self, register: u8, value: u64) {
        if register >= 16 {
            panic!("Invalid register {}", register);
        }

        self.registers[register as usize] = value;
    }

    pub fn add_value(&mut self, register: u8, value: u64) {
        if register >= 16 {
            panic!("Invalid register {}", register);
        }

        if self.wrapping {
            self.registers[register as usize] =
                self.registers[register as usize].wrapping_add(value);
        } else {
            self.registers[register as usize] = self.registers[register as usize]
                .checked_add(value)
                .unwrap_or(u64::MAX);
        }
    }

    pub fn sub_value(&mut self, register: u8, value: u64) {
        if register >= 16 {
            panic!("Invalid register {}", register);
        }

        if self.wrapping {
            self.registers[register as usize] =
                self.registers[register as usize].wrapping_sub(value);
        } else {
            self.registers[register as usize] = self.registers[register as usize]
                .checked_sub(value)
                .unwrap_or(0);
        }
    }
}

impl Default for Registers {
    fn default() -> Self {
        return Self {
            registers: [0u64; 16],
            wrapping: false,
        };
    }
}

#[cfg(test)]
mod tests {
    use vxl_iset::instruction_arguments::Register;

    use super::*;

    #[test]
    fn test_wrapping_add() {
        let mut registers = Registers::new(true);

        registers.set_value(Register::R0 as u8, u64::MAX - 10);

        assert_eq!(registers.get_value(Register::R0 as u8), u64::MAX - 10);

        registers.add_value(Register::R0 as u8, 11);

        assert_eq!(registers.get_value(Register::R0 as u8), 0);
    }

    #[test]
    fn test_clamped_add() {
        let mut registers = Registers::new(false);

        registers.set_value(Register::R0 as u8, u64::MAX - 10);

        assert_eq!(registers.get_value(Register::R0 as u8), u64::MAX - 10);

        registers.add_value(Register::R0 as u8, 11);

        assert_eq!(registers.get_value(Register::R0 as u8), u64::MAX);
    }

    #[test]
    fn test_wrapping_sub() {
        let mut registers = Registers::new(true);

        registers.set_value(Register::R0 as u8, 10);
        assert_eq!(registers.get_value(Register::R0 as u8), 10);

        registers.sub_value(Register::R0 as u8, 11);
        assert_eq!(registers.get_value(Register::R0 as u8), u64::MAX);
    }

    #[test]
    fn test_clamped_sub() {
        let mut registers = Registers::new(false);

        registers.set_value(Register::R0 as u8, 10);
        assert_eq!(registers.get_value(Register::R0 as u8), 10);

        registers.sub_value(Register::R0 as u8, 11);
        assert_eq!(registers.get_value(Register::R0 as u8), 0);
    }
}
