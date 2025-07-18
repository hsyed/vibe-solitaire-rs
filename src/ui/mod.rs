use gpui::{
    AnyElement, FontWeight, InteractiveElement, IntoElement, MouseDownEvent, ParentElement, Styled,
    div, px, rgb, white,
};

use crate::game::actions::GameAction;
use crate::game::deck::Card;
use crate::game::state::{GameState, Position};

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

/// Render a tableau column with stacked cards and click interactions
pub fn render_tableau_column_interactive<F>(
    column_index: usize,
    cards: &[Card],
    game_state: &GameState,
    mut on_click: F,
) -> impl IntoElement
where
    F: FnMut(Position) + 'static,
{
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
            let position = Position::Tableau(column_index, i);
            let is_clickable = game_state.can_click_position(position);
            let is_top_card = i == cards.len() - 1;

            let card_element = if is_clickable && is_top_card {
                let _pos = position;
                render_card_interactive(
                    *card,
                    true,
                    Some(|| {
                        // For now, just print a message - we'll fix the callback later
                        println!("Card clicked");
                    }),
                )
            } else {
                render_card_interactive(*card, false, None::<fn()>)
            };

            if i == 0 {
                // First card - no offset
                column = column.child(card_element);
            } else {
                // Subsequent cards - add negative margin to create stacking effect
                column = column.child(
                    div()
                        .mt(px(-CARD_HEIGHT + TABLEAU_CARD_OFFSET))
                        .child(card_element),
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
                .child(suit_labels[pile_index]),
        )
}

/// Render a foundation pile with click interactions
pub fn render_foundation_pile_interactive<F>(
    pile_index: usize,
    cards: &[Card],
    _game_state: &GameState,
    _on_click: F,
) -> AnyElement
where
    F: FnMut(Position) + 'static,
{
    if cards.is_empty() {
        render_empty_foundation(pile_index).into_any_element()
    } else {
        render_card_interactive(*cards.last().unwrap(), false, None::<fn()>).into_any_element()
    }
}

/// Render the stock pile with click interactions
pub fn render_stock_pile_interactive<F>(
    cards: &[Card],
    game_state: &GameState,
    mut on_click: F,
) -> AnyElement
where
    F: FnMut(Position) + 'static,
{
    let is_clickable = game_state.can_click_position(Position::Stock);

    if cards.is_empty() {
        let mut empty_pile = div()
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
                    .child("Stock"),
            );

        if is_clickable {
            empty_pile = empty_pile
                .cursor_pointer()
                .hover(|style| style.border_color(rgb(0x3B82F6)));
            // TODO: Implement click handling when we figure out the correct GPUI API
            // .on_click(move |_, _| on_click(Position::Stock));
        }

        empty_pile.into_any_element()
    } else {
        // Show face-down card representing the stock
        let stock_card = Card::new(
            crate::game::deck::Suit::Spades,
            crate::game::deck::Rank::Ace,
            false,
        );

        if is_clickable {
            // TODO: Fix click handling - for now just show as clickable
            render_card_interactive(stock_card, true, None::<fn()>).into_any_element()
        } else {
            render_card_interactive(stock_card, false, None::<fn()>).into_any_element()
        }
    }
}

/// Render the waste pile with click interactions
pub fn render_waste_pile_interactive<F>(
    cards: &[Card],
    _game_state: &GameState,
    _on_click: F,
) -> AnyElement
where
    F: FnMut(Position) + 'static,
{
    if cards.is_empty() {
        render_empty_pile("Waste").into_any_element()
    } else {
        // Show top card of waste pile (not clickable for now)
        render_card_interactive(*cards.last().unwrap(), false, None::<fn()>).into_any_element()
    }
}

/// Render a single card (non-interactive version)
pub fn render_card(card: Card) -> impl IntoElement {
    render_card_interactive(card, false, None::<fn()>)
}

/// Render a tableau column (non-interactive version)
pub fn render_tableau_column(_column_index: usize, cards: &[Card]) -> impl IntoElement {
    let mut column = div()
        .flex()
        .flex_col()
        .w(px(CARD_WIDTH))
        .min_h(px(CARD_HEIGHT));

    if cards.is_empty() {
        column = column.child(render_empty_pile(""));
    } else {
        for (i, card) in cards.iter().enumerate() {
            let card_element = render_card(*card);

            if i == 0 {
                column = column.child(card_element);
            } else {
                column = column.child(
                    div()
                        .mt(px(-CARD_HEIGHT + TABLEAU_CARD_OFFSET))
                        .child(card_element),
                );
            }
        }
    }

    column
}

/// Render a foundation pile (non-interactive version)
pub fn render_foundation_pile(pile_index: usize, cards: &[Card]) -> AnyElement {
    if cards.is_empty() {
        render_empty_foundation(pile_index).into_any_element()
    } else {
        render_card(*cards.last().unwrap()).into_any_element()
    }
}

/// Render the stock pile (non-interactive version)
pub fn render_stock_pile(cards: &[Card]) -> AnyElement {
    if cards.is_empty() {
        render_empty_pile("Stock").into_any_element()
    } else {
        render_card(Card::new(
            crate::game::deck::Suit::Spades,
            crate::game::deck::Rank::Ace,
            false,
        ))
        .into_any_element()
    }
}

/// Render the waste pile (non-interactive version)
pub fn render_waste_pile(cards: &[Card]) -> AnyElement {
    if cards.is_empty() {
        render_empty_pile("Waste").into_any_element()
    } else {
        render_card(*cards.last().unwrap()).into_any_element()
    }
}

/// Main GameBoard component that renders the complete solitaire layout (non-interactive for now)
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
                        .child(render_waste_pile(&game_state.waste)),
                )
                .child(
                    // Right side: Four foundation piles
                    div()
                        .flex()
                        .gap_2()
                        .child(render_foundation_pile(0, &game_state.foundations[0]))
                        .child(render_foundation_pile(1, &game_state.foundations[1]))
                        .child(render_foundation_pile(2, &game_state.foundations[2]))
                        .child(render_foundation_pile(3, &game_state.foundations[3])),
                ),
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
                .child(render_tableau_column(6, &game_state.tableau[6])),
        )
}
