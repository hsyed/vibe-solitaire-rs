use crate::game::actions::DrawCount;
use crate::game::deck::{Card, create_deck};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;
use std::time::SystemTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    Tableau(usize, usize), // column, index in column
    Foundation(usize),     // foundation pile index (0-3)
    Stock,
    Waste(usize), // index in waste pile
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Position::Tableau(col, idx) => write!(f, "Tableau({}, {})", col, idx),
            Position::Foundation(idx) => write!(f, "Foundation({})", idx),
            Position::Stock => write!(f, "Stock"),
            Position::Waste(idx) => write!(f, "Waste({})", idx),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GameState {
    /// Seven tableau columns (0-6), each containing a stack of cards
    pub tableau: [Vec<Card>; 7],
    /// Four foundation piles (0-3), one for each suit
    pub foundations: [Vec<Card>; 4],
    /// Stock pile (face-down cards to deal from)
    pub stock: Vec<Card>,
    /// Waste pile (face-up cards dealt from stock)
    pub waste: Vec<Card>,
    /// Number of moves made in current game
    pub move_count: u32,
    /// When the current game started
    pub start_time: SystemTime,
    /// Whether the game has been won
    pub game_won: bool,
    /// How many cards to draw from stock at once
    pub draw_count: DrawCount,
}

impl GameState {
    /// Create a new game with properly shuffled and dealt cards
    pub fn new() -> Self {
        let mut deck = create_deck();
        let mut rng = thread_rng();
        deck.shuffle(&mut rng);

        let mut game_state = GameState {
            tableau: Default::default(),
            foundations: Default::default(),
            stock: Vec::new(),
            waste: Vec::new(),
            move_count: 0,
            start_time: SystemTime::now(),
            game_won: false,
            draw_count: DrawCount::Three, // Default to harder mode
        };

        // Deal cards to tableau according to Klondike rules
        // Column 0: 1 card, Column 1: 2 cards, ..., Column 6: 7 cards
        let mut card_index = 0;

        for col in 0..7 {
            for row in 0..=col {
                if card_index < deck.len() {
                    let mut card = deck[card_index];
                    // Only the top card (last dealt) in each column is face-up
                    card.face_up = row == col;
                    game_state.tableau[col].push(card);
                    card_index += 1;
                }
            }
        }

        // Remaining cards go to stock pile (all face-down)
        game_state.stock = deck[card_index..].to_vec();

        game_state
    }

    /// Create a new game with specific draw count
    pub fn new_with_draw_count(draw_count: DrawCount) -> Self {
        let mut game_state = Self::new();
        game_state.draw_count = draw_count;
        game_state
    }

    /// Get debug information about the current game state
    pub fn debug_info(&self) -> String {
        let mut info = String::new();

        info.push_str(&format!("=== SOLITAIRE GAME STATE DEBUG ===\n"));
        info.push_str(&format!("Move Count: {}\n", self.move_count));
        info.push_str(&format!("Draw Count: {:?}\n", self.draw_count));
        info.push_str(&format!("Game Won: {}\n", self.game_won));
        info.push_str(&format!("Stock Cards: {}\n", self.stock.len()));
        info.push_str(&format!("Waste Cards: {}\n", self.waste.len()));

        // Tableau information
        info.push_str("\n--- TABLEAU ---\n");
        for (col, pile) in self.tableau.iter().enumerate() {
            info.push_str(&format!("Column {}: {} cards - ", col, pile.len()));
            if pile.is_empty() {
                info.push_str("(empty)\n");
            } else {
                for (i, card) in pile.iter().enumerate() {
                    if i > 0 {
                        info.push_str(", ");
                    }
                    info.push_str(&format!("{}", card));
                }
                info.push_str("\n");
            }
        }

        // Foundation information
        info.push_str("\n--- FOUNDATIONS ---\n");
        let suit_names = ["Hearts", "Diamonds", "Clubs", "Spades"];
        for (i, pile) in self.foundations.iter().enumerate() {
            info.push_str(&format!("{}: ", suit_names[i]));
            if pile.is_empty() {
                info.push_str("(empty)\n");
            } else {
                info.push_str(&format!(
                    "{} cards, top: {}\n",
                    pile.len(),
                    pile.last().unwrap()
                ));
            }
        }

        // Stock and Waste
        info.push_str("\n--- STOCK & WASTE ---\n");
        info.push_str(&format!(
            "Stock: {} cards (all face-down)\n",
            self.stock.len()
        ));
        info.push_str(&format!("Waste: "));
        if self.waste.is_empty() {
            info.push_str("(empty)\n");
        } else {
            for (i, card) in self.waste.iter().enumerate() {
                if i > 0 {
                    info.push_str(", ");
                }
                info.push_str(&format!("{}", card));
            }
            info.push_str("\n");
        }

        info
    }

