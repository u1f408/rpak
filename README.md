# rpak

Simple no\_std-compatible helper for the Quake 2 PAK format.

## Library usage

Add to your `Cargo.toml`:

```toml
rpak = "0.2"
```

The "std" feature is enabled by default, for no\_std environments (only where
`alloc` is available), use:

```toml
rpak = { version = "0.2", default-features = false }
```

To load an archive:

```rust
use rpak::PakArchive;

let mut data: Vec<u8> = Vec::new();
let _ = File::open("data.pak")?.read_to_end(&mut data)?;

let archive = PakArchive::from_bytes(&data[..])?;
assert!(archive.files.len() > 0);
```

## License

MIT
