# Solitaire Game Design Document

## Overview

This design document outlines the architecture for a Klondike Solitaire game built with Rust, GPUI framework, and gpui-component crates. The application will be a desktop game featuring drag-and-drop interactions, smooth animations, and a clean user interface following modern design principles.

GPUI is a modern UI framework for Rust that provides reactive components, efficient rendering, and built-in support for animations and interactions. The gpui-component crate extends GPUI with additional reusable UI components that will help accelerate development.

## Architecture

The application follows a component-based architecture with clear separation of concerns:

```
┌─────────────────────────────────────────┐
│              App Component              │
├─────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────────┐   │
│  │   Header    │  │   Game Board    │   │
│  │ Component   │  │   Component     │   │
│  └─────────────┘  └─────────────────┘   │
│                   ┌─────────────────┐   │
│                   │   Statistics    │   │
│                   │   Component     │   │
│                   └─────────────────┘   │
└─────────────────────────────────────────┘
```

### Core Modules

1. **Game State Module** - Manages the solitaire game logic and state
2. **Card Module** - Defines card data structures and operations
3. **UI Components Module** - GPUI components for rendering game elements
4. **Interaction Module** - Handles drag-and-drop and click interactions
5. **Animation Module** - Manages card movement and flip animations
6. **Statistics Module** - Tracks game statistics and persistence

## Components and Interfaces

### Game State Management

```rust
pub struct GameState {
    pub tableau: [Vec<Card>; 7],
    pub foundations: [Vec<Card>; 4],
    pub stock: Vec<Card>,
    pub waste: Vec<Card>,
    pub move_count: u32,
    pub start_time: SystemTime,
    pub game_won: bool,
    pub draw_count: DrawCount,
}

pub enum DrawCount {
    One,   // Deal 1 card at a time from stock (easier)
    Three, // Deal 3 cards at a time from stock (harder)
}

pub enum GameAction {
    MoveCard { from: Position, to: Position },
    FlipCard(Position),
    DealFromStock,
    NewGame,
    Undo,
}
```

### Card System

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
    pub face_up: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Rank {
    Ace = 1,
    Two = 2,
    // ... through King = 13
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Position {
    Tableau(usize, usize),
    Foundation(usize),
    Stock,
    Waste(usize),
}
```

### GPUI Components

#### Main App Component
```rust
pub struct SolitaireApp {
    game_state: Model<GameState>,    // Reactive state container for game data
    statistics: Model<Statistics>,   // Reactive state container for stats data
    drag_state: Option<DragState>,   // Local drag state (not shared)
}

impl Render for SolitaireApp {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .child(self.render_header(cx))
            .child(self.render_game_board(cx))
            .child(self.render_statistics(cx))
    }
}
```

#### Card Component
```rust
pub struct CardComponent {
    card: Card,
    position: Position,
    draggable: bool,
    highlighted: bool,
}

impl Render for CardComponent {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size(px(CARD_WIDTH), px(CARD_HEIGHT))
            .bg(if self.card.face_up { white() } else { blue() })
            .border_1()
            .border_color(black())
            .rounded_md()
            .when(self.draggable, |div| {
                div.cursor_pointer()
                    .on_drag(|drag, cx| {
                        // Handle drag start
                    })
            })
            .child(self.render_card_content())
    }
}
```

#### Game Board Component
```rust
pub struct GameBoard {
    game_state: Model<GameState>,
}

impl GameBoard {
    fn render_tableau(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .gap_4()
            .children((0..7).map(|col| self.render_tableau_column(col, cx)))
    }

    fn render_foundations(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .gap_4()
            .children((0..4).map(|foundation| self.render_foundation_pile(foundation, cx)))
    }
}
```

### Drag and Drop System

```rust
pub struct DragState {
    pub dragged_cards: Vec<Card>,
    pub source_position: Position,
    pub current_mouse_position: Point<Pixels>,
    pub valid_drop_targets: Vec<Position>,
}

pub trait DropTarget {
    fn can_accept(&self, cards: &[Card]) -> bool;
    fn accept_drop(&mut self, cards: Vec<Card>) -> Result<(), String>;
    fn get_drop_position(&self) -> Position;
}
```

## Data Models

### Game Statistics
```rust
pub struct Statistics {
    pub games_played: u32,
    pub games_won: u32,
    pub best_time: Option<Duration>,
    pub average_moves: f32,
    pub current_streak: u32,
    pub best_streak: u32,
}

impl Statistics {
    pub fn win_percentage(&self) -> f32 {
        if self.games_played == 0 {
            0.0
        } else {
            (self.games_won as f32 / self.games_played as f32) * 100.0
        }
    }
}
```

### Move History for Undo
```rust
pub struct Move {
    pub action: GameAction,
    pub previous_state: GameStateSnapshot,
    pub timestamp: SystemTime,
}

pub struct GameStateSnapshot {
    pub tableau: [Vec<Card>; 7],
    pub foundations: [Vec<Card>; 4],
    pub stock: Vec<Card>,
    pub waste: Vec<Card>,
    pub move_count: u32,
}
```

## Error Handling

### Game Rule Validation
```rust
pub enum GameError {
    InvalidMove {
        from: Position,
        to: Position,
        reason: String,
    },
    CardNotFound(Position),
    EmptyPile(Position),
    GameAlreadyWon,
    NoMovesToUndo,
}

