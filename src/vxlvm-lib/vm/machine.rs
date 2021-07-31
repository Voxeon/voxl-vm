use super::{Memory, Registers, Stack};
use crate::error::VMError;

use vxl_iset::execute_instruction::ExecuteInstruction;
use vxl_iset::instruction::Instruction;
use vxl_iset::instruction_arguments::{Address, Immediate, Register};
use vxl_iset::syscall_handler::SyscallHandler;

use alloc::vec::Vec;
use core::convert::TryInto;
use paste::paste;

macro_rules! compute_operation {
    ($t:ty, $op:ident) => {
        paste! {
            fn [<compute_ $t _ $op>](&self, a: $t, b: $t) -> Result<$t, VMError> {
                let value = match self.behaviour {
                    OverflowBehaviour::Wrapping => $t::[<wrapping_ $op>](
                        a,
                        b,
                    ),
                    OverflowBehaviour::Clamping => $t::[<saturating_ $op>](
                        a,b
                    ),
                    OverflowBehaviour::Reporting => {
                        if let Some(v) = $t::[<checked_ $op>](
                            a,b
                        ) {
                            v
                        } else {
                            return Err(VMError::IntegerOverflowError);
                        }
                    }
                };

                return Ok(value);
            }
        }
    };
}

macro_rules! compute_float_operation {
    ($op:ident, $op_symbol:tt) => {
        paste! {
            fn [<compute_float_ $op>](&self, a: u64, b: u64) -> u64 {
                let a_f64: f64 = unsafe { core::mem::transmute(a) };
                let b_f64: f64 = unsafe { core::mem::transmute(b) };

                return unsafe { core::mem::transmute(a_f64 $op_symbol b_f64) };
            }
        }
    };
}

type VMResult<T> = Result<T, VMError>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OverflowBehaviour {
    Wrapping,
    Clamping,
    Reporting,
}

pub struct VM {
    memory: Memory,
    stack: Stack,
    register_bank: Registers,
    instructions: Vec<Instruction>,
    ip: usize,
    halted: bool,
    behaviour: OverflowBehaviour,
}

impl Default for OverflowBehaviour {
    fn default() -> Self {
        return Self::Clamping;
    }
}

impl VM {
    const FLAGS_MASK: u64 = (u64::MAX - 0b111);
    const EQUALS_MASK: u64 = 0b001;
    const LESS_THAN_MASK: u64 = 0b010;
    const GREATER_THAN_MASK: u64 = 0b100;

    pub fn new(instructions: Vec<Instruction>) -> Self {
        return Self::new_fixed_start(instructions, 0);
    }

    pub fn new_fixed_start(instructions: Vec<Instruction>, ip: usize) -> Self {
        return Self {
            memory: Memory::default(),
            stack: Stack::default(),
            register_bank: Registers::default(),
            instructions,
            ip,
            halted: false,
            behaviour: OverflowBehaviour::default(),
        };
    }

    pub fn new_with_options(
        stack_size: usize,
        instructions: Vec<Instruction>,
        ip: usize,
        behaviour: OverflowBehaviour,
    ) -> Self {
        return Self {
            memory: Memory::new(),
            stack: Stack::new(stack_size),
            register_bank: Registers::default(),
            instructions,
            ip,
            halted: false,
            behaviour,
        };
    }

    pub fn run<H: SyscallHandler<Self>>(&mut self, handler: &mut H) -> VMResult<()> {
        while self.ip < self.instructions.len() && !self.halted {
            self.run_next(handler)?;
        }

        return Ok(());
    }

    pub fn run_next<H: SyscallHandler<Self>>(&mut self, handler: &mut H) -> VMResult<()> {
        if self.halted {
            return Err(VMError::SystemHalted);
        }

        if self.ip > self.instructions.len() {
            return Err(VMError::NoInstruction);
        }

        if !self.execute_instruction(self.instructions[self.ip], handler)? {
            self.ip += 1;
        }

        return Ok(());
    }

    pub fn halt(&mut self) {
        self.halted = true;
    }

    pub fn registers(&self) -> &Registers {
        return &self.register_bank;
    }

    pub fn registers_mut(&mut self) -> &mut Registers {
        return &mut self.register_bank;
    }

    pub fn stack(&self) -> &Stack {
        return &self.stack;
    }

    pub fn memory(&self) -> &Memory {
        return &self.memory;
    }

    pub fn memory_mut(&mut self) -> &mut Memory {
        return &mut self.memory;
    }

