use gpui::{
    FontWeight, IntoElement, ParentElement, Styled, div, px, rgb, white,
};

use crate::game::deck::Card;

// Card dimensions in pixels
const CARD_WIDTH: f32 = 80.0;
const CARD_HEIGHT: f32 = 112.0;

/// Render a single card
pub fn render_card(card: Card) -> impl IntoElement {
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
            .child(
                // Top-left rank and suit
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .p_1()
                    .child(
                        div()
                            .text_color(text_color)
                            .font_weight(FontWeight::BOLD)
                            .text_size(px(14.0))
                            .child(card.rank.display()),
                    )
                    .child(
                        div()
                            .text_color(text_color)
                            .text_size(px(16.0))
                            .child(card.suit.symbol()),
                    ),
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
    };

    div()
        .w(px(CARD_WIDTH))
        .h(px(CARD_HEIGHT))
        .bg(white())
        .border_2()
        .border_color(rgb(0x000000))
        .rounded_md()
        .shadow_lg()
        .child(card_content)
}