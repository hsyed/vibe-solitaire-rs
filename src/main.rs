use gpui::{
    AppContext, Application, Context, FontWeight, InteractiveElement, IntoElement, MouseButton,
    ParentElement, Render, Styled, Window, WindowOptions, div, px, rgb, white, Point, Pixels,
    MouseMoveEvent,
};

mod game;
mod ui;

use game::actions::GameAction;
use game::deck::Card;
use game::state::{GameState, Position};

#[derive(Debug, Clone)]
pub struct DragState {
    pub dragged_cards: Vec<Card>,
    pub source_position: Position,
    pub current_mouse_position: Point<Pixels>,
    pub valid_drop_targets: Vec<Position>,
    pub drag_offset: Point<Pixels>, // Offset from mouse to card origin
}

struct SolitaireApp {
    game_state: GameState,
    drag_state: Option<DragState>,
}

impl SolitaireApp {
    fn new() -> Self {
        Self {
            game_state: GameState::new(),
            drag_state: None,
        }
    }

    fn handle_action(&mut self, action: GameAction, cx: &mut Context<Self>) {
        match self.game_state.handle_action(action) {
            Ok(()) => {
                // Action succeeded, trigger a re-render
                cx.notify();
            }
            Err(error) => {
                // For now, just print the error. In a real app, we might show a message to the user
                println!("Action failed: {}", error);
            }
        }
    }

    fn start_drag(&mut self, position: Position, mouse_position: Point<Pixels>, cx: &mut Context<Self>) {
        let dragged_cards = self.get_draggable_cards(position);
        if !dragged_cards.is_empty() {
            let valid_drop_targets = self.get_valid_drop_targets(&dragged_cards, position);
            
            self.drag_state = Some(DragState {
                dragged_cards,
                source_position: position,
                current_mouse_position: mouse_position,
                valid_drop_targets,
                drag_offset: Point::new(px(0.0), px(0.0)), // Default offset for now
            });
            
            println!("Started dragging from {:?} with {} cards", position, self.drag_state.as_ref().unwrap().dragged_cards.len());
            cx.notify();
        }
    }

    fn end_drag(&mut self, drop_position: Option<Position>, cx: &mut Context<Self>) {
        if let Some(drag_state) = self.drag_state.take() {
            if let Some(target) = drop_position {
                if drag_state.valid_drop_targets.contains(&target) {
                    // Perform the move
                    let move_action = GameAction::MoveCard {
                        from: drag_state.source_position,
                        to: target,
                    };
                    self.handle_action(move_action, cx);
                    println!("Dropped cards at {:?}", target);
                } else {
                    println!("Invalid drop target: {:?}", target);
                }
            } else {
                println!("Drag cancelled - no drop target");
            }
            cx.notify();
        }
    }

    fn get_draggable_cards(&self, position: Position) -> Vec<Card> {
        // Use the game state's logic to get draggable cards
        match self.game_state.get_cards_at_position(position) {
            Ok(cards) => cards,
            Err(_) => Vec::new(),
        }
    }

    fn get_valid_drop_targets(&self, cards: &[Card], source: Position) -> Vec<Position> {
        if cards.is_empty() {
            return Vec::new();
        }
        
        let first_card = cards[0]; // The card that will be placed on the destination
        let mut targets = Vec::new();
        
        // Check tableau columns
        for col in 0..7 {
            let tableau_pos = Position::Tableau(col, self.game_state.tableau[col].len());
            if self.can_drop_on_tableau(first_card, col) && !self.is_same_position(source, Position::Tableau(col, 0)) {
                targets.push(tableau_pos);
            }
        }
        
        // Check foundation piles (only for single cards)
        if cards.len() == 1 {
            for foundation in 0..4 {
                let foundation_pos = Position::Foundation(foundation);
                if self.can_drop_on_foundation(first_card, foundation) {
                    targets.push(foundation_pos);
                }
            }
        }
        
        targets
    }

    fn can_drop_on_tableau(&self, card: Card, col: usize) -> bool {
        if col >= 7 {
            return false;
        }
        
        let pile = &self.game_state.tableau[col];
        if pile.is_empty() {
            // Can only place King on empty tableau
            return card.rank == game::deck::Rank::King;
        }
        
        let top_card = pile.last().unwrap();
        card.can_place_on_tableau(top_card)
    }

    fn can_drop_on_foundation(&self, card: Card, foundation: usize) -> bool {
        if foundation >= 4 {
            return false;
        }
        
        let pile = &self.game_state.foundations[foundation];
        let top_card = pile.last();
        card.can_place_on_foundation(top_card)
    }

    fn is_same_position(&self, pos1: Position, pos2: Position) -> bool {
        match (pos1, pos2) {
            (Position::Tableau(col1, _), Position::Tableau(col2, _)) => col1 == col2,
            _ => false,
        }
    }

