# typeshare
Generate code in different languages from Rust type definitions for FFI interop.

WARNING: This utility is still in early development and not ready for production use.

## Usage

```
cargo install typeshare
typeshare --type=ts some/file.rs
typeshare --type=swift some/file.rs
```