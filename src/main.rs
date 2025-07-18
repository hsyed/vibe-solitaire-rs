use gpui::{
    AppContext, Application, Context, ElementId, FontWeight, InteractiveElement, IntoElement,
    MouseButton, ParentElement, Render, Styled, Window, WindowOptions, div, prelude::*, px, rgb,
    white,
};

mod game;
mod ui;

use game::actions::GameAction;
use game::deck::Card;
use game::state::{GameState, Position};

#[derive(Debug, Clone)]
pub struct DragInfo {
    pub source_position: Position,
    pub dragged_cards: Vec<Card>,
    pub valid_drop_targets: Vec<Position>,
}

impl Render for DragInfo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // Render the dragged cards in a stack
        let mut drag_element = div().flex().flex_col().opacity(0.8); // Make it semi-transparent to show it's being dragged

        for (i, card) in self.dragged_cards.iter().enumerate() {
            let card_element = div()
                .child(ui::render_card(*card))
                .border_2()
                .border_color(rgb(0x3B82F6)); // Blue border to indicate dragging

            if i == 0 {
                drag_element = drag_element.child(card_element);
            } else {
                // Stack subsequent cards with small offset to show sequence
                drag_element = drag_element.child(
                    div()
                        .mt(px(-ui::CARD_HEIGHT + 12.0)) // Smaller offset for dragged cards
                        .child(card_element),
                );
            }
        }

        drag_element
    }
}

struct SolitaireApp {
    game_state: GameState,
    current_drag: Option<DragInfo>,
}

