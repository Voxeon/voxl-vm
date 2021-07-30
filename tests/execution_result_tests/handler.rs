use std::cell::RefCell;
use std::rc::Rc;

use voxl_instruction_set::instruction_arguments::Register;
use voxl_instruction_set::syscall_handler::SyscallHandler;
use vxlvm::vm::VM;

pub struct System {
    printed: Rc<RefCell<Vec<String>>>,
}

impl<'a> System {
    pub fn new() -> Self {
        return Self {
            printed: Rc::new(RefCell::new(Vec::new())),
        };
    }

    pub fn with_buffer(printed: Rc<RefCell<Vec<String>>>) -> Self {
        return Self { printed };
    }
}

impl SyscallHandler<VM> for System {
    fn execute_target_specific_call(&mut self, _call: u64, _machine: &mut VM) -> Option<u64> {
        return None;
    }

    fn exit(&mut self, _machine: &mut VM) -> Option<u64> {
        panic!("Failed. Unexpectedly reached an unimplemented handler function.");
    }

    fn write_byte_terminal(&mut self, _machine: &mut VM) -> Option<u64> {
        panic!("Failed. Unexpectedly reached an unimplemented handler function.");
    }

    fn write_terminal(&mut self, machine: &mut VM) -> Option<u64> {
        self.printed.borrow_mut().push(format!(
            "{}\n",
            machine.registers().get_value(Register::R0 as u8)
        ));

        return Some(0);
    }

    fn read_byte_terminal(&mut self, _machine: &mut VM) -> Option<u64> {
        panic!("Failed. Unexpectedly reached an unimplemented handler function.");
    }

    fn read_terminal(&mut self, _machine: &mut VM) -> Option<u64> {
        panic!("Failed. Unexpectedly reached an unimplemented handler function.");
    }

    fn open_file(&mut self, _machine: &mut VM) -> Option<u64> {
        panic!("Failed. Unexpectedly reached an unimplemented handler function.");
    }

    fn close_file(&mut self, _machine: &mut VM) -> Option<u64> {
        panic!("Failed. Unexpectedly reached an unimplemented handler function.");
    }

    fn read_file(&mut self, _machine: &mut VM) -> Option<u64> {
        panic!("Failed. Unexpectedly reached an unimplemented handler function.");
    }

    fn write_file(&mut self, _machine: &mut VM) -> Option<u64> {
        panic!("Failed. Unexpectedly reached an unimplemented handler function.");
    }

    fn execute_file(&mut self, _machine: &mut VM) -> Option<u64> {
        panic!("Failed. Unexpectedly reached an unimplemented handler function.");
    }

    fn execute_vxl_file(&mut self, _machine: &mut VM) -> Option<u64> {
        panic!("Failed. Unexpectedly reached an unimplemented handler function.");
    }

    fn delete_file(&mut self, _machine: &mut VM) -> Option<u64> {
        panic!("Failed. Unexpectedly reached an unimplemented handler function.");
    }

    fn move_file(&mut self, _machine: &mut VM) -> Option<u64> {
        panic!("Failed. Unexpectedly reached an unimplemented handler function.");
    }

    fn copy_file(&mut self, _machine: &mut VM) -> Option<u64> {
        panic!("Failed. Unexpectedly reached an unimplemented handler function.");
    }

    fn time_of_day(&mut self, _machine: &mut VM) -> Option<u64> {
        panic!("Failed. Unexpectedly reached an unimplemented handler function.");
    }
}
