# NAS (Non-Access Spectrum) Helper Library Commands

## Build, Test, Lint Commands
- Build: `cargo build`
- Run tests: `cargo test`
- Run single test: `cargo test test_snow_3g_f8`
- Check compilation: `cargo check`
- Format code: `cargo fmt`
- Lint: `cargo clippy`

## Code Style Guidelines
- Use Rust 2021 edition conventions
- Follow standard Rust naming conventions (snake_case for functions/variables, CamelCase for types)
- Use the `tlv` and `tlv-derive` macros for TLV encoding/decoding
- Use the `bitfield` macros for bit field implementations
- Impl `Debug` for all types
- Use `Option<T>` for optional fields
- Auto-generated files should not be modified directly
- Use `unsafe` only when necessary (e.g., for FFI)

## Error Handling
- Use Result<T, E> for functions that can fail
- Propagate errors upward when appropriate
- Document errors with clear messages