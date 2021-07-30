use alloc::string::{String, ToString};
use alloc::format;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum LoaderError {
    NotEnoughBytesForHeader,
    UnsupportedVersion,
    InvalidChecksum,
    NonMatchingFileSize,
    InvalidMagic,
    InvalidEndHeaderMarker,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum ValidatorError {
    UnexpectedEndOfBytes,
    UnknownRegisterCountForOpcode,
    UnknownImmediateCountForOpcode,
    UnknownAddressCountForOpcode,
    InvalidInstructionFormat,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum VMError {
    SystemHalted,
    NoInstruction,
    AccessBeyondStackBounds,
    FailedMalloc,
    FailedFreeNoAddressError(u64),
    FailedSetNoAddressError(u64),
    FailedGetNoAddressError(u64),
    IndexBeyondBoundsError(u64, u64), // index, bound
    IntegerOverflowError,
    UnsignedIntegerOverflowError,
    FloatOverflowError,
    AttemptedModuloZeroOperation,
    UnknownSystemCall(u64),
    Unknown(String),
}

pub trait VXLVMError {
    fn specific_description(&self) -> String;

    fn short_description(&self) -> String;
}

impl LoaderError {
    pub fn as_u8(&self) -> u8 {
        return match self {
            LoaderError::NotEnoughBytesForHeader => 0,
            LoaderError::UnsupportedVersion => 1,
            LoaderError::InvalidChecksum => 2,
            LoaderError::NonMatchingFileSize => 3,
            LoaderError::InvalidMagic => 4,
            LoaderError::InvalidEndHeaderMarker => 5,
        };
    }
}

impl ValidatorError {
    pub fn as_u8(&self) -> u8 {
        return match self {
            ValidatorError::UnexpectedEndOfBytes => 0,
            ValidatorError::UnknownRegisterCountForOpcode => 1,
            ValidatorError::UnknownImmediateCountForOpcode => 2,
            ValidatorError::UnknownAddressCountForOpcode => 3,
            ValidatorError::InvalidInstructionFormat => 4,
        };
    }
}

impl VMError {
    pub fn as_u8(&self) -> u8 {
        return match self {
            VMError::SystemHalted => 0,
            VMError::NoInstruction => 1,
            VMError::AccessBeyondStackBounds => 2,
            VMError::FailedMalloc => 3,
            VMError::FailedFreeNoAddressError(_) => 4,
            VMError::FailedSetNoAddressError(_) => 5,
            VMError::FailedGetNoAddressError(_) => 6,
            VMError::IndexBeyondBoundsError(_, _) => 7,
            VMError::IntegerOverflowError => 8,
            VMError::UnsignedIntegerOverflowError => 9,
            VMError::FloatOverflowError => 10,
            VMError::AttemptedModuloZeroOperation => 11,
            VMError::UnknownSystemCall(_) => 12,
            VMError::Unknown(_) => u8::MAX,
        };
    }
}

impl VXLVMError for LoaderError {
    fn specific_description(&self) -> String {
        return match self {
            LoaderError::NotEnoughBytesForHeader => "The supplied input does not contain enough bytes for the header.",
            LoaderError::UnsupportedVersion => "This file contains code for an unsupported voxeol version.",
            LoaderError::InvalidChecksum => "This file's checksum is invalid.",
            LoaderError::NonMatchingFileSize => "The expected file size does not match the actual size.",
            LoaderError::InvalidMagic => "This file is not of the executable-voxeol format.",
            LoaderError::InvalidEndHeaderMarker => "Could not find the end-header marker.",
        }.to_string();
    }

    fn short_description(&self) -> String {
        return format!("Loader Error: {}", self.as_u8());
    }
}

impl VXLVMError for ValidatorError {
    fn specific_description(&self) -> String {
        return match self {
            ValidatorError::UnexpectedEndOfBytes => "Unexpectedly ran out of bytes to validate.",
            ValidatorError::UnknownRegisterCountForOpcode => "Unknown register count for opcode.",
            ValidatorError::UnknownImmediateCountForOpcode => "Unknown immediate count for opcode.",
            ValidatorError::UnknownAddressCountForOpcode => "Unknown address count for opcode.",
            ValidatorError::InvalidInstructionFormat => "Invalid instruction format",
        }.to_string();
    }

    fn short_description(&self) -> String {
        return format!("Validator Error: {}", self.as_u8());
    }
}

impl VXLVMError for VMError {
    fn specific_description(&self) -> String {
        return match self {
            VMError::SystemHalted => "System is halted.".to_string(),
            VMError::NoInstruction => "No instruction to execute.".to_string(),
            VMError::AccessBeyondStackBounds => "Attempt to access beyond stack bounds".to_string(),
            VMError::FailedMalloc => "Failed to allocate memory".to_string(),
            VMError::FailedFreeNoAddressError(a) => format!("Failed to free. No address {}.", a),
            VMError::FailedSetNoAddressError(a) => format!("Failed to set memory. No address {}.", a),
            VMError::FailedGetNoAddressError(a) => format!("Failed to get from memory. No address {}.", a),
            VMError::IndexBeyondBoundsError(i, b) => format!("Index: {}, is beyond the bounds of {}.", i, b),
            VMError::IntegerOverflowError => "Integer overflow.".to_string(),
            VMError::UnsignedIntegerOverflowError => "Unsigned integer overflow".to_string(),
            VMError::FloatOverflowError => "Float overflow.".to_string(),
            VMError::AttemptedModuloZeroOperation => "Attempted a divide by 0 operation".to_string(),
            VMError::UnknownSystemCall(c) => format!("Unknown system call {}", c),
            VMError::Unknown(s) => s.clone(),
        };
    }

    fn short_description(&self) -> String {
        return format!("Machine Error: {}", self.as_u8());
    }
}