    fn update_drag_position(&mut self, mouse_position: Point<Pixels>, cx: &mut Context<Self>) {
        if let Some(ref mut drag_state) = self.drag_state {
            drag_state.current_mouse_position = mouse_position;
            cx.notify(); // Trigger re-render to update overlay position
        }
    }

    fn render_game_board_with_drag_drop(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let drag_info = if let Some(ref drag_state) = self.drag_state {
            format!("Dragging {} cards from {:?} - {} valid targets (highlighted in green)", 
                drag_state.dragged_cards.len(), 
                drag_state.source_position,
                drag_state.valid_drop_targets.len())
        } else {
            "Click on face-up cards to drag them! Try dragging card sequences too!".to_string()
        };

        div()
            .flex()
            .flex_col()
            .size_full()
            .gap_4()
            .on_mouse_move(cx.listener(|app, event: &gpui::MouseMoveEvent, _window, cx| {
                // Update drag position when mouse moves
                app.update_drag_position(event.position, cx);
            }))
            .on_mouse_up(MouseButton::Left, cx.listener(|app, _event, _window, cx| {
                // End drag operation when mouse is released (cancel if no valid target)
                app.end_drag(None, cx);
            }))
            .child(
                // Drag state info
                div()
                    .text_sm()
                    .text_color(white())
                    .text_center()
                    .child(drag_info)
            )
            .child(
                // Top row: Stock, Waste, and Foundations
                div()
                    .flex()
                    .justify_between()
                    .items_start()
                    .child(
                        // Left side: Stock and Waste
                        div()
                            .flex()
                            .gap_2()
                            .child(self.render_clickable_stock_pile(cx))
                            .child(self.render_waste_pile_with_drag(cx)),
                    )
                    .child(
                        // Right side: Four foundation piles with drop zones
                        div()
                            .flex()
                            .gap_2()
                            .child(self.render_foundation_with_drop(0, cx))
                            .child(self.render_foundation_with_drop(1, cx))
                            .child(self.render_foundation_with_drop(2, cx))
                            .child(self.render_foundation_with_drop(3, cx)),
                    ),
            )
            .child(
                // Bottom row: Seven tableau columns with simple drag functionality
                div()
                    .flex()
                    .justify_center()
                    .gap_2()
                    .child(self.render_tableau_with_drag(0, cx))
                    .child(self.render_tableau_with_drag(1, cx))
                    .child(self.render_tableau_with_drag(2, cx))
                    .child(self.render_tableau_with_drag(3, cx))
                    .child(self.render_tableau_with_drag(4, cx))
                    .child(self.render_tableau_with_drag(5, cx))
                    .child(self.render_tableau_with_drag(6, cx)),
            )
    }

