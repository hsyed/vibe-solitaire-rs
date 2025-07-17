# Technology Stack

## Core Technologies
- **Language**: Rust (2024 edition)
- **UI Framework**: GPUI (from Zed Industries)
- **Build System**: Cargo
- **Random Number Generation**: `rand` crate (v0.8)

## Dependencies
```toml
[dependencies]
gpui = { git = "https://github.com/zed-industries/zed" }
rand = "0.8"
```

## Common Commands

### Development
```bash
# Build the project
cargo build

# Run the application
cargo run

# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Check for compilation errors
cargo check

# Format code
cargo fmt

# Run clippy linter
cargo clippy
```

### Testing
- Use `cargo test` for running unit tests
- Tests are embedded in source files using `#[cfg(test)]` modules
- Use `println!` in tests for debug output (run with `--nocapture`)

## Code Style
- impl should follow struct strictly -- DO NOT seperate struct declaration and impl.
- Follow standard Rust formatting with `cargo fmt`
- Use `cargo clippy` for linting
- Comprehensive unit tests for all game logic
- Clear documentation comments for public APIs