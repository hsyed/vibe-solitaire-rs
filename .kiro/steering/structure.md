# Project Structure

## Directory Layout
```
src/
├── main.rs              # Application entry point with GPUI setup
├── game/                # Core game logic (no UI dependencies)
│   ├── mod.rs          # Game module exports
│   ├── deck.rs         # Card, Suit, Rank definitions and deck creation
│   ├── state.rs        # GameState struct and game state management
│   └── actions.rs      # GameAction enum and move definitions
└── ui/                 # UI components and rendering
    └── mod.rs          # UI module (components to be implemented)
```

## Architecture Patterns

### Separation of Concerns
- **Game Logic** (`src/game/`): Pure Rust logic with no UI dependencies
- **UI Layer** (`src/ui/`): GPUI components that render game state
- **Main App** (`src/main.rs`): Connects UI to game logic

### Key Structs and Enums
- `Card`: Represents playing cards with suit, rank, and face state
- `GameState`: Complete game state including tableau, foundations, stock, waste
- `Position`: Enum for card locations (Tableau, Foundation, Stock, Waste)
- `GameAction`: Enum for all possible game moves
- `DrawCount`: One or Three card draw modes

### Testing Strategy
- Unit tests embedded in each module using `#[cfg(test)]`
- Comprehensive test coverage for game logic
- Debug output methods for development (`debug_info()`, `summary()`)

### Code Organization
- Each module has clear responsibilities
- Public APIs are well-documented
- Game state is immutable where possible
- Actions are represented as data structures