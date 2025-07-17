use gpui::{
    AppContext, Application, Context, FontWeight, IntoElement, ParentElement, Render, Styled,
    Window, WindowOptions, div, px, rgb, white,
};

mod game;
mod ui;

use game::state::GameState;
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
}

impl Render for SolitaireApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
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
                        // Main game board
                        render_game_board(&self.game_state),
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
