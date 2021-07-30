use alloc::vec::Vec;
use digest::Digest;

use crate::error::{LoaderError, ValidatorError};
use crate::validator::Validator;
use voxl_instruction_set::instruction::Instruction;
use voxl_instruction_set::vxl_file::VXLHeader;

#[derive(Clone, PartialEq, Debug)]
pub struct Loader {
    header: VXLHeader,
    program_bytes: Vec<u8>,
}

impl Loader {
    pub fn load_bytes(bytes: &[u8]) -> Result<Self, LoaderError> {
        if bytes.len() < VXLHeader::HEADER_SIZE {
            return Err(LoaderError::NotEnoughBytesForHeader);
        }

        // Check the magic
        for i in 0..VXLHeader::MAGIC.len() {
            if bytes[i] != VXLHeader::MAGIC[i] {
                return Err(LoaderError::InvalidMagic);
            }
        }

        let mut current_index = 4;

        let version = bytes[current_index];
        let file_size;
        let starting_offset;
        let flags;
        let mut checksum = [0u8; VXLHeader::HEADER_CHECKSUM_SIZE];

        if !VXLHeader::SUPPORTED_VERSIONS.contains(&version) {
            return Err(LoaderError::UnsupportedVersion);
        }

        current_index += 1;

        let mut working_bytes = [0u8; 8];

        for i in 0..8 {
            working_bytes[i] = bytes[current_index];
            current_index += 1;
        }

        file_size = u64::from_le_bytes(working_bytes);

        for i in 0..8 {
            working_bytes[i] = bytes[current_index];
            current_index += 1;
        }

        starting_offset = u64::from_le_bytes(working_bytes);

        flags = bytes[current_index];
        current_index += 1;

        for i in 0..VXLHeader::HEADER_CHECKSUM_SIZE {
            checksum[i] = bytes[current_index];
            current_index += 1;
        }

        // Check the trailing byte
        if bytes[current_index] != VXLHeader::END_HEADER_BYTE {
            return Err(LoaderError::InvalidEndHeaderMarker);
        }

        current_index += 1;

        let program_bytes = bytes[current_index..].to_vec();

        if program_bytes.len() as u64 != file_size {
            return Err(LoaderError::NonMatchingFileSize);
        }

        let ldr = Self {
            header: VXLHeader::new(version, file_size, starting_offset, flags, checksum),
            program_bytes,
        };

        return Ok(ldr);
    }

    pub fn validate(&self) -> Result<(), LoaderError> {
        if self.header.flags() & VXLHeader::CHECKSUM_MASK == 0 {
            return self.verify_checksum(sha2::Sha224::new());
        } else {
            return self.verify_checksum(sha3::Sha3_224::new());
        }
    }

    fn verify_checksum<D: Digest>(&self, mut digest: D) -> Result<(), LoaderError> {
        digest.update(self.program_bytes.clone());
        let output = digest.finalize();

        if output.len() != self.header.checksum().len() {
            return Err(LoaderError::InvalidChecksum);
        }

        for i in 0..output.len() {
            if output[i] != self.header.checksum()[i] {
                return Err(LoaderError::InvalidChecksum);
            }
        }

        return Ok(());
    }

    pub fn get_header(&self) -> VXLHeader {
        return self.header;
    }

    pub fn to_program_bytes(self) -> Vec<u8> {
        return self.program_bytes;
    }

