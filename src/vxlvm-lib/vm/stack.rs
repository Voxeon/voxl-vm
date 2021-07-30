use core::convert::TryInto;

use alloc::vec;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct Stack {
    items: Vec<u8>,
}

impl Stack {
    // 2MB in bytes
    const DEFAULT_SIZE: usize = 2000 * 1000;

    pub fn new(size: usize) -> Self {
        return Self {
            items: vec![0; size],
        };
    }

    pub fn insert(&mut self, index: u64, values: Vec<u8>) -> bool {
        for i in 0..values.len() {
            if index as usize + i >= self.items.len() {
                return false;
            }

            self.items[index as usize + i] = values[i];
        }

        return true;
    }

    pub fn insert_u64(&mut self, index: u64, value: u64) -> bool {
        return self.insert(index, value.to_le_bytes().to_vec());
    }

    pub fn get_top(&self, top: u64, amount: u64) -> Option<&[u8]> {
        if top > self.items.len() as u64 || amount > self.items.len() as u64 || top < amount {
            return None;
        }

        return Some(&self.items[top as usize - (amount as usize)..top as usize]);
    }

    pub fn get_top_u64(&self, top: u64) -> Option<u64> {
        let bytes: &[u8; 8] = self.get_top(top, 8)?.try_into().ok()?;
        return Some(u64::from_le_bytes(*bytes));
    }
}

impl Default for Stack {
    fn default() -> Self {
        return Self::new(Self::DEFAULT_SIZE);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_bytes() {
        let mut stack = Stack::new(10);
        let mut top = 0;

        assert!(stack.insert(top, vec![1, 2, 3, 4, 5, 6]));
        top += 6;

        assert_eq!(stack.items, vec![1, 2, 3, 4, 5, 6, 0, 0, 0, 0]);

        assert!(stack.insert(top, vec![1, 2, 3, 4]));
        assert_eq!(stack.items, vec![1, 2, 3, 4, 5, 6, 1, 2, 3, 4]);
    }

    #[test]
    fn test_push_u64() {
        let mut stack = Stack::new(16);

        assert!(stack.insert_u64(0, 442));
        assert_eq!(
            stack.items,
            vec![186, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );

        assert!(stack.insert_u64(8, 1192));

        assert_eq!(
            stack.items,
            vec![186, 1, 0, 0, 0, 0, 0, 0, 168, 4, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn test_failed_push_bytes() {
        let mut stack = Stack::new(10);
        let mut top = 0;
        assert!(stack.insert(top, vec![1, 2, 3, 4, 5, 6]));
        top += 6;

        assert_eq!(stack.items, vec![1, 2, 3, 4, 5, 6, 0, 0, 0, 0]);
        assert!(stack.insert(top, vec![1, 2, 3, 4]));

        top += 4;
        assert_eq!(stack.items, vec![1, 2, 3, 4, 5, 6, 1, 2, 3, 4]);

        assert!(!stack.insert(top, vec![1]));
    }

    #[test]
    fn test_failed_push_bytes_2() {
        let mut stack = Stack::new(10);

        assert!(!stack.insert(0, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]));
    }

    #[test]
    fn test_pop_bytes() {
        let mut stack = Stack::new(10);
        let mut top = 0;

        assert!(stack.insert(top, vec![1, 2, 3, 4, 5, 6]));
        top += 6;

        assert_eq!(stack.items, vec![1, 2, 3, 4, 5, 6, 0, 0, 0, 0]);

        assert!(stack.insert(top, vec![1, 2, 3, 4]));
        top += 4;
        assert_eq!(stack.items, vec![1, 2, 3, 4, 5, 6, 1, 2, 3, 4]);

        assert_eq!(stack.get_top(top, 4).unwrap(), &[1, 2, 3, 4]);
        top -= 4;
        assert_eq!(stack.items, vec![1, 2, 3, 4, 5, 6, 1, 2, 3, 4]);
        assert!(stack.insert(top, vec![88, 89, 90, 91]));
        assert_eq!(stack.items, vec![1, 2, 3, 4, 5, 6, 88, 89, 90, 91]);
    }

    #[test]
    fn test_failed_pop_bytes() {
        let mut stack = Stack::new(10);
        let mut top = 0;

        assert!(stack.insert(top, vec![1, 2, 3, 4, 5, 6]));
        top += 6;

        assert_eq!(stack.items, vec![1, 2, 3, 4, 5, 6, 0, 0, 0, 0]);

        assert!(stack.insert(top, vec![1, 2, 3, 4]));
        top += 4;
        assert_eq!(stack.items, vec![1, 2, 3, 4, 5, 6, 1, 2, 3, 4]);

        assert!(stack.get_top(top, 14).is_none());
    }
}
