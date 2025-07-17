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

## GPUI Framework Notes

note: this is added to aid agents in targetting gpui which they are not too familiar with. 

### Event Handling Patterns
- **Correct Pattern**: Use `cx.listener()` for event handlers that need to update app state
  ```rust
  .on_mouse_down(gpui::MouseButton::Left, cx.listener(|app, _event, _window, cx| {
      app.handle_action(SomeAction, cx);
  }))
  ```
- **Closure Signature**: `cx.listener()` expects 4 parameters: `(app, event, window, context)`
- **Avoid**: Direct closures in event handlers - they can't access app state
- **Avoid**: `cx.handle()`, `cx.view()`, `cx.weak_handle()` - these methods don't exist

### UI Component Patterns
- **Interactive Elements**: Must import `InteractiveElement` trait for `.hover()`, `.cursor_pointer()`
- **Hover States**: Use `.hover(|style| style.property(value))` for visual feedback
- **Mouse Events**: Use `.on_mouse_down(MouseButton::Left, handler)` for click handling

### Context and State Management
- **State Updates**: Use `cx.notify()` to trigger re-renders after state changes
- **Method Parameters**: Event handlers receive `&mut Context<Self>`, not `&mut Window`
- **Component Communication**: Pass context through method parameters when needed

### Common Pitfalls to Avoid
- Don't try to capture `cx` in closures - use `cx.listener()` instead
- Don't use `update()` on focus handles - use proper listener pattern
- Don't forget to import required traits (`InteractiveElement`, etc.)
- Event handler closures must match expected parameter counts exactly

### Imports Needed for Interactive UI
```rust
use gpui::{
    InteractiveElement, // For .hover(), .cursor_pointer()
    MouseButton,        // For mouse event handling
    // ... other imports
};
```