    fn render_tableau_with_drag(&mut self, col: usize, cx: &mut Context<Self>) -> impl IntoElement {
        let cards = &self.game_state.tableau[col];
        let is_valid_drop_target = self.drag_state.as_ref()
            .map(|drag| drag.valid_drop_targets.contains(&Position::Tableau(col, cards.len())))
            .unwrap_or(false);

        let mut column = div()
            .flex()
            .flex_col()
            .w(px(ui::CARD_WIDTH))
            .min_h(px(ui::CARD_HEIGHT));

        // Add drop zone styling if this is a valid drop target
        if is_valid_drop_target {
            column = column
                .bg(rgb(0x22C55E)) // Green highlight for valid drop
                .border_4()
                .border_color(rgb(0x16A34A)) // Darker green border
                .rounded_lg(); // More prominent rounded corners
        }

        if cards.is_empty() {
            // Show empty placeholder for tableau with drop functionality
            let drop_position = Position::Tableau(col, 0);
            column = column
                .child(ui::render_empty_pile(""))
                .on_mouse_up(
                    MouseButton::Left,
                    cx.listener(move |app, _event, _window, cx| {
                        println!("Dropped on empty tableau column {}", col);
                        app.end_drag(Some(drop_position), cx);
                    }),
                );
        } else {
            // Render stacked cards with drag functionality
            for (i, card) in cards.iter().enumerate() {
                let position = Position::Tableau(col, i);
                let is_top_card = i == cards.len() - 1;
                let is_draggable = card.face_up && !self.get_draggable_cards(position).is_empty();

                let mut card_element = if is_draggable {
                    // Face-up card that can be dragged (either single or as part of sequence)
                    div()
                        .relative() // Ensure proper positioning
                        .child(ui::render_card(*card))
                        .cursor_pointer()
                        .hover(|style| style.shadow_xl().border_color(rgb(0x3B82F6)))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |app, _event, _window, cx| {
                                println!("Mouse down on draggable card at {:?}", position);
                                let mouse_pos = Point::new(px(0.0), px(0.0));
                                app.start_drag(position, mouse_pos, cx);
                            }),
                        )
                } else if is_top_card && !card.face_up {
                    // Top face-down card - make it flippable with proper positioning
                    div()
                        .relative() // Ensure proper positioning
                        .child(ui::render_card(*card))
                        .cursor_pointer()
                        .hover(|style| style.shadow_xl().border_color(rgb(0xF59E0B)))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |app, _event, _window, cx| {
                                println!("Flipping card at {:?}", position);
                                app.handle_action(GameAction::FlipCard(position), cx);
                            }),
                        )
                } else {
                    // Other cards - just render normally wrapped in div for type compatibility
                    div().child(ui::render_card(*card))
                };

                // Add drop functionality to the top card area if it's a valid drop target
                if is_top_card && is_valid_drop_target {
                    let drop_position = Position::Tableau(col, cards.len());
                    card_element = card_element.on_mouse_up(
                        MouseButton::Left,
                        cx.listener(move |app, _event, _window, cx| {
                            println!("Dropped on tableau column {} (on top card)", col);
                            app.end_drag(Some(drop_position), cx);
                        }),
                    );
                }

                if i == 0 {
                    // First card - no offset
                    column = column.child(card_element);
                } else {
                    // Subsequent cards - add negative margin to create stacking effect
                    // For the top card, ensure it's positioned to receive mouse events
                    let card_container = if is_top_card {
                        div()
                            .mt(px(-ui::CARD_HEIGHT + ui::TABLEAU_CARD_OFFSET))
                            .relative() // Ensure proper positioning for mouse events
                            .child(card_element)
                    } else {
                        div()
                            .mt(px(-ui::CARD_HEIGHT + ui::TABLEAU_CARD_OFFSET))
                            .child(card_element)
                    };
                    column = column.child(card_container);
                }
            }
        }

        column
    }

    fn render_clickable_stock_pile(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        if self.game_state.stock.is_empty() {
            // Empty stock pile - clickable to recycle waste
            div()
                .w(px(ui::CARD_WIDTH))
                .h(px(ui::CARD_HEIGHT))
                .bg(rgb(0x1F2937))
                .border_2()
                .border_color(rgb(0x4B5563))
                .border_dashed()
                .rounded_md()
                .flex()
                .items_center()
                .justify_center()
                .cursor_pointer()
                .hover(|style| style.border_color(rgb(0x3B82F6)))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|app, _event, _window, cx| {
                        println!("Stock pile clicked! (empty) - Recycling waste to stock");
                        app.handle_action(GameAction::DealFromStock, cx);
                    }),
                )
                .child(
                    div()
                        .text_color(rgb(0x9CA3AF))
                        .text_size(px(12.0))
                        .font_weight(FontWeight::MEDIUM)
                        .child("Stock"),
                )
        } else {
            // Stock pile with cards - show face-down card
            div()
                .w(px(ui::CARD_WIDTH))
                .h(px(ui::CARD_HEIGHT))
                .bg(white())
                .border_2()
                .border_color(rgb(0x000000))
                .rounded_md()
                .shadow_lg()
                .cursor_pointer()
                .hover(|style| style.shadow_xl().border_color(rgb(0x3B82F6)))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|app, _event, _window, cx| {
                        println!("Stock pile clicked! (with cards) - Dealing cards");
                        app.handle_action(GameAction::DealFromStock, cx);
                    }),
                )
                .child(
                    div()
                        .size_full()
                        .bg(rgb(0x1E3A8A))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(div().text_color(white()).text_size(px(24.0)).child("🂠")),
                )
        }
    }

    fn render_waste_pile_with_drag(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        if self.game_state.waste.is_empty() {
            div().child(ui::render_empty_pile("Waste"))
        } else {
            let top_card = *self.game_state.waste.last().unwrap();
            let position = Position::Waste(self.game_state.waste.len() - 1);
            
            // Make the waste pile card draggable
            div()
                .child(ui::render_card(top_card))
                .cursor_pointer()
                .hover(|style| style.shadow_xl().border_color(rgb(0x3B82F6)))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |app, _event, _window, cx| {
                        println!("Mouse down on waste pile card at {:?}", position);
                        let mouse_pos = Point::new(px(0.0), px(0.0));
                        app.start_drag(position, mouse_pos, cx);
                    }),
                )
        }
    }

    fn render_foundation_with_drop(&mut self, foundation: usize, cx: &mut Context<Self>) -> impl IntoElement {
        let cards = &self.game_state.foundations[foundation];
        let is_valid_drop_target = self.drag_state.as_ref()
            .map(|drag| drag.valid_drop_targets.contains(&Position::Foundation(foundation)))
            .unwrap_or(false);

        let position = Position::Foundation(foundation);
        
        if cards.is_empty() {
            // Empty foundation - show drop zone
            let suit_labels = ["♥", "♦", "♣", "♠"];
            let suit_colors = [
                rgb(0xDC2626), // Hearts - red
                rgb(0xDC2626), // Diamonds - red
                rgb(0x000000), // Clubs - black
                rgb(0x000000), // Spades - black
            ];

            let mut empty_foundation = div()
                .w(px(ui::CARD_WIDTH))
                .h(px(ui::CARD_HEIGHT))
                .bg(rgb(0x1F2937)) // Dark gray background
                .border_2()
                .border_color(rgb(0x4B5563)) // Lighter gray border
                .border_dashed()
                .rounded_md()
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .text_color(suit_colors[foundation])
                        .text_size(px(32.0))
                        .child(suit_labels[foundation]),
                );

            if is_valid_drop_target {
                empty_foundation = empty_foundation
                    .bg(rgb(0x22C55E)) // Green highlight for valid drop zones
                    .border_4()
                    .border_color(rgb(0x16A34A)); // Darker green border
            }

            // Make it a drop target
            empty_foundation
                .on_mouse_up(
                    MouseButton::Left,
                    cx.listener(move |app, _event, _window, cx| {
                        app.end_drag(Some(position), cx);
                    }),
                )
        } else {
            // Foundation with cards - show top card with drop functionality
            let card_element = ui::render_card(*cards.last().unwrap());
            
            if is_valid_drop_target {
                // Wrap in highlighted container with drop functionality
                div()
                    .bg(rgb(0x22C55E)) // Green highlight for valid drop
                    .rounded_md()
                    .p_1()
                    .child(card_element)
                    .on_mouse_up(
                        MouseButton::Left,
                        cx.listener(move |app, _event, _window, cx| {
                            app.end_drag(Some(position), cx);
                        }),
                    )
            } else {
                // Just show the card without drop functionality when not a valid target
                div().child(card_element)
            }
        }
    }

    fn render_dragged_cards_overlay(&self) -> impl IntoElement {
        if let Some(ref drag_state) = self.drag_state {
            // Create a visual representation of the dragged cards following the cursor
            // Offset the cards slightly from the cursor for better visibility
            let offset_x = px(-ui::CARD_WIDTH / 2.0); // Center horizontally on cursor
            let offset_y = px(-ui::CARD_HEIGHT / 4.0); // Slightly above cursor
            
            let mut overlay = div()
                .absolute()
                .left(drag_state.current_mouse_position.x + offset_x)
                .top(drag_state.current_mouse_position.y + offset_y)
                .flex()
                .flex_col();

            // Render each dragged card with slight offset to show sequence
            for (i, card) in drag_state.dragged_cards.iter().enumerate() {
                let card_element = div()
                    .child(ui::render_card(*card))
                    .opacity(0.9) // Semi-transparent to show it's being dragged
                    .shadow_2xl() // Strong shadow for visual feedback
                    .border_2()
                    .border_color(rgb(0x3B82F6)); // Blue border to indicate dragging

                if i == 0 {
                    overlay = overlay.child(card_element);
                } else {
                    // Stack subsequent cards with small offset to show sequence
                    overlay = overlay.child(
                        div()
                            .mt(px(-ui::CARD_HEIGHT + 12.0)) // Smaller offset for dragged cards
                            .child(card_element)
                    );
                }
            }

            overlay
        } else {
            // No drag in progress, return empty div
            div().w(px(0.0)).h(px(0.0))
        }
    }
}

impl Render for SolitaireApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x0F5132)) // Green felt background
            .p_4()
            .relative() // Enable absolute positioning for overlay
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        // Header with game title
                        div()
                            .text_xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(white())
                            .text_center()
                            .child("Klondike Solitaire"),
                    )
                    .child(
                        // Game status bar
                        div()
                            .text_sm()
                            .text_color(white())
                            .text_center()
                            .child(self.game_state.summary()),
                    )
                    .child(
                        // Main game board with drag and drop functionality
                        self.render_game_board_with_drag_drop(cx),
                    ),
            )
            .child(
                // Dragged cards overlay (rendered on top)
                self.render_dragged_cards_overlay()
            )
    }
}

fn main() {
    Application::new().run(|cx| {
        // Configure the application to quit when all windows are closed
        cx.activate(true);

        cx.on_window_closed(|cx| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();

        // Open the main window
        let _window = cx
            .open_window(WindowOptions::default(), |_, cx| {
                cx.new(|_| SolitaireApp::new())
            })
            .unwrap();
    });
}