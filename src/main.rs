use gpui::{AppContext, Application, WindowOptions};

mod game;
mod ui;

use crate::ui::app::SolitaireApp;

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