    pub fn to_instructions<V: Validator>(
        self,
        mut validator: V,
    ) -> Result<(VXLHeader, Vec<Instruction>), ValidatorError> {
        validator.append_bytes(self.program_bytes);

        let instructions = validator.process_all_instructions()?;

        return Ok((self.header, instructions));
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;
    use paste::paste;

    use super::*;

    macro_rules! test_validation {
        ($reference:expr, $program_bytes:expr, $flags:literal, $out:expr, $name:ident) => {
            paste! {
                #[test]
                fn [<test_ $name>]() {
                    let mut checksum = [0u8; 28];

                    let hex_reference =
                        hex::decode($reference).unwrap();

                    if hex_reference.len() != checksum.len() {
                        panic!("Different length for reference to expected.");
                    }

                    for i in 0..hex_reference.len() {
                        checksum[i] = hex_reference[i];
                    }

                    let loader = Loader {
                        header: VXLHeader::new(0x0, $program_bytes.len() as u64, 0, $flags, checksum),
                        program_bytes: $program_bytes,
                    };

                    assert_eq!(loader.validate(), $out);
                }
            }
        };
    }
    /*
    Byte Offset - Value

    0x0 - Magic bytes (0x65, 0x58, 0x56, 0x4c)
    0x4 - Executable version
    0x5 - File size in bytes excluding header (Little endian)
    0xd - Starting instruction offset (Little endian)
    0x15 - Flags (From LSB to MSB 0 = Hash algorithm (1 = SHA3-224, 0 = SHA2-224))
    0x16 - Checksum (SHA3 or SHA2 hash of the expected file. (28 bytes))
    0x32 - End header byte (0xaa)
    */
    macro_rules! test_load_header {
        ([$($magic_bytes:literal),+], [$($checksum:literal),+], [$($program_size:literal),+], [$($program:literal),+], $flags:literal, $header:expr, $name:ident) => {
            paste! {
                #[test]
                fn [<test_ $name>]() {
                    let bytes = [
                        // Magic bytes
                        $($magic_bytes,)*
                        // Executable version
                        0x0,
                        // File size in bytes
                        $($program_size,)*
                        // Starting Instruction Offset
                        0x0, 0x0, 0x0, 0x0,
                        0x0, 0x0, 0x0, 0x0,
                        // flags
                        $flags,
                        // Checksum
                        $($checksum,)*
                        // End header byte
                        0xaa,
                        // Program
                        $($program,)*
                    ];

                    let loader = Loader::load_bytes(&bytes).unwrap();
                    let header = loader.get_header();

                    assert_eq!(header, $header);
                }
            }
        };
        ([$($magic_bytes:literal),+], [$($checksum:literal),+], [$($program_size:literal),+], [$($program:literal),+], $flags:literal, out $out:expr, $name:ident) => {
            test_load_header!([$($magic_bytes),+], [$($checksum),+], [$($program_size),+], [$($program),+], $flags, 0xaa, out $out, $name);
        };
        ([$($magic_bytes:literal),+], [$($checksum:literal),+], [$($program_size:literal),+], [$($program:literal),+], $flags:literal, $end_header:literal, out $out:expr, $name:ident) => {
            paste! {
                #[test]
                fn [<test_ $name>]() {
                    let bytes = [
                        // Magic bytes
                        $($magic_bytes,)*
                        // Executable version
                        0x0,
                        // File size in bytes
                        $($program_size,)*
                        // Starting Instruction Offset
                        0x0, 0x0, 0x0, 0x0,
                        0x0, 0x0, 0x0, 0x0,
                        // flags
                        $flags,
                        // Checksum
                        $($checksum,)*
                        // End header byte
                        $end_header,
                        // Program
                        $($program,)*
                    ];

                    assert_eq!(Loader::load_bytes(&bytes), $out);
                }
            }
        };
    }

    test_validation!(
        "469936336c0622ceb461a74d03480610093471b22be11ec9835f1072",
        vec![0x0f, 0x0f, 0x0f, 0xff],
        0b0000_0001,
        Ok(()),
        validate_sha3
    );

    test_validation!(
        "469936336c0622ceb461a74d03480610093471b22be11ec9835f1072",
        vec![0x0f, 0x0f, 0x0f, 0xfd],
        0b0000_0001,
        Err(LoaderError::InvalidChecksum),
        validate_sha3_invalid
    );

    test_validation!(
        "469936336c0622ceb461a74d03480610093471b22be11ec9835f1071",
        vec![0x0f, 0x0f, 0x0f, 0xff],
        0b0000_0001,
        Err(LoaderError::InvalidChecksum),
        validate_sha3_invalid_2
    );

    test_validation!(
        "37f5c1212897602d16d17b8bcf5d926fd5aab286f25fe1d019288e99",
        vec![0x0f, 0x0f, 0x0f, 0xff],
        0b0000_0000,
        Ok(()),
        validate_sha2
    );

    test_validation!(
        "37f5c1212897602d16d17b8bcf5d926fd5aab286f25fe1d019288e99",
        vec![0x0f, 0x0f, 0x0f, 0xfd],
        0b0000_0000,
        Err(LoaderError::InvalidChecksum),
        validate_sha2_invalid
    );

    test_validation!(
        "37f5c1212897602d16d17b8bcf5d926fd5aab286f25fe1d019288e98",
        vec![0x0f, 0x0f, 0x0f, 0xff],
        0b0000_0000,
        Err(LoaderError::InvalidChecksum),
        validate_sha2_invalid_2
    );

    test_load_header!(
        [0x65, 0x58, 0x56, 0x4c],
        [
            0x6, 0x99, 0x36, 0x33, 0x6c, 0x6, 0x22, 0xce, 0xb4, 0x61, 0xa7, 0x4d, 0x3, 0x48, 0x6,
            0x10, 0x9, 0x34, 0x71, 0xb2, 0x2b, 0xe1, 0x1e, 0xc9, 0x83, 0x5f, 0x10, 0x72
        ],
        [0x4, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
        [0x1, 0x2, 0x3, 0x4],
        0b0000_0001,
        VXLHeader::new(
            0,
            4,
            0,
            1,
            [
                0x6, 0x99, 0x36, 0x33, 0x6c, 0x6, 0x22, 0xce, 0xb4, 0x61, 0xa7, 0x4d, 0x3, 0x48,
                0x6, 0x10, 0x9, 0x34, 0x71, 0xb2, 0x2b, 0xe1, 0x1e, 0xc9, 0x83, 0x5f, 0x10, 0x72
            ],
        ),
        load_valid_header
    );

    test_load_header!(
        [0x65, 0x58, 0x56, 0x4c],
        [
            0x6, 0x99, 0x36, 0x33, 0x6c, 0x6, 0x22, 0xce, 0xb4, 0x61, 0xa7, 0x4d, 0x3, 0x48, 0x6,
            0x10, 0x9, 0x34, 0x71, 0xb2, 0x2b, 0xe1, 0x1e, 0xc9, 0x83, 0x5f, 0x10, 0x72
        ],
        [0x4, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
        [0x1, 0x2, 0x3, 0x3],
        0b0000_0001,
        VXLHeader::new(
            0,
            4,
            0,
            1,
            [
                0x6, 0x99, 0x36, 0x33, 0x6c, 0x6, 0x22, 0xce, 0xb4, 0x61, 0xa7, 0x4d, 0x3, 0x48,
                0x6, 0x10, 0x9, 0x34, 0x71, 0xb2, 0x2b, 0xe1, 0x1e, 0xc9, 0x83, 0x5f, 0x10, 0x72
            ],
        ),
        load_valid_header_2
    );

    // Test magic
    test_load_header!(
        [0x00, 0x58, 0x56, 0x4c],
        [
            0x6, 0x99, 0x36, 0x33, 0x6c, 0x6, 0x22, 0xce, 0xb4, 0x61, 0xa7, 0x4d, 0x3, 0x48, 0x6,
            0x10, 0x9, 0x34, 0x71, 0xb2, 0x2b, 0xe1, 0x1e, 0xc9, 0x83, 0x5f, 0x10, 0x72
        ],
        [0x4, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
        [0x1, 0x2, 0x3, 0x3],
        0b0000_0001,
        out Err(LoaderError::InvalidMagic),
        load_invalid_header_magic_1
    );

    test_load_header!(
        [0x65, 0x59, 0x56, 0x4c],
        [
            0x6, 0x99, 0x36, 0x33, 0x6c, 0x6, 0x22, 0xce, 0xb4, 0x61, 0xa7, 0x4d, 0x3, 0x48, 0x6,
            0x10, 0x9, 0x34, 0x71, 0xb2, 0x2b, 0xe1, 0x1e, 0xc9, 0x83, 0x5f, 0x10, 0x72
        ],
        [0x4, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
        [0x1, 0x2, 0x3, 0x3],
        0b0000_0001,
        out Err(LoaderError::InvalidMagic),
        load_invalid_header_magic_2
    );

    test_load_header!(
        [0x65, 0x58, 0x57, 0x4c],
        [
            0x6, 0x99, 0x36, 0x33, 0x6c, 0x6, 0x22, 0xce, 0xb4, 0x61, 0xa7, 0x4d, 0x3, 0x48, 0x6,
            0x10, 0x9, 0x34, 0x71, 0xb2, 0x2b, 0xe1, 0x1e, 0xc9, 0x83, 0x5f, 0x10, 0x72
        ],
        [0x4, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
        [0x1, 0x2, 0x3, 0x3],
        0b0000_0001,
        out Err(LoaderError::InvalidMagic),
        load_invalid_header_magic_3
    );

    test_load_header!(
        [0x65, 0x58, 0x56, 0x4b],
        [
            0x6, 0x99, 0x36, 0x33, 0x6c, 0x6, 0x22, 0xce, 0xb4, 0x61, 0xa7, 0x4d, 0x3, 0x48, 0x6,
            0x10, 0x9, 0x34, 0x71, 0xb2, 0x2b, 0xe1, 0x1e, 0xc9, 0x83, 0x5f, 0x10, 0x72
        ],
        [0x4, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
        [0x1, 0x2, 0x3, 0x3],
        0b0000_0001,
        out Err(LoaderError::InvalidMagic),
        load_invalid_header_magic_4
    );

    test_load_header!(
        [0x65, 0x58, 0x56, 0x4c],
        [
            0x6, 0x99, 0x36, 0x33, 0x6c, 0x6, 0x22, 0xce, 0xb4, 0x61, 0xa7, 0x4d, 0x3, 0x48, 0x6,
            0x10, 0x9, 0x34, 0x71, 0xb2, 0x2b, 0xe1, 0x1e, 0xc9, 0x83, 0x5f, 0x10, 0x72
        ],
        [0x4, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
        [0x1, 0x2, 0x3, 0x3],
        0b0000_0001,
        0xab,
        out Err(LoaderError::InvalidEndHeaderMarker),
        load_invalid_header_terminator
    );
}
