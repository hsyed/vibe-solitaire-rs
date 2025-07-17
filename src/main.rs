use gpui::{
    AppContext, Application, Context, FontWeight, IntoElement, ParentElement, Render, Styled,
    Window, WindowOptions, div, rgb, white,
};

mod game;
mod ui;

use game::deck::{Card, Rank, Suit};
use game::state::GameState;
use ui::render_card;

struct SolitaireApp {
    game_state: GameState,
}

impl SolitaireApp {
    fn new() -> Self {
        Self {
            game_state: GameState::new(),
        }
    }
}

impl Render for SolitaireApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x0F5132))
            .p_4()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .text_xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(white())
                            .child("Klondike Solitaire - Card Component Test"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(white())
                            .child("Sample Cards - Testing CardComponent Rendering:"),
                    )
                    .child(
                        // Sample cards display
                        div()
                            .flex()
                            .gap_4()
                            .flex_wrap()
                            .p_4()
                            .child(render_card(Card::new(Suit::Hearts, Rank::Ace, true)))
                            .child(render_card(Card::new(Suit::Diamonds, Rank::King, true)))
                            .child(render_card(Card::new(Suit::Clubs, Rank::Queen, true)))
                            .child(render_card(Card::new(Suit::Spades, Rank::Jack, true)))
                            .child(render_card(Card::new(Suit::Hearts, Rank::Ten, true)))
                            .child(render_card(Card::new(Suit::Diamonds, Rank::Two, true)))
                            .child(render_card(Card::new(Suit::Clubs, Rank::Seven, false))) // Face down
                            .child(render_card(Card::new(Suit::Spades, Rank::Five, false))), // Face down
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(white())
                            .child(self.game_state.summary()),
                    )
                    .child(
                        div()
                            .bg(rgb(0x000000))
                            .text_color(rgb(0x00FF00))
                            .p_4()
                            .rounded_md()
                            .text_xs()
                            .child(self.game_state.debug_info()),
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