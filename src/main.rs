use gpui::{
    AppContext, Application, Context, FontWeight, InteractiveElement, IntoElement, MouseDownEvent,
    ParentElement, Render, Styled, Window, WindowOptions, div, px, rgb, white,
};

mod game;
mod ui;

use game::actions::GameAction;
use game::deck::Card;
use game::state::{GameState, Position};
use ui::render_game_board;

struct SolitaireApp {
    game_state: GameState,
}

impl SolitaireApp {
    fn new() -> Self {
        Self {
            game_state: GameState::new(),
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

    fn handle_stock_click(&mut self, cx: &mut Context<Self>) {
        self.handle_action(GameAction::DealFromStock, cx);
    }

    fn handle_tableau_click(&mut self, position: Position, cx: &mut Context<Self>) {
        // Check if this is a face-down card that can be flipped
        if let Position::Tableau(col, idx) = position {
            if let Some(card) = self.game_state.tableau[col].get(idx) {
                if !card.face_up {
                    self.handle_action(GameAction::FlipCard(position), cx);
                    return;
                }
            }
        }
        // Otherwise, it's a move action (not implemented yet)
        println!("Card move not implemented yet: {:?}", position);
    }

    fn render_game_board_with_stock_click(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        use crate::game::deck::{Card, Rank, Suit};
        use ui::{
            CARD_HEIGHT, CARD_WIDTH, render_foundation_pile, render_tableau_column,
            render_waste_pile,
        };

        div()
            .flex()
            .flex_col()
            .size_full()
            .gap_4()
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
                            .child(render_waste_pile(&self.game_state.waste)),
                    )
                    .child(
                        // Right side: Four foundation piles
                        div()
                            .flex()
                            .gap_2()
                            .child(render_foundation_pile(0, &self.game_state.foundations[0]))
                            .child(render_foundation_pile(1, &self.game_state.foundations[1]))
                            .child(render_foundation_pile(2, &self.game_state.foundations[2]))
                            .child(render_foundation_pile(3, &self.game_state.foundations[3])),
                    ),
            )
            .child(
                // Bottom row: Seven tableau columns
                div()
                    .flex()
                    .justify_center()
                    .gap_2()
                    .child(render_tableau_column(0, &self.game_state.tableau[0]))
                    .child(render_tableau_column(1, &self.game_state.tableau[1]))
                    .child(render_tableau_column(2, &self.game_state.tableau[2]))
                    .child(render_tableau_column(3, &self.game_state.tableau[3]))
                    .child(render_tableau_column(4, &self.game_state.tableau[4]))
                    .child(render_tableau_column(5, &self.game_state.tableau[5]))
                    .child(render_tableau_column(6, &self.game_state.tableau[6])),
            )
    }

    fn render_clickable_stock_pile(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        use crate::game::deck::{Card, Rank, Suit};

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
                .on_mouse_down(gpui::MouseButton::Left, cx.listener(|app, _event, _window, cx| {
                    println!("Stock pile clicked! (empty) - Recycling waste to stock");
                    app.handle_action(GameAction::DealFromStock, cx);
                }))
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
                .on_mouse_down(gpui::MouseButton::Left, cx.listener(|app, _event, _window, cx| {
                    println!("Stock pile clicked! (with cards) - Dealing cards");
                    app.handle_action(GameAction::DealFromStock, cx);
                }))
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
}

impl Render for SolitaireApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x0F5132)) // Green felt background
            .p_4()
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
                        // Main game board with clickable stock pile
                        self.render_game_board_with_stock_click(cx),
                    ),
            )
    }
}

fn main() {
    Application::new().run(|cx| {
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|_| SolitaireApp::new())
        })
        .unwrap();
    });
}
