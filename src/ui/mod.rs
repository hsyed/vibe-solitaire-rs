use gpui::{
    AnyElement, FontWeight, IntoElement, ParentElement, Styled, div, px, rgb, white,
};

use crate::game::deck::Card;
use crate::game::state::GameState;

// Card dimensions in pixels
const CARD_WIDTH: f32 = 80.0;
const CARD_HEIGHT: f32 = 112.0;

// Layout constants
const CARD_SPACING: f32 = 8.0;
const PILE_SPACING: f32 = 16.0;
const TABLEAU_CARD_OFFSET: f32 = 20.0; // Vertical offset for stacked cards

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
                .child(label)
        )
}

/// Render a tableau column with stacked cards
pub fn render_tableau_column(_column_index: usize, cards: &[Card]) -> impl IntoElement {
    let mut column = div()
        .flex()
        .flex_col()
        .w(px(CARD_WIDTH))
        .min_h(px(CARD_HEIGHT));

    if cards.is_empty() {
        // Show empty placeholder for tableau
        column = column.child(render_empty_pile(""));
    } else {
        // Render stacked cards with offset
        for (i, card) in cards.iter().enumerate() {
            let card_element = render_card(*card);
            
            if i == 0 {
                // First card - no offset
                column = column.child(card_element);
            } else {
                // Subsequent cards - add negative margin to create stacking effect
                column = column.child(
                    div()
                        .mt(px(-CARD_HEIGHT + TABLEAU_CARD_OFFSET))
                        .child(card_element)
                );
            }
        }
    }

    column
}

/// Render an empty foundation pile with suit symbol
pub fn render_empty_foundation(pile_index: usize) -> impl IntoElement {
    let suit_labels = ["♥", "♦", "♣", "♠"];
    let suit_colors = [
        rgb(0xDC2626), // Hearts - red
        rgb(0xDC2626), // Diamonds - red  
        rgb(0x000000), // Clubs - black
        rgb(0x000000), // Spades - black
    ];

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
                .text_color(suit_colors[pile_index])
                .text_size(px(32.0))
                .child(suit_labels[pile_index])
        )
}

/// Render a foundation pile
pub fn render_foundation_pile(pile_index: usize, cards: &[Card]) -> AnyElement {
    if cards.is_empty() {
        render_empty_foundation(pile_index).into_any_element()
    } else {
        render_card(*cards.last().unwrap()).into_any_element()
    }
}

/// Render the stock pile
pub fn render_stock_pile(cards: &[Card]) -> AnyElement {
    if cards.is_empty() {
        render_empty_pile("Stock").into_any_element()
    } else {
        // Show face-down card representing the stock
        render_card(Card::new(
            crate::game::deck::Suit::Spades, 
            crate::game::deck::Rank::Ace, 
            false
        )).into_any_element()
    }
}

/// Render the waste pile
pub fn render_waste_pile(cards: &[Card]) -> AnyElement {
    if cards.is_empty() {
        render_empty_pile("Waste").into_any_element()
    } else {
        // Show top card of waste pile
        render_card(*cards.last().unwrap()).into_any_element()
    }
}

/// Main GameBoard component that renders the complete solitaire layout
pub fn render_game_board(game_state: &GameState) -> impl IntoElement {
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
                        .child(render_stock_pile(&game_state.stock))
                        .child(render_waste_pile(&game_state.waste))
                )
                .child(
                    // Right side: Four foundation piles
                    div()
                        .flex()
                        .gap_2()
                        .child(render_foundation_pile(0, &game_state.foundations[0]))
                        .child(render_foundation_pile(1, &game_state.foundations[1]))
                        .child(render_foundation_pile(2, &game_state.foundations[2]))
                        .child(render_foundation_pile(3, &game_state.foundations[3]))
                )
        )
        .child(
            // Bottom row: Seven tableau columns
            div()
                .flex()
                .justify_center()
                .gap_2()
                .child(render_tableau_column(0, &game_state.tableau[0]))
                .child(render_tableau_column(1, &game_state.tableau[1]))
                .child(render_tableau_column(2, &game_state.tableau[2]))
                .child(render_tableau_column(3, &game_state.tableau[3]))
                .child(render_tableau_column(4, &game_state.tableau[4]))
                .child(render_tableau_column(5, &game_state.tableau[5]))
                .child(render_tableau_column(6, &game_state.tableau[6]))
        )
}