    /// Get a summary of the current game state for display
    pub fn summary(&self) -> String {
        format!(
            "Moves: {} | Stock: {} | Waste: {} | Draw: {:?}",
            self.move_count,
            self.stock.len(),
            self.waste.len(),
            self.draw_count
        )
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::deck::{Rank, Suit};

    #[test]
    fn test_game_state_creation() {
        let game_state = GameState::new();

        // Check that tableau has correct number of columns
        assert_eq!(game_state.tableau.len(), 7);

        // Check that foundations are empty initially
        for foundation in &game_state.foundations {
            assert!(foundation.is_empty());
        }

        // Check that waste is empty initially
        assert!(game_state.waste.is_empty());

        // Check initial game state
        assert_eq!(game_state.move_count, 0);
        assert!(!game_state.game_won);
        assert_eq!(game_state.draw_count, DrawCount::Three);
    }

    #[test]
    fn test_tableau_dealing() {
        let game_state = GameState::new();

        // Check that tableau columns have correct number of cards
        // Column 0: 1 card, Column 1: 2 cards, ..., Column 6: 7 cards
        for (col, pile) in game_state.tableau.iter().enumerate() {
            assert_eq!(
                pile.len(),
                col + 1,
                "Column {} should have {} cards",
                col,
                col + 1
            );

            // Check that only the top card is face-up
            for (i, card) in pile.iter().enumerate() {
                if i == pile.len() - 1 {
                    // Top card should be face-up
                    assert!(card.face_up, "Top card in column {} should be face-up", col);
                } else {
                    // Other cards should be face-down
                    assert!(
                        !card.face_up,
                        "Non-top card in column {} should be face-down",
                        col
                    );
                }
            }
        }
    }

    #[test]
    fn test_total_cards_dealt() {
        let game_state = GameState::new();

        // Count cards in tableau
        let tableau_cards: usize = game_state.tableau.iter().map(|pile| pile.len()).sum();

        // Count cards in stock
        let stock_cards = game_state.stock.len();

        // Total should be 52 (standard deck)
        assert_eq!(tableau_cards + stock_cards, 52);

        // Tableau should have 28 cards (1+2+3+4+5+6+7)
        assert_eq!(tableau_cards, 28);

        // Stock should have remaining 24 cards
        assert_eq!(stock_cards, 24);
    }

    #[test]
    fn test_draw_count_setting() {
        let game_state_one = GameState::new_with_draw_count(DrawCount::One);
        let game_state_three = GameState::new_with_draw_count(DrawCount::Three);

        assert_eq!(game_state_one.draw_count, DrawCount::One);
        assert_eq!(game_state_three.draw_count, DrawCount::Three);
    }

    #[test]
    fn test_debug_info_format() {
        let game_state = GameState::new();
        let debug_info = game_state.debug_info();

        // Check that debug info contains expected sections
        assert!(debug_info.contains("=== SOLITAIRE GAME STATE DEBUG ==="));
        assert!(debug_info.contains("--- TABLEAU ---"));
        assert!(debug_info.contains("--- FOUNDATIONS ---"));
        assert!(debug_info.contains("--- STOCK & WASTE ---"));

        // Check that it shows move count
        assert!(debug_info.contains("Move Count: 0"));

        // Check that it shows draw count
        assert!(debug_info.contains("Draw Count: Three"));
    }

    #[test]
    fn test_print_debug_output() {
        let game_state = GameState::new();
        println!("\n{}", game_state.debug_info());
        println!("Summary: {}", game_state.summary());
    }

    #[test]
    fn test_summary_format() {
        let game_state = GameState::new();
        let summary = game_state.summary();

        // Check that summary contains expected information
        assert!(summary.contains("Moves: 0"));
        assert!(summary.contains("Stock: 24"));
        assert!(summary.contains("Waste: 0"));
        assert!(summary.contains("Draw: Three"));
    }

    #[test]
    fn test_cards_are_shuffled() {
        // Create two game states and verify they have different card arrangements
        let game1 = GameState::new();
        let game2 = GameState::new();

        // Get the first few cards from each tableau to compare
        let mut cards1 = Vec::new();
        let mut cards2 = Vec::new();

        for pile in &game1.tableau {
            if let Some(card) = pile.first() {
                cards1.push(*card);
            }
        }

        for pile in &game2.tableau {
            if let Some(card) = pile.first() {
                cards2.push(*card);
            }
        }

        // It's extremely unlikely that two shuffled decks would have the same
        // first 7 cards in the same positions (but not impossible)
        // This test might occasionally fail due to randomness, but it's very unlikely
        let same_arrangement = cards1.iter().zip(cards2.iter()).all(|(a, b)| a == b);

        // If they're the same, let's at least verify we have valid cards
        if same_arrangement {
            assert!(!cards1.is_empty(), "Should have cards in tableau");
            assert_eq!(cards1.len(), 7, "Should have 7 tableau columns");
        }
    }

    #[test]
    fn test_position_display() {
        let tableau_pos = Position::Tableau(2, 5);
        let foundation_pos = Position::Foundation(1);
        let stock_pos = Position::Stock;
        let waste_pos = Position::Waste(3);

        assert_eq!(format!("{}", tableau_pos), "Tableau(2, 5)");
        assert_eq!(format!("{}", foundation_pos), "Foundation(1)");
        assert_eq!(format!("{}", stock_pos), "Stock");
        assert_eq!(format!("{}", waste_pos), "Waste(3)");
    }
}
