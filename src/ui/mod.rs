use gpui::{
    FontWeight, InteractiveElement, IntoElement, ParentElement, Styled, div, px, rgb, white,
};

pub mod app;

use crate::game::deck::Card;

// Card dimensions in pixels
pub const CARD_WIDTH: f32 = 80.0;
pub const CARD_HEIGHT: f32 = 112.0;

// Layout constants
pub const TABLEAU_CARD_OFFSET: f32 = 20.0; // Vertical offset for stacked cards

/// Render a single card with optional click handler and hover state
pub fn render_card_interactive(
    card: Card,
    clickable: bool,
    _on_click: Option<fn()>,
) -> impl IntoElement {
    let card_content = if !card.face_up {
        // Face-down card - show card back pattern
        div()
            .size_full()
            .bg(rgb(0x1E3A8A)) // Dark blue background
            .flex()
            .items_center()
            .justify_center()
            .child(div().text_color(white()).text_size(px(24.0)).child("🂠"))
    } else {
        // Face-up card - show rank and suit
        let text_color = if card.is_red() {
            rgb(0xDC2626) // Red color for hearts and diamonds
        } else {
            rgb(0x000000) // Black color for clubs and spades
        };

        div()
            .size_full()
            .flex()
            .flex_col()
            .p_1()
            .child(
                div()
                    .text_color(text_color)
                    .font_weight(FontWeight::BOLD)
                    .text_size(px(14.0))
                    .child(card.rank.display()),
            )
            .child(
                // Center suit symbol (larger)
                div().flex_1().flex().items_center().justify_center().child(
                    div()
                        .text_color(text_color)
                        .text_size(px(32.0))
                        .child(card.suit.symbol()),
                ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_end()
                    .justify_end()
                    .text_color(text_color)
                    .font_weight(FontWeight::BOLD)
                    .text_size(px(14.0))
                    .child(card.rank.display()),
            )
    };

    let mut card_div = div()
        .w(px(CARD_WIDTH))
        .h(px(CARD_HEIGHT))
        .bg(white())
        .border_2()
        .border_color(rgb(0x000000))
        .rounded_md()
        .shadow_lg();

    if clickable {
        card_div = card_div
            .cursor_pointer()
            .hover(|style| style.shadow_xl().border_color(rgb(0x3B82F6))); // Blue border on hover

        if let Some(click_handler) = _on_click {
            card_div = card_div.on_mouse_down(gpui::MouseButton::Left, move |_, _, _| {
                click_handler();
            });
        }
    }

    card_div.child(card_content)
}

/// Render an empty pile placeholder with visual indicator
pub fn render_empty_pile(label: &'static str) -> impl IntoElement {
    div()
        .w(px(CARD_WIDTH))
        .h(px(CARD_HEIGHT))
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
                .text_color(rgb(0x9CA3AF)) // Light gray text
                .text_size(px(12.0))
                .font_weight(FontWeight::MEDIUM)
                .child(label),
        )
}

/// Render a single card (non-interactive version)
pub fn render_card(card: Card) -> impl IntoElement {
    render_card_interactive(card, false, None::<fn()>)
}