impl SolitaireApp {
    fn new() -> Self {
        Self {
            game_state: GameState::new(),
            current_drag: None,
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

    fn handle_drop(
        &mut self,
        drag_info: &DragInfo,
        drop_position: Position,
        cx: &mut Context<Self>,
    ) {
        if drag_info.valid_drop_targets.contains(&drop_position) {
            // Perform the move
            let move_action = GameAction::MoveCard {
                from: drag_info.source_position,
                to: drop_position,
            };
            self.handle_action(move_action, cx);
        }

        // Clear drag state
        self.current_drag = None;
        cx.notify();
    }

    fn get_draggable_cards(&self, position: Position) -> Vec<Card> {
        // Use the game state's logic to get draggable cards
        self.game_state
            .get_cards_at_position(position)
            .unwrap_or_else(|_| Vec::new())
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
            if self.can_drop_on_tableau(first_card, col)
                && !self.is_same_position(source, Position::Tableau(col, 0))
            {
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

    fn render_game_board_with_drag_drop(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let drag_info_text = "Drag and drop cards to move them! Foundation piles and tableau columns are drop targets.".to_string();

        div()
            .flex()
            .flex_col()
            .size_full()
            .gap_4()
            .child(
                // Drag state info
                div()
                    .text_sm()
                    .text_color(white())
                    .text_center()
                    .child(drag_info_text),
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
        // Don't highlight as we'll let the drop handler do validation
        let is_valid_drop_target = false;

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
            let empty_placeholder = div()
                .id(ElementId::Name(format!("tableau_{}", col).into()))
                .child(ui::render_empty_pile(""))
                .on_drop(cx.listener(move |app, drag_info: &DragInfo, _window, cx| {
                    println!("ON_DROP HANDLER CALLED: empty tableau column {}", col);
                    app.handle_drop(drag_info, drop_position, cx);
                }));
            column = column.child(empty_placeholder);
        } else {
            // Render stacked cards with drag functionality
            for (i, card) in cards.iter().enumerate() {
                let position = Position::Tableau(col, i);
                let is_top_card = i == cards.len() - 1;
                let is_draggable = card.face_up && !self.get_draggable_cards(position).is_empty();

                let mut card_element = if is_draggable {
                    // Face-up card that can be dragged (either single or as part of sequence)
                    let card_id = card.id();
                    div()
                        .id(ElementId::Name(format!("card_{}", card_id).into())) // TODO: ugh another format ?
                        .relative() // Ensure proper positioning
                        .child(ui::render_card(*card))
                        .cursor_pointer()
                        .hover(|style| style.shadow_xl().border_color(rgb(0x3B82F6)))
                        .on_drag(
                            {
                                let dragged_cards = self.get_draggable_cards(position);
                                let valid_drop_targets =
                                    self.get_valid_drop_targets(&dragged_cards, position);
                                DragInfo {
                                    source_position: position,
                                    dragged_cards,
                                    valid_drop_targets,
                                }
                            },
                            move |drag_info: &DragInfo, _cursor_position, _window, cx| {
                                println!(
                                    "Drag started: from {:?}, {} valid targets: {:?}",
                                    drag_info.source_position,
                                    drag_info.valid_drop_targets.len(),
                                    drag_info.valid_drop_targets
                                );
                                cx.new(|_| drag_info.clone())
                            },
                        )
                } else {
                    // Other cards - just render normally wrapped in div for type compatibility
                    div()
                        .id(ElementId::Name(format!("static_card_{}", card.id()).into())) // TODO: ugh another format ?
                        .child(ui::render_card(*card))
                };

                // Add drop functionality to the top card area
                if is_top_card {
                    let drop_position = Position::Tableau(col, cards.len());
                    card_element = card_element.on_drop(cx.listener(
                        move |app, drag_info: &DragInfo, _window, cx| {
                            println!(
                                "ON_DROP HANDLER CALLED: tableau column {} (on top card)",
                                col
                            );
                            app.handle_drop(drag_info, drop_position, cx);
                        },
                    ));
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
            div()
                .id(ElementId::Name("empty_waste".into()))
                .child(ui::render_empty_pile("Waste"))
        } else {
            let top_card = *self.game_state.waste.last().unwrap();
            let position = Position::Waste(self.game_state.waste.len() - 1);
            let card_id = top_card.id();

            // Make the waste pile card draggable
            div()
                .id(ElementId::Name(format!("waste_card_{}", card_id).into()))
                .child(ui::render_card(top_card))
                .cursor_pointer()
                .hover(|style| style.shadow_xl().border_color(rgb(0x3B82F6)))
                .on_drag(
                    {
                        let dragged_cards = self.get_draggable_cards(position);
                        let valid_drop_targets =
                            self.get_valid_drop_targets(&dragged_cards, position);
                        DragInfo {
                            source_position: position,
                            dragged_cards,
                            valid_drop_targets,
                        }
                    },
                    move |drag_info: &DragInfo, _cursor_position, _window, cx| {
                        println!(
                            "Drag started: from {:?}, {} valid targets: {:?}",
                            drag_info.source_position,
                            drag_info.valid_drop_targets.len(),
                            drag_info.valid_drop_targets
                        );
                        cx.new(|_| drag_info.clone())
                    },
                )
        }
    }

    fn render_foundation_with_drop(
        &mut self,
        foundation: usize,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let cards = &self.game_state.foundations[foundation];
        // Don't highlight as we'll let the drop handler do validation
        let is_valid_drop_target = false;

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
                .id(ElementId::Name(format!("foundation_{}", foundation).into()))
                .on_drop(cx.listener(move |app, drag_info: &DragInfo, _window, cx| {
                    println!("ON_DROP HANDLER CALLED: foundation {}", foundation);
                    app.handle_drop(drag_info, position, cx);
                }))
        } else {
            // Foundation with cards - show top card with drop functionality
            let card_element = ui::render_card(*cards.last().unwrap());

            // Always add drop functionality to foundation top cards
            div()
                .id(ElementId::Name(
                    format!("foundation_{}_top", foundation).into(),
                ))
                .child(card_element)
                .on_drop(cx.listener(move |app, drag_info: &DragInfo, _window, cx| {
                    println!(
                        "ON_DROP HANDLER CALLED: foundation {} (on top card)",
                        foundation
                    );
                    app.handle_drop(drag_info, position, cx);
                }))
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