impl std::fmt::Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameError::InvalidMove { from, to, reason } => {
                write!(f, "Invalid move from {:?} to {:?}: {}", from, to, reason)
            }
            // ... other error cases
        }
    }
}
```

### UI Error Handling
- Invalid moves will show visual feedback (red highlight, shake animation)
- System errors will display user-friendly messages
- Game state corruption will trigger automatic recovery or new game prompt

## Testing Strategy

### Unit Tests
- **Card Logic Tests**: Validate card creation, comparison, and utility functions
- **Game Rules Tests**: Test move validation for all game rules
- **State Management Tests**: Verify game state transitions and consistency
- **Statistics Tests**: Ensure accurate tracking and calculation of game statistics

### Integration Tests
- **Component Interaction Tests**: Test communication between GPUI components
- **Drag and Drop Tests**: Validate complete drag-and-drop workflows
- **Game Flow Tests**: Test complete game scenarios from start to finish
- **Persistence Tests**: Verify statistics saving and loading

### UI Tests
- **Rendering Tests**: Ensure components render correctly in various states
- **Animation Tests**: Verify smooth animations and transitions
- **Responsive Tests**: Test UI behavior with different window sizes
- **Accessibility Tests**: Ensure keyboard navigation and screen reader support

### Performance Tests
- **Rendering Performance**: Measure frame rates during animations
- **Memory Usage**: Monitor memory consumption during extended play
- **Startup Time**: Ensure fast application launch

## Animation and Visual Design

### Card Animations
- **Move Animation**: Smooth bezier curve movement between positions (300ms duration)
- **Flip Animation**: 3D flip effect when revealing cards (200ms duration)
- **Deal Animation**: Staggered dealing from stock to waste (100ms per card)
- **Win Animation**: Cascading foundation cards celebration effect

### Visual Feedback
- **Hover States**: Subtle highlight on interactive elements
- **Drop Zones**: Visual indicators for valid drop targets during drag
- **Invalid Moves**: Red flash and shake animation for rejected moves
- **Loading States**: Smooth transitions during game initialization

### Color Scheme
- **Background**: Soft green felt texture (#0F5132)
- **Cards**: Clean white background with black borders
- **Red Suits**: Classic red (#DC3545) for hearts and diamonds
- **Black Suits**: Pure black (#000000) for clubs and spades
- **Highlights**: Gold accent (#FFD700) for selected/highlighted elements

## GPUI Framework Glossary

### Core GPUI Types and Concepts

#### State Management
- **`Model<T>`**: A reactive state container that holds shared application data. When the data inside changes, all UI components that depend on it automatically re-render. Think of it like a "smart" shared reference.
- **`ViewContext<T>`**: Provides access to the current view's state and allows interaction with the GPUI system (scheduling updates, handling events, etc.)

#### UI Components
- **`Render` trait**: The main trait that UI components implement. The `render()` method returns the visual representation of the component.
- **`IntoElement`**: A trait that converts various types into renderable UI elements.
- **`div()`**: Creates a div-like container element, similar to HTML divs. You can chain methods to style it (`.flex()`, `.bg()`, `.size()`, etc.)

#### Styling and Layout
- **`px(value)`**: Creates a pixel-based measurement (e.g., `px(100)` for 100 pixels)
- **`.flex()`**: Makes an element use flexbox layout
- **`.size(width, height)`**: Sets element dimensions
- **`.bg(color)`**: Sets background color
- **`.border_1()`**: Adds a 1-pixel border
- **`.rounded_md()`**: Adds medium border radius

#### Events and Interactions
- **`.on_drag()`**: Attaches a drag event handler to an element
- **`.cursor_pointer()`**: Changes cursor to pointer on hover
- **`.when(condition, |element| ...)`**: Conditionally applies styling or behavior

#### Colors and Styling
- **`white()`, `black()`, `blue()`**: Built-in color functions
- **`Point<Pixels>`**: Represents a 2D coordinate with pixel measurements

#### Custom Types (We'll Define)
- **`DragState`**: Our custom struct to track what's being dragged and where
- **`Position`**: Our custom enum to represent card locations in the game

### How GPUI Components Work
1. Components implement the `Render` trait
2. The `render()` method returns a tree of UI elements
3. GPUI efficiently updates only the parts that changed
4. Models provide reactive data that triggers re-renders automatically

## Technical Considerations

### GPUI Integration
- **Model System**: Use `Model<T>` for reactive state management - when game state changes, all dependent UI components automatically re-render
- **Component Architecture**: Build reusable UI components that can subscribe to model changes
- **Animation System**: Leverage GPUI's built-in animation system for smooth transitions
- **Event Handling**: Use GPUI's event handling for mouse and keyboard interactions
- **Drag and Drop**: Implement custom drag-and-drop using GPUI's gesture recognition system

### Performance Optimizations
- Minimize re-renders by using GPUI's efficient diffing system
- Cache card images and reuse components where possible
- Use GPUI's layout system for efficient positioning calculations
- Implement viewport culling for large card stacks if needed

### Platform Considerations
- Design for desktop-first experience with mouse interactions
- Ensure proper window resizing and minimum size constraints
- Support standard desktop keyboard shortcuts (Ctrl+Z for undo, F2 for new game)
- Integrate with system clipboard for potential future features