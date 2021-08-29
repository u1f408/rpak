//! PAK headers

use alloc::string::String;
use alloc::vec::Vec;

use byteorder::{ByteOrder, LittleEndian};

/// The PAK file type identifier - the 4 ASCII characters "PACK"
pub const PAK_IDENTIFIER: [u8; 4] = [b'P', b'A', b'C', b'K'];

/// Length of the PAK file header
pub const PAK_HEADER_LENGTH: u32 = 12;

/// Length of a PAK file table entry
pub const PAK_FILE_LENGTH: u32 = 64;

/// The PAK file header
pub struct PakHeader {
    /// Offset of the file table, from the start of the file
    pub table_offset: u32,

    /// Size of the file table
    pub table_size: u32,
}

impl PakHeader {
    /// Create a new PAK header with the given file table offset and size
    pub fn new(table_offset: u32, table_size: u32) -> Self {
        Self {
            table_offset,
            table_size,
        }
    }

    /// Parse a PAK header from a byte slice.
    ///
    /// Returns `Err(())` if the PAK file header was invalid.
    pub fn from_bytes(data: &[u8]) -> Result<Self, ()> {
        let mut identifier = [0u8; 4];
        identifier.copy_from_slice(&data[0..4]);
        if identifier != PAK_IDENTIFIER {
            return Err(())
        }

        let table_offset = LittleEndian::read_u32(&data[4..]);
        let table_size = LittleEndian::read_u32(&data[8..]);

        Ok(Self {
            table_offset,
            table_size,
        })
    }

    /// Return this PAK header as a byte vector.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::new();
        let mut buf = [0u8; 4];

        // Identifier
        output.extend(&PAK_IDENTIFIER[..]);

        // Table offset
        LittleEndian::write_u32(&mut buf[..], self.table_offset);
        output.extend(&buf[..]);

        // Table size
        LittleEndian::write_u32(&mut buf[..], self.table_size);
        output.extend(&buf[..]);

        output
    }
}

impl Default for PakHeader {
    fn default() -> Self {
        Self {
            table_offset: 0,
            table_size: 0,
        }
    }
}

/// An entry in the PAK file table
pub struct PakFileEntry {
    /// Null-terminated file name.
    pub name_bytes: [u8; 56],

    /// Offset of the file data, from the start of the file.
    pub offset: u32,

    /// Size of the file.
    pub size: u32,
}

impl PakFileEntry {
    /// Create a new file entry with the given file name, offset, and size.
    ///
    /// This function truncates the input filename at 55 characters, the
    /// maximum allowed by the PAK specification.
    pub fn new(file_name: String, offset: u32, size: u32) -> Self {
        let mut out = Self {
            name_bytes: [0u8; 56],
            offset,
            size,
        };

        out.set_name(file_name);
        out
    }

    /// Return the name of the file represented by this file entry.
    pub fn get_name(&self) -> String {
        let mut s: Vec<u8> = Vec::new();
        let mut i: usize = 0;
        while self.name_bytes[i] != 0 {
            s.push(self.name_bytes[i]);
            i += 1;
        }

        String::from_utf8(s).unwrap_or(String::from("unknown.bin"))
    }
    
    /// Set the name of the file.
    ///
    /// This function truncates the input filename at 55 characters, the
    /// maximum allowed by the PAK specification.
    pub fn set_name(&mut self, file_name: String) {
        self.name_bytes = [0u8; 56];

        // Copy file name bytes, truncating at 55 bytes (allowing the 56th
        // to be the null terminator)
        for (i, b) in (0..).zip(file_name.as_bytes().iter()) {
            if i < 55 {
                self.name_bytes[i] = *b;
            }
        }
    }

    /// Parse a file entry from a byte slice.
    pub fn from_bytes(data: &[u8]) -> Self {
        let mut name_bytes = [0u8; 56];
        name_bytes.copy_from_slice(&data[0..56]);

        let offset = LittleEndian::read_u32(&data[56..]);
        let size = LittleEndian::read_u32(&data[60..]);

        Self {
            name_bytes,
            offset,
            size,
        }
    }

    /// Return this file entry as a byte vector.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::new();
        let mut buf = [0u8; 4];

        // File name
        output.extend(&self.name_bytes[..]);

        // File offset
        LittleEndian::write_u32(&mut buf[..], self.offset);
        output.extend(&buf[..]);

        // File size
        LittleEndian::write_u32(&mut buf[..], self.size);
        output.extend(&buf[..]);

        output
    }
}
