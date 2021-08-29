use rpak::{PakHeader, PakFileEntry, PakArchive};

const EXPECTED_SINGLE: &[u8] = include_bytes!("test_data/test_single.pak");
const EXPECTED_MULTIPLE: &[u8] = include_bytes!("test_data/test_multiple.pak");
const EXPECTED_HEADER: &[u8] = include_bytes!("test_data/header.bin");
const EXPECTED_FILE_ENTRY: &[u8] = include_bytes!("test_data/file_entry.bin");

#[test]
fn archive_create_single_file() {
    let mut archive = PakArchive::new();
    archive.add_file(String::from("test.txt"), b"hello world!\n");
    let output = archive.as_bytes();

    assert_eq!(output.len(), EXPECTED_SINGLE.len());
    assert_eq!(output[..], EXPECTED_SINGLE[..]);
}

#[test]
fn archive_create_multiple_files() {
    let mut archive = PakArchive::new();
    archive.add_file(String::from("1.txt"), b"hello world!\n");
    archive.add_file(String::from("2.txt"), b"hello world!\n");
    archive.add_file(String::from("3.txt"), b"hello world!\n");
    let output = archive.as_bytes();

    assert_eq!(output.len(), EXPECTED_MULTIPLE.len());
    assert_eq!(output[..], EXPECTED_MULTIPLE[..]);
}

#[test]
fn archive_read() {
    let archive = PakArchive::from_bytes(&EXPECTED_MULTIPLE[..]).unwrap();

    assert_eq!(archive.files.len(), 3);
    assert_eq!(archive.files[0].0, String::from("1.txt"));
    assert_eq!(archive.files[1].0, String::from("2.txt"));
    assert_eq!(archive.files[2].0, String::from("3.txt"));
}

#[test]
fn header_as_bytes() {
    let header = PakHeader::new(64, 64);
    let output = header.as_bytes();

    assert_eq!(output.len(), EXPECTED_HEADER.len());
    assert_eq!(output[..], EXPECTED_HEADER[..]);
}

#[test]
fn header_from_bytes() {
    let header = PakHeader::from_bytes(&EXPECTED_HEADER[..]).unwrap();

    assert_eq!(header.table_offset, 64);
    assert_eq!(header.table_size, 64);
}

#[test]
fn file_entry_as_bytes() {
    let file_entry = PakFileEntry::new(String::from("test.txt"), 64, 8);
    let output = file_entry.as_bytes();

    assert_eq!(output.len(), EXPECTED_FILE_ENTRY.len());
    assert_eq!(output[..], EXPECTED_FILE_ENTRY[..]);
}

#[test]
fn file_entry_from_bytes() {
    let file_entry = PakFileEntry::from_bytes(&EXPECTED_FILE_ENTRY[..]);

    assert_eq!(file_entry.get_name(), String::from("test.txt"));
    assert_eq!(file_entry.offset, 64);
    assert_eq!(file_entry.size, 8);
}