    fn push_stack(&mut self, value: u64) -> VMResult<()> {
        if !self
            .stack
            .insert_u64(self.register_bank.get_value(Register::RFP as u8), value)
        {
            return Err(VMError::AccessBeyondStackBounds);
        }

        self.register_bank.set_value(
            Register::RFP as u8,
            self.register_bank.get_value(Register::RFP as u8) + 8,
        );

        return Ok(());
    }

    fn pop_stack(&mut self) -> VMResult<u64> {
        if let Some(v) = self
            .stack
            .get_top_u64(self.register_bank.get_value(Register::RFP as u8))
        {
            self.register_bank.sub_value(Register::RFP as u8, 8);

            return Ok(v);
        } else {
            return Err(VMError::AccessBeyondStackBounds);
        }
    }

    compute_operation!(i64, add);
    compute_operation!(i64, sub);
    compute_operation!(i64, mul);

    fn compute_i64_div(&self, a: i64, b: i64) -> Result<i64, VMError> {
        match self.behaviour {
            OverflowBehaviour::Wrapping => {
                return Ok(a.wrapping_div(b));
            }
            OverflowBehaviour::Clamping | OverflowBehaviour::Reporting => {
                return a.checked_div(b).ok_or(VMError::IntegerOverflowError);
            }
        }
    }

    fn compute_i64_mod(&self, a: i64, b: i64) -> Result<i64, VMError> {
        if b == 0 {
            return Err(VMError::AttemptedModuloZeroOperation);
        }

        match self.behaviour {
            OverflowBehaviour::Wrapping => {
                return Ok(a.wrapping_rem(b));
            }
            OverflowBehaviour::Clamping | OverflowBehaviour::Reporting => {
                return a.checked_rem(b).ok_or(VMError::IntegerOverflowError);
            }
        }
    }

    compute_operation!(u64, add);
    compute_operation!(u64, sub);
    compute_operation!(u64, mul);

    fn compute_u64_div(&self, a: u64, b: u64) -> Result<u64, VMError> {
        match self.behaviour {
            OverflowBehaviour::Wrapping => {
                return Ok(a.wrapping_div(b));
            }
            OverflowBehaviour::Clamping | OverflowBehaviour::Reporting => {
                return a.checked_div(b).ok_or(VMError::IntegerOverflowError);
            }
        }
    }

    fn compute_u64_mod(&self, a: u64, b: u64) -> Result<u64, VMError> {
        if b == 0 {
            return Err(VMError::AttemptedModuloZeroOperation);
        }

        match self.behaviour {
            OverflowBehaviour::Wrapping => {
                return Ok(a.wrapping_rem(b));
            }
            OverflowBehaviour::Clamping | OverflowBehaviour::Reporting => {
                return a.checked_rem(b).ok_or(VMError::IntegerOverflowError);
            }
        }
    }

    compute_float_operation!(add, +);
    compute_float_operation!(sub, -);
    compute_float_operation!(div, /);
    compute_float_operation!(mul, *);
}

impl ExecuteInstruction for VM {
    type Machine = Self;
    // A value of true indicates that the IP was modified and shouldn't be updated this cycle.
    type Output = VMResult<bool>;

    fn execute_nop(&mut self) -> Self::Output {
        return Ok(false);
    }

    fn execute_syscall<H: SyscallHandler<Self::Machine>>(
        &mut self,
        handler: &mut H,
        i: Immediate,
    ) -> Self::Output {
        let output = handler
            .execute_call(i.into(), self)
            .ok_or(VMError::UnknownSystemCall(i.into()))?;

        self.register_bank.set_value(Register::ROU as u8, output);

        return Ok(false);
    }

    fn execute_ldb(&mut self, i: Immediate, r: Register) -> Self::Output {
        let mut value: u64 = i.into();
        value &= 0xFF;

        self.register_bank.set_value(r as u8, value);

        return Ok(false);
    }

    fn execute_ldi(&mut self, i: Immediate, r: Register) -> Self::Output {
        self.register_bank.set_value(r as u8, i.into());

        return Ok(false);
    }

    fn execute_ldf(&mut self, i: Immediate, r: Register) -> Self::Output {
        self.register_bank.set_value(r as u8, i.into());

        return Ok(false);
    }

    fn execute_mov(&mut self, r: Register, r1: Register) -> Self::Output {
        self.register_bank
            .set_value(r as u8, self.register_bank.get_value(r1 as u8));

        return Ok(false);
    }

    fn execute_push(&mut self, r: Register) -> Self::Output {
        let v = self.register_bank.get_value(r as u8);

        self.push_stack(v)?;

        return Ok(false);
    }

    fn execute_pop(&mut self, r: Register) -> Self::Output {
        let v = self.pop_stack()?;
        self.register_bank.set_value(r as u8, v);

        return Ok(false);
    }

    fn execute_sget(&mut self, r: Register, r1: Register) -> Self::Output {
        self.register_bank.set_value(
            r as u8,
            self.stack
                .get_top_u64(self.register_bank.get_value(r1 as u8))
                .ok_or(VMError::AccessBeyondStackBounds)?,
        );

        return Ok(false);
    }

    fn execute_malloc(&mut self, r: Register, r1: Register) -> Self::Output {
        let address = self.memory.allocate(self.register_bank.get_value(r1 as u8));
        self.register_bank
            .set_value(r as u8, address.ok_or(VMError::FailedMalloc)?);

        return Ok(false);
    }

    fn execute_malloci(&mut self, i: Immediate, r: Register) -> Self::Output {
        let address = self.memory.allocate(i.into());
        self.register_bank
            .set_value(r as u8, address.ok_or(VMError::FailedMalloc)?);

        return Ok(false);
    }

    fn execute_free(&mut self, r: Register) -> Self::Output {
        let address = self.register_bank.get_value(r as u8);

        if !self.memory.free(&address) {
            return Err(VMError::FailedFreeNoAddressError(address));
        }

        return Ok(false);
    }

    fn execute_freea(&mut self, a: Address) -> Self::Output {
        let address = a.into();

        if !self.memory.free(&address) {
            return Err(VMError::FailedFreeNoAddressError(address));
        }

        return Ok(false);
    }

    fn execute_setb(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output {
        // r = address
        // r1 = index
        // r2 = value
        let address = self.register_bank.get_value(r as u8);
        let index = self.register_bank.get_value(r1 as u8);
        let value = self.register_bank.get_value(r2 as u8);

        let block = self
            .memory
            .retrieve_mutable(&address)
            .ok_or(VMError::FailedSetNoAddressError(address))?;

        if index >= block.len() as u64 {
            return Err(VMError::IndexBeyondBoundsError(index, block.len() as u64));
        }

        // We want the lower bits
        block[index as usize] = (value & 0xFF) as u8;

        return Ok(false);
    }

    fn execute_seti(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output {
        // r = address
        // r1 = index
        // r2 = value
        let address = self.register_bank.get_value(r as u8);
        let index = self.register_bank.get_value(r1 as u8);
        let value = self.register_bank.get_value(r2 as u8);

        let block = self
            .memory
            .retrieve_mutable(&address)
            .ok_or(VMError::FailedSetNoAddressError(address))?;

        if index + 8 >= block.len() as u64 {
            return Err(VMError::IndexBeyondBoundsError(index, block.len() as u64));
        }

        for (i, b) in value.to_le_bytes().iter().enumerate() {
            block[index as usize + i] = *b;
        }

        return Ok(false);
    }

    fn execute_isetb(&mut self, i: Immediate, r: Register, r1: Register) -> Self::Output {
        // i = index
        // r = address
        // r1 = value

        let index = i.into();
        let address = self.register_bank.get_value(r as u8);
        let value = self.register_bank.get_value(r1 as u8);

        let block = self
            .memory
            .retrieve_mutable(&address)
            .ok_or(VMError::FailedSetNoAddressError(address))?;

        if index >= block.len() as u64 {
            return Err(VMError::IndexBeyondBoundsError(index, block.len() as u64));
        }

        // We want the lower bits
        block[index as usize] = (value & 0xFF) as u8;

        return Ok(false);
    }

    fn execute_iseti(&mut self, i: Immediate, r: Register, r1: Register) -> Self::Output {
        // i = index
        // r = address
        // r1 = value

        let index = i.into();
        let address = self.register_bank.get_value(r as u8);
        let value = self.register_bank.get_value(r1 as u8);

        let block = self
            .memory
            .retrieve_mutable(&address)
            .ok_or(VMError::FailedSetNoAddressError(address))?;

        if index + 8 >= block.len() as u64 {
            return Err(VMError::IndexBeyondBoundsError(index, block.len() as u64));
        }

        for (i, b) in value.to_le_bytes().iter().enumerate() {
            block[index as usize + i] = *b;
        }

        return Ok(false);
    }

    fn execute_getb(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output {
        // r = dest
        // r1 = address
        // r2 = offset

        let address = self.register_bank.get_value(r1 as u8);
        let offset = self.register_bank.get_value(r2 as u8);

        let loc = self
            .memory
            .retrieve(&address)
            .ok_or(VMError::FailedGetNoAddressError(address))?;

        if offset > loc.len() as u64 {
            return Err(VMError::IndexBeyondBoundsError(offset, loc.len() as u64));
        }

        self.register_bank
            .set_value(r as u8, loc[offset as usize] as u64);

        return Ok(false);
    }

    fn execute_geti(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output {
        // r = dest
        // r1 = address
        // r2 = offset

        let address = self.register_bank.get_value(r1 as u8);
        let offset = self.register_bank.get_value(r2 as u8);

        let loc = self
            .memory
            .retrieve(&address)
            .ok_or(VMError::FailedGetNoAddressError(address))?;

        if offset + 8 > loc.len() as u64 {
            return Err(VMError::IndexBeyondBoundsError(offset, loc.len() as u64));
        }

        let arr: [u8; 8] = loc[offset as usize..offset as usize + 8]
            .try_into()
            .expect("Unexpected conversion to u64 fail.");

        self.register_bank
            .set_value(r as u8, u64::from_le_bytes(arr));

        return Ok(false);
    }

    fn execute_igetb(&mut self, i: Immediate, r: Register, r1: Register) -> Self::Output {
        // i = offset
        // r = destination
        // r1 = address

        let address = self.register_bank.get_value(r1 as u8);
        let offset = i.into();

        let loc = self
            .memory
            .retrieve(&address)
            .ok_or(VMError::FailedGetNoAddressError(address))?;

        if offset > loc.len() as u64 {
            return Err(VMError::IndexBeyondBoundsError(offset, loc.len() as u64));
        }

        self.register_bank
            .set_value(r as u8, loc[offset as usize] as u64);

        return Ok(false);
    }

    fn execute_igeti(&mut self, i: Immediate, r: Register, r1: Register) -> Self::Output {
        // i = offset
        // r = destination
        // r1 = address

        let address = self.register_bank.get_value(r1 as u8);
        let offset = i.into();

        let loc = self
            .memory
            .retrieve(&address)
            .ok_or(VMError::FailedGetNoAddressError(address))?;

        if offset + 8 > loc.len() as u64 {
            return Err(VMError::IndexBeyondBoundsError(offset, loc.len() as u64));
        }

        let arr: [u8; 8] = loc[offset as usize..offset as usize + 8]
            .try_into()
            .expect("Unexpected conversion to u64 fail.");

        self.register_bank
            .set_value(r as u8, u64::from_le_bytes(arr));

        return Ok(false);
    }

    fn execute_last(&mut self, r: Register, r1: Register) -> Self::Output {
        // r = dest
        // r1 = address

        let address = self.register_bank.get_value(r1 as u8);
        let arr = self
            .memory
            .retrieve(&address)
            .ok_or(VMError::FailedGetNoAddressError(address))?;

        if arr.len() < 8 {
            self.register_bank.set_value(
                r as u8,
                *arr.last()
                    .ok_or(VMError::IndexBeyondBoundsError(1, arr.len() as u64))?
                    as u64,
            );
        } else {
            let mut last_word = [0u8; 8];

            for i in (0..8).rev() {
                last_word[7 - i] = arr[arr.len() - i - 1];
            }

            self.register_bank
                .set_value(r as u8, u64::from_le_bytes(last_word));
        }

        return Ok(false);
    }

    fn execute_length(&mut self, r: Register, r1: Register) -> Self::Output {
        // r = dest
        // r1 = address
        let address = self.register_bank.get_value(r1 as u8);

        let loc = self
            .memory
            .retrieve(&address)
            .ok_or(VMError::FailedGetNoAddressError(address))?;

        self.register_bank.set_value(r as u8, loc.len() as u64);

        return Ok(false);
    }

    fn execute_clone(&mut self, r: Register, r1: Register) -> Self::Output {
        // r = dest
        // r1 = src address

        let src_address = self.register_bank.get_value(r1 as u8);

        let bytes = self
            .memory
            .retrieve(&src_address)
            .ok_or(VMError::FailedGetNoAddressError(src_address))?
            .clone();

        self.register_bank.set_value(
            r as u8,
            self.memory
                .allocate_with(bytes)
                .ok_or(VMError::FailedMalloc)?,
        );

        return Ok(false);
    }

    fn execute_copy(
        &mut self,
        r: Register,
        r1: Register,
        r2: Register,
        r3: Register,
        r4: Register,
    ) -> Self::Output {
        // r = destination address
        // r1 = destination offset
        // r2 = src address
        // r3 = src offset
        // r4 = number of bytes

        let dest_address = self.register_bank.get_value(r as u8);
        let dest_offset = self.register_bank.get_value(r1 as u8);
        let src_address = self.register_bank.get_value(r2 as u8);
        let src_offset = self.register_bank.get_value(r3 as u8);
        let bytes = self.register_bank.get_value(r4 as u8);

        for i in 0..bytes {
            let s_off = src_offset + i;
            let d_off = dest_offset + i;

            self.memory.set(
                &dest_address,
                &d_off,
                self.memory.get(&src_address, &s_off)?,
            )?;
        }

        return Ok(false);
    }

    fn execute_copyi(
        &mut self,
        i: Immediate,
        i1: Immediate,
        i2: Immediate,
        r: Register,
        r1: Register,
    ) -> Self::Output {
        // i = destination offset
        // i1 = src offset
        // i2 = number of bytes
        // r = destination address
        // r1 = src address

        let dest_address = self.register_bank.get_value(r as u8);
        let dest_offset: u64 = i.into();
        let src_address = self.register_bank.get_value(r1 as u8);
        let src_offset: u64 = i1.into();
        let bytes: u64 = i2.into();

        for i in 0..bytes {
            self.memory.set(
                &dest_address,
                &(dest_offset + i),
                self.memory.get(&src_address, &(src_offset + i))?,
            )?;
        }

        return Ok(false);
    }

    fn execute_addi(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output {
        // r = dest
        // r1 = lhs
        // r2 = rhs

        let value = self.compute_i64_add(
            self.register_bank.get_value(r1 as u8) as i64,
            self.register_bank.get_value(r2 as u8) as i64,
        )?;

        self.register_bank.set_value(r as u8, value as u64);

        return Ok(false);
    }

    fn execute_subi(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output {
        // r = dest
        // r1 = lhs
        // r2 = rhs

        let value = self.compute_i64_sub(
            self.register_bank.get_value(r1 as u8) as i64,
            self.register_bank.get_value(r2 as u8) as i64,
        )?;

        self.register_bank.set_value(r as u8, value as u64);

        return Ok(false);
    }

    fn execute_muli(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output {
        // r = dest
        // r1 = lhs
        // r2 = rhs

        let value = self.compute_i64_mul(
            self.register_bank.get_value(r1 as u8) as i64,
            self.register_bank.get_value(r2 as u8) as i64,
        )?;

        self.register_bank.set_value(r as u8, value as u64);

        return Ok(false);
    }

    fn execute_divi(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output {
        // r = dest
        // r1 = lhs
        // r2 = rhs

        let value = self.compute_i64_div(
            self.register_bank.get_value(r1 as u8) as i64,
            self.register_bank.get_value(r2 as u8) as i64,
        )?;

        self.register_bank.set_value(r as u8, value as u64);

        return Ok(false);
    }

    fn execute_modi(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output {
        // r = dest
        // r1 = lhs
        // r2 = rhs

        let value = self.compute_i64_mod(
            self.register_bank.get_value(r1 as u8) as i64,
            self.register_bank.get_value(r2 as u8) as i64,
        )?;

        self.register_bank.set_value(r as u8, value as u64);

        return Ok(false);
    }

    fn execute_addu(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output {
        // r = dest
        // r1 = lhs
        // r2 = rhs

        let value = self.compute_u64_add(
            self.register_bank.get_value(r1 as u8),
            self.register_bank.get_value(r2 as u8),
        )?;

        self.register_bank.set_value(r as u8, value as u64);

        return Ok(false);
    }

    fn execute_subu(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output {
        // r = dest
        // r1 = lhs
        // r2 = rhs

        let value = self.compute_u64_sub(
            self.register_bank.get_value(r1 as u8),
            self.register_bank.get_value(r2 as u8),
        )?;

        self.register_bank.set_value(r as u8, value as u64);

        return Ok(false);
    }

    fn execute_mulu(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output {
        // r = dest
        // r1 = lhs
        // r2 = rhs

        let value = self.compute_u64_mul(
            self.register_bank.get_value(r1 as u8),
            self.register_bank.get_value(r2 as u8),
        )?;

        self.register_bank.set_value(r as u8, value as u64);

        return Ok(false);
    }

    fn execute_divu(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output {
        // r = dest
        // r1 = lhs
        // r2 = rhs

        let value = self.compute_u64_div(
            self.register_bank.get_value(r1 as u8),
            self.register_bank.get_value(r2 as u8),
        )?;

        self.register_bank.set_value(r as u8, value as u64);

        return Ok(false);
    }

    fn execute_modu(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output {
        // r = dest
        // r1 = lhs
        // r2 = rhs

        let value = self.compute_u64_mod(
            self.register_bank.get_value(r1 as u8),
            self.register_bank.get_value(r2 as u8),
        )?;

        self.register_bank.set_value(r as u8, value as u64);

        return Ok(false);
    }

    fn execute_addf(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output {
        // r = dest
        // r1 = lhs
        // r2 = rhs

        let value = self.compute_float_add(
            self.register_bank.get_value(r1 as u8),
            self.register_bank.get_value(r2 as u8),
        );

        self.register_bank.set_value(r as u8, value);

        return Ok(false);
    }

    fn execute_subf(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output {
        // r = dest
        // r1 = lhs
        // r2 = rhs

        let value = self.compute_float_sub(
            self.register_bank.get_value(r1 as u8),
            self.register_bank.get_value(r2 as u8),
        );

        self.register_bank.set_value(r as u8, value);

        return Ok(false);
    }

    fn execute_mulf(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output {
        // r = dest
        // r1 = lhs
        // r2 = rhs

        let value = self.compute_float_mul(
            self.register_bank.get_value(r1 as u8),
            self.register_bank.get_value(r2 as u8),
        );

        self.register_bank.set_value(r as u8, value);

        return Ok(false);
    }

    fn execute_divf(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output {
        // r = dest
        // r1 = lhs
        // r2 = rhs

        let value = self.compute_float_div(
            self.register_bank.get_value(r1 as u8),
            self.register_bank.get_value(r2 as u8),
        );

        self.register_bank.set_value(r as u8, value);

        return Ok(false);
    }

    fn execute_rotl(&mut self, r: Register, r1: Register) -> Self::Output {
        // r = register
        // r1 = number of rotations

        let rotations = self.register_bank.get_value(r1 as u8);
        let value = self.register_bank.get_value(r as u8);

        self.register_bank.set_value(
            r as u8,
            value.rotate_left(rotations.try_into().unwrap_or(u32::MAX)),
        );

        return Ok(false);
    }

    fn execute_rotli(&mut self, i: Immediate, r: Register) -> Self::Output {
        // i = number of rotations
        // r = register

        let rotations: u64 = i.into();
        let value = self.register_bank.get_value(r as u8);

        self.register_bank.set_value(
            r as u8,
            value.rotate_left(rotations.try_into().unwrap_or(u32::MAX)),
        );

        return Ok(false);
    }

    fn execute_rotr(&mut self, r: Register, r1: Register) -> Self::Output {
        // r = register
        // r1 = number of rotations

        let rotations = self.register_bank.get_value(r1 as u8);
        let value = self.register_bank.get_value(r as u8);

        self.register_bank.set_value(
            r as u8,
            value.rotate_right(rotations.try_into().unwrap_or(u32::MAX)),
        );

        return Ok(false);
    }

    fn execute_rotri(&mut self, i: Immediate, r: Register) -> Self::Output {
        // r = register
        // i = number of rotations

        let rotations: u64 = i.into();
        let value = self.register_bank.get_value(r as u8);

        self.register_bank.set_value(
            r as u8,
            value.rotate_right(rotations.try_into().unwrap_or(u32::MAX)),
        );

        return Ok(false);
    }

    fn execute_sll(&mut self, r: Register, r1: Register) -> Self::Output {
        // r = register to shift
        // r1 = amount to shift

        let shift = self.register_bank.get_value(r1 as u8);

        self.register_bank
            .set_value(r as u8, self.register_bank.get_value(r as u8) << shift);

        return Ok(false);
    }

    fn execute_slli(&mut self, i: Immediate, r: Register) -> Self::Output {
        // r = register to shift
        // i = amount to shift

        let shift: u64 = i.into();

        self.register_bank
            .set_value(r as u8, self.register_bank.get_value(r as u8) << shift);

        return Ok(false);
    }

    fn execute_srl(&mut self, r: Register, r1: Register) -> Self::Output {
        // r = register to shift
        // r1 = amount to shift

        let shift = self.register_bank.get_value(r1 as u8);

        self.register_bank
            .set_value(r as u8, self.register_bank.get_value(r as u8) >> shift);

        return Ok(false);
    }

    fn execute_srli(&mut self, i: Immediate, r: Register) -> Self::Output {
        // r = register to shift
        // i = amount to shift

        let shift: u64 = i.into();

        self.register_bank
            .set_value(r as u8, self.register_bank.get_value(r as u8) >> shift);

        return Ok(false);
    }

    fn execute_not(&mut self, r: Register) -> Self::Output {
        self.register_bank
            .set_value(r as u8, !self.register_bank.get_value(r as u8));

        return Ok(false);
    }

    fn execute_and(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output {
        // r = dest
        // r1 = lhs
        // r2 = rhs

        let lhs = self.register_bank.get_value(r1 as u8);
        let rhs = self.register_bank.get_value(r2 as u8);

        self.register_bank.set_value(r as u8, lhs & rhs);

        return Ok(false);
    }

    fn execute_or(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output {
        // r = dest
        // r1 = lhs
        // r2 = rhs

        let lhs = self.register_bank.get_value(r1 as u8);
        let rhs = self.register_bank.get_value(r2 as u8);

        self.register_bank.set_value(r as u8, lhs | rhs);

        return Ok(false);
    }

    fn execute_xor(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output {
        // r = dest
        // r1 = lhs
        // r2 = rhs

        let lhs = self.register_bank.get_value(r1 as u8);
        let rhs = self.register_bank.get_value(r2 as u8);

        self.register_bank.set_value(r as u8, lhs ^ rhs);

        return Ok(false);
    }

    fn execute_cmp(&mut self, r: Register, r1: Register) -> Self::Output {
        // r = lhs
        // r1 = rhs

        let lhs = self.register_bank.get_value(r as u8);
        let rhs = self.register_bank.get_value(r1 as u8);

        let v = match lhs.cmp(&rhs) {
            core::cmp::Ordering::Less => 0b010,
            core::cmp::Ordering::Equal => 0b001,
            core::cmp::Ordering::Greater => 0b100,
        };

        self.register_bank.set_value(
            Register::RFL as u8,
            self.register_bank.get_value(Register::RFL as u8) & Self::FLAGS_MASK | v,
        );

        return Ok(false);
    }

    fn execute_cmpi(&mut self, r: Register, r1: Register) -> Self::Output {
        // r = lhs
        // r1 = rhs

        let lhs = self.register_bank.get_value(r as u8) as i64;
        let rhs = self.register_bank.get_value(r1 as u8) as i64;

        let v = match lhs.cmp(&rhs) {
            core::cmp::Ordering::Less => 0b010,
            core::cmp::Ordering::Equal => 0b001,
            core::cmp::Ordering::Greater => 0b100,
        };

        self.register_bank.set_value(
            Register::RFL as u8,
            self.register_bank.get_value(Register::RFL as u8) & Self::FLAGS_MASK | v,
        );

        return Ok(false);
    }

    fn execute_cmpf(&mut self, r: Register, r1: Register) -> Self::Output {
        // r = lhs
        // r1 = rhs

        let lhs: f64 = unsafe { core::mem::transmute(self.register_bank.get_value(r as u8)) };
        let rhs: f64 = unsafe { core::mem::transmute(self.register_bank.get_value(r1 as u8)) };

        let value;

        if lhs < rhs {
            value = 0b010;
        } else if lhs == rhs {
            value = 0b001;
        } else if lhs > rhs {
            value = 0b100;
        } else {
            value = 0b000;
        }

        self.register_bank.set_value(
            Register::RFL as u8,
            self.register_bank.get_value(Register::RFL as u8) & Self::FLAGS_MASK | value,
        );

        return Ok(false);
    }

    fn execute_jmp(&mut self, a: Address) -> Self::Output {
        let new_ip: u64 = a.into();
        self.ip = new_ip as usize;

        return Ok(true);
    }

    fn execute_jeq(&mut self, a: Address) -> Self::Output {
        let flags = self.register_bank.get_value(Register::RFL as u8);

        if (flags & Self::EQUALS_MASK) == 1 {
            let new_ip: u64 = a.into();
            self.ip = new_ip as usize;

            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    fn execute_jne(&mut self, a: Address) -> Self::Output {
        let flags = self.register_bank.get_value(Register::RFL as u8);

        if (flags & Self::EQUALS_MASK) == 0 {
            let new_ip: u64 = a.into();
            self.ip = new_ip as usize;

            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    fn execute_jge(&mut self, a: Address) -> Self::Output {
        let flags = self.register_bank.get_value(Register::RFL as u8);

        if (flags & Self::GREATER_THAN_MASK) >> 2 == 1 || (flags & Self::EQUALS_MASK) == 1 {
            let new_ip: u64 = a.into();
            self.ip = new_ip as usize;

            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    fn execute_jgt(&mut self, a: Address) -> Self::Output {
        let flags = self.register_bank.get_value(Register::RFL as u8);

        if (flags & Self::GREATER_THAN_MASK) >> 2 == 1 {
            let new_ip: u64 = a.into();
            self.ip = new_ip as usize;

            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    fn execute_jle(&mut self, a: Address) -> Self::Output {
        let flags = self.register_bank.get_value(Register::RFL as u8);

        if (flags & Self::LESS_THAN_MASK) >> 1 == 1 || (flags & Self::EQUALS_MASK) == 1 {
            let new_ip: u64 = a.into();
            self.ip = new_ip as usize;

            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    fn execute_jlt(&mut self, a: Address) -> Self::Output {
        let flags = self.register_bank.get_value(Register::RFL as u8);

        if (flags & Self::LESS_THAN_MASK) >> 1 == 1 {
            let new_ip: u64 = a.into();
            self.ip = new_ip as usize;

            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    fn execute_i2f(&mut self, r: Register) -> Self::Output {
        let v = self.register_bank.get_value(r as u8) as i64;

        let value: f64 = v as f64;
        let float = unsafe { core::mem::transmute(value) };

        self.register_bank.set_value(r as u8, float);

        return Ok(false);
    }

    fn execute_f2i(&mut self, r: Register) -> Self::Output {
        let v: f64 = unsafe { core::mem::transmute(self.register_bank.get_value(r as u8)) };

        let value = (v as i64) as u64;

        self.register_bank.set_value(r as u8, value);

        return Ok(false);
    }

    fn execute_swpa(&mut self, a: Address, a1: Address) -> Self::Output {
        let a_mem = self
            .memory
            .take(&a.into())
            .ok_or(VMError::FailedGetNoAddressError(a.into()))?;

        let a1_mem = self
            .memory
            .take(&a1.into())
            .ok_or(VMError::FailedGetNoAddressError(a1.into()))?;

        self.memory.assign_empty(a.into(), a1_mem);
        self.memory.assign_empty(a1.into(), a_mem);

        return Ok(false);
    }

    fn execute_swpar(&mut self, r: Register, r1: Register) -> Self::Output {
        let a = self.register_bank.get_value(r as u8);
        let a1 = self.register_bank.get_value(r1 as u8);

        let a_mem = self
            .memory
            .take(&a)
            .ok_or(VMError::FailedGetNoAddressError(a.into()))?;

        let a1_mem = self
            .memory
            .take(&a1)
            .ok_or(VMError::FailedGetNoAddressError(a1.into()))?;

        self.memory.assign_empty(a, a1_mem);
        self.memory.assign_empty(a1, a_mem);

        return Ok(false);
    }

    fn execute_swpr(&mut self, r: Register, r1: Register) -> Self::Output {
        // a = r
        // b = r1

        let temp = self.register_bank.get_value(r as u8);

        self.register_bank
            .set_value(r as u8, self.register_bank.get_value(r1 as u8));
        self.register_bank.set_value(r1 as u8, temp);

        return Ok(false);
    }

    fn execute_call(&mut self, a: Address) -> Self::Output {
        self.push_stack(self.register_bank.get_value(Register::RFP as u8))?;
        self.push_stack(self.register_bank.get_value(Register::RSP as u8))?;
        self.push_stack(self.ip as u64 + 1)?;

        self.register_bank.set_value(
            Register::RSP as u8,
            self.register_bank.get_value(Register::RFP as u8),
        );

        let new_ip: u64 = a.into();

        self.ip = new_ip as usize;

        return Ok(true);
    }

    fn execute_ret(&mut self) -> Self::Output {
        let ip = self.pop_stack()?;
        let rsp = self.pop_stack()?;
        let rfp = self.pop_stack()?;

        self.ip = ip as usize;
        self.register_bank.set_value(Register::RSP as u8, rsp);
        self.register_bank.set_value(Register::RFP as u8, rfp);

        return Ok(true);
    }

    fn execute_halt(&mut self) -> Self::Output {
        self.halt();

        return Ok(false);
    }
}
