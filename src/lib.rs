//! Simple no\_std-compatible helper for the Quake 2 PAK format.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

use alloc::string::String;
use alloc::vec::Vec;

mod header;
pub use self::header::*;

/// An in-memory representation of a PAK archive
pub struct PakArchive<'a> {
    /// Files contained within this archive
    pub files: Vec<(String, &'a [u8])>,
}

impl<'a> PakArchive<'a> {
    /// Create a new empty PAK archive.
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
        }
    }

    /// Add a file to the archive with the given file name, containing the
    /// given data.
    pub fn add_file(&mut self, file_name: String, file_data: &'a [u8]) { 
        self.files.push((file_name, file_data));
    }

    /// Decode a PAK archive from an input byte slice.
    ///
    /// Returns `Err(())` if the PAK file header was invalid.
    pub fn from_bytes(data: &'a [u8]) -> Result<Self, ()> {
        let header = PakHeader::from_bytes(data)?;

        let mut files: Vec<(String, &'a [u8])> = Vec::new();
        let mut offset: usize = header.table_offset as usize;
        while offset < data.len() {
            let file_entry = PakFileEntry::from_bytes(&data[offset..]);
            let file_offset = file_entry.offset as usize;
            let file_size = file_entry.size as usize;

            let file_data = &data[file_offset..(file_offset + file_size)];
            files.push((file_entry.get_name(), file_data));

            offset += PAK_FILE_LENGTH as usize;
        }

        Ok(Self {
            files,
        })
    }

    /// Encode this archive representation into a byte vector.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut offset: u32 = PAK_HEADER_LENGTH;
        let mut output: Vec<u8> = Vec::new();

        let mut file_table: Vec<PakFileEntry> = Vec::new();
        for (fname, fdata) in self.files.iter() {
            let flength: u32 = (fdata.len() & 0xFFFFFFFF) as u32;

            // Create file table entry
            file_table.push(PakFileEntry::new(fname.clone(), offset, flength));

            // Append file data to output
            output.extend(&fdata[..]);

            // Adjust offset
            offset += flength;
        }

        // Construct file table
        let ftable_offset = offset;
        for ftable_entry in file_table.iter() {
            let tdata = ftable_entry.as_bytes();
            output.extend(&tdata[..]);
            offset += (tdata.len() & 0xFFFFFFFF) as u32
        }

        // Construct header and return
        let header = PakHeader::new(ftable_offset, offset - ftable_offset);
        Vec::from([header.as_bytes(), output].concat())
    }
}
