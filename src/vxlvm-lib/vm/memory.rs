use alloc::collections::{BTreeMap, BinaryHeap};
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::Reverse;

use crate::error::VMError;

#[derive(Debug)]
pub struct Memory {
    memory: BTreeMap<u64, Vec<u8>>,
    freed_addresses: BinaryHeap<Reverse<u64>>,
}

impl Memory {
    pub fn new() -> Self {
        return Self {
            memory: BTreeMap::new(),
            freed_addresses: BinaryHeap::new(),
        };
    }

    pub fn allocate(&mut self, size: u64) -> Option<u64> {
        let address = self.alloc_next_address()?;

        self.memory.insert(address, vec![0; size as usize]);

        return Some(address);
    }

    pub fn assign(&mut self, address: u64, data: Vec<u8>) -> bool {
        if !self.memory.contains_key(&address) {
            return false;
        }

        self.memory.insert(address, data);

        return true;
    }

    pub fn assign_empty(&mut self, address: u64, data: Vec<u8>) -> bool {
        if self.memory.contains_key(&address) {
            return false;
        }

        self.memory.insert(address, data);

        return true;
    }

    pub fn allocate_with(&mut self, bytes: Vec<u8>) -> Option<u64> {
        let address = self.alloc_next_address()?;

        self.memory.insert(address, bytes);

        return Some(address);
    }

    pub fn free(&mut self, address: &u64) -> bool {
        let success = self.memory.remove(address).is_some();

        if success {
            if *address as usize != self.memory.len() {
                self.freed_addresses.push(Reverse(*address));
            }
        }

        return success;
    }

    pub fn retrieve(&self, address: &u64) -> Option<&Vec<u8>> {
        return self.memory.get(address);
    }

    pub fn retrieve_mutable(&mut self, address: &u64) -> Option<&mut Vec<u8>> {
        return self.memory.get_mut(address);
    }

    pub fn set(&mut self, address: &u64, offset: &u64, value: u8) -> Result<(), VMError> {
        if let Some(bytes) = self.retrieve_mutable(address) {
            if *offset >= bytes.len() as u64 {
                return Err(VMError::IndexBeyondBoundsError(*offset, bytes.len() as u64));
            }

            bytes[*offset as usize] = value;
            return Ok(());
        } else {
            return Err(VMError::FailedSetNoAddressError(*address));
        }
    }

    pub fn get(&self, address: &u64, offset: &u64) -> Result<u8, VMError> {
        if let Some(bytes) = self.retrieve(address) {
            if *offset >= bytes.len() as u64 {
                return Err(VMError::IndexBeyondBoundsError(*offset, bytes.len() as u64));
            }

            return Ok(bytes[*offset as usize]);
        } else {
            return Err(VMError::FailedGetNoAddressError(*address));
        }
    }

    pub fn take(&mut self, address: &u64) -> Option<Vec<u8>> {
        return self.memory.remove(address);
    }

    #[inline]
    fn alloc_next_address(&mut self) -> Option<u64> {
        if let Some(a) = self.freed_addresses.pop() {
            return Some(a.0);
        } else {
            return Some(self.memory.len() as u64);
        }
    }

    pub fn total_allocated(&self) -> u64 {
        let mut total = 0;

        for (_, loc) in &self.memory {
            total += loc.len() as u64;
        }

        return total;
    }
}

impl Default for Memory {
    fn default() -> Self {
        return Memory::new();
    }
}
