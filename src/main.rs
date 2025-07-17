use gpui::{div, rgb, AppContext, Application, Context, IntoElement, ParentElement, Render, Styled, Window, WindowOptions};

mod game;
mod ui;

struct SolitaireApp;

impl Render for SolitaireApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x0F5132))
            .justify_center()
            .items_center()
            .child("Solitaire Game")
    }
}

fn main() {
    Application::new().run(|cx| {
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|_| SolitaireApp)
        })
        .unwrap();
    });
}