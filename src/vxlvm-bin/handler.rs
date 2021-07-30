use voxl_instruction_set::instruction_arguments::Register;
use voxl_instruction_set::syscall_handler::SyscallHandler;
use vxlvm::vm::VM;

use std::io::{self, stdout, Read, Write};
use std::collections::{BTreeMap};
use std::fs::File;
use std::time;

pub struct OSHandler {
    files: BTreeMap<u64, File>,
    /// Used to track where we can start searching from for a new id.
    lowest_removed_file_id: Option<u64>,
}

impl OSHandler {
    pub fn new() -> Self {
        return Self {
            files: BTreeMap::new(),
            lowest_removed_file_id: None,
        };
    }

    fn determine_next_id(&mut self) -> Option<u64> {
        if let Some((last, _)) = self.files.iter().next_back() {
            if *last < u64::MAX - 1 {
                return Some(*last + 1);
            } else if u64::MAX - 1 == self.files.len() as u64 - 1 {
                // u64::MAX is reserved.
                return None;
            } else {
                // Every id has been used once, so lets find a spare one.
                if let Some(id) = self.lowest_removed_file_id {
                    let mut prev = None;
                    let mut res = None;

                    for (id, _) in self.files.iter().skip(id as usize) {
                        if let Some(prev) = prev {
                            if *id - prev >= 2 {
                                res = Some(prev + 1);
                                break;
                            }
                        } else if *id == u64::MAX - 1 {
                            return None;
                        } else {
                            prev = Some(*id);
                        }
                    }

                    // Clear. We only optimise the first id after closing the previous one.
                    self.lowest_removed_file_id = None;
                    return res;
                } else {
                    let mut prev = None;

                    for (id, _) in self.files.iter() {
                        if let Some(prev) = prev {
                            if *id - prev >= 2 {
                                return Some(prev + 1);
                            }
                        } else {
                            prev = Some(*id);
                        }
                    }

                    // Theoretically should never be reached. Should be caught in the length check.
                    return None;
                }
            }
        } else {
            return Some(0);
        }
    }

    fn release_id(&mut self, id: u64) -> Option<File> {
        let f = self.files.remove(&id)?;

        if let Some(lowest) = self.lowest_removed_file_id {
            if id < lowest {
                self.lowest_removed_file_id = Some(lowest);
            }
        } else {
            self.lowest_removed_file_id = Some(id);
        }

        return Some(f);
    }
}

impl SyscallHandler<VM> for OSHandler {
    fn execute_target_specific_call(&mut self, _call: u64, _machine: &mut VM) -> Option<u64> {
        return None;
    }

    fn exit(&mut self, machine: &mut VM) -> Option<u64> {
        std::process::exit(
            (machine.registers().get_value(Register::R0 as u8) % (u32::MAX as u64)) as i32,
        );
    }

    fn write_byte_terminal(&mut self, machine: &mut VM) -> Option<u64> {
        let byte = (machine.registers().get_value(Register::R0 as u8) & 0xff) as u8;

        if stdout().write(&[byte]).is_err() {
            return Some(1);
        } else {
            return Some(0);
        }
    }

    fn write_terminal(&mut self, machine: &mut VM) -> Option<u64> {
        let ptr = machine.registers().get_value(Register::R0 as u8);

        if let Some(string_bytes) = machine.memory().retrieve(&ptr) {
            if stdout().write(string_bytes).is_err() {
                return Some(2);
            }

            return Some(0);
        } else {
            return Some(1);
        }
    }

    fn read_byte_terminal(&mut self, _machine: &mut VM) -> Option<u64> {
        let mut byte = [0u8];

        let _ = io::stdin().read(&mut byte);

        return Some(byte[0] as u64);
    }

    fn read_terminal(&mut self, machine: &mut VM) -> Option<u64> {
        let ptr = machine.registers().get_value(Register::R0 as u8);

        if let Some(dest) = machine.memory_mut().retrieve_mutable(&ptr) {
            if let Ok(n) = io::stdin().read(dest) {
                return Some(n as u64);
            } else {
                return Some(0);
            }
        } else {
            return Some(0);
        }
    }

    fn open_file(&mut self, machine: &mut VM) -> Option<u64> {
        let id = self.determine_next_id();

        todo!();
    }

    fn close_file(&mut self, machine: &mut VM) -> Option<u64> {
        todo!()
    }

    fn read_file(&mut self, machine: &mut VM) -> Option<u64> {
        todo!()
    }

    fn write_file(&mut self, machine: &mut VM) -> Option<u64> {
        todo!()
    }

    fn execute_file(&mut self, machine: &mut VM) -> Option<u64> {
        todo!()
    }

    fn execute_vxl_file(&mut self, machine: &mut VM) -> Option<u64> {
        todo!()
    }

    fn delete_file(&mut self, machine: &mut VM) -> Option<u64> {
        todo!()
    }

    fn move_file(&mut self, machine: &mut VM) -> Option<u64> {
        todo!()
    }

    fn copy_file(&mut self, machine: &mut VM) -> Option<u64> {
        todo!()
    }

    fn time_of_day(&mut self, _machine: &mut VM) -> Option<u64> {
        return time::SystemTime::now().duration_since(time::UNIX_EPOCH).ok().map(|d| d.as_secs());
    }
}
