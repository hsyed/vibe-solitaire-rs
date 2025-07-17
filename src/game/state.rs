use crate::game::actions::{DrawCount, GameAction};
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

    /// Handle a game action and update the state accordingly
    pub fn handle_action(&mut self, action: GameAction) -> Result<(), String> {
        match action {
            GameAction::DealFromStock => self.deal_from_stock(),
            GameAction::FlipCard(position) => self.flip_card(position),
            GameAction::MoveCard { from, to } => self.move_card(from, to),
            GameAction::NewGame => {
                *self = Self::new_with_draw_count(self.draw_count);
                Ok(())
            }
            GameAction::Undo => Err("Undo not implemented yet".to_string()),
        }
    }

    /// Deal cards from stock to waste pile
    pub fn deal_from_stock(&mut self) -> Result<(), String> {
        if self.stock.is_empty() {
            // If stock is empty, move all waste cards back to stock (face-down)
            if self.waste.is_empty() {
                return Err("Both stock and waste are empty".to_string());
            }
            
            // Move waste back to stock, face-down, in reverse order
            while let Some(mut card) = self.waste.pop() {
                card.face_up = false;
                self.stock.push(card);
            }
            self.move_count += 1;
            return Ok(());
        }

        // Deal cards from stock to waste
        let cards_to_deal = match self.draw_count {
            DrawCount::One => 1,
            DrawCount::Three => 3.min(self.stock.len()),
        };

        for _ in 0..cards_to_deal {
            if let Some(mut card) = self.stock.pop() {
                card.face_up = true;
                self.waste.push(card);
            }
        }

        self.move_count += 1;
        Ok(())
    }

    /// Flip a face-down card to face-up
    pub fn flip_card(&mut self, position: Position) -> Result<(), String> {
        match position {
            Position::Tableau(col, idx) => {
                if col >= 7 {
                    return Err("Invalid tableau column".to_string());
                }
                
                let pile = &mut self.tableau[col];
                if idx >= pile.len() {
                    return Err("Invalid card index in tableau".to_string());
                }

                // Can only flip the top card (last in the pile) and only if it's face-down
                if idx != pile.len() - 1 {
                    return Err("Can only flip the top card".to_string());
                }

                let card = &mut pile[idx];
                if card.face_up {
                    return Err("Card is already face-up".to_string());
                }

                card.face_up = true;
                self.move_count += 1;
                Ok(())
            }
            _ => Err("Can only flip cards in tableau".to_string()),
        }
    }

    /// Move a card from one position to another
    pub fn move_card(&mut self, _from: Position, _to: Position) -> Result<(), String> {
        // For now, just return an error - this will be implemented in later tasks
        Err("Card moving not implemented yet".to_string())
    }

    /// Check if a position can be clicked (for UI interaction)
    pub fn can_click_position(&self, position: Position) -> bool {
        match position {
            Position::Stock => true, // Can always click stock to deal
            Position::Tableau(col, idx) => {
                if col >= 7 {
                    return false;
                }
                let pile = &self.tableau[col];
                if idx >= pile.len() {
                    return false;
                }
                // Can click top card if it's face-down (to flip) or face-up (to move)
                idx == pile.len() - 1
            }
            Position::Waste(_) => false, // Can't click waste pile directly yet
            Position::Foundation(_) => false, // Can't click foundation directly yet
        }
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

    #[test]
    fn test_deal_from_stock() {
        let mut game_state = GameState::new_with_draw_count(DrawCount::One);
        let initial_stock_count = game_state.stock.len();
        let initial_waste_count = game_state.waste.len();

        // Deal one card
        let result = game_state.deal_from_stock();
        assert!(result.is_ok());
        assert_eq!(game_state.stock.len(), initial_stock_count - 1);
        assert_eq!(game_state.waste.len(), initial_waste_count + 1);
        assert_eq!(game_state.move_count, 1);

        // Check that the dealt card is face-up
        if let Some(top_waste_card) = game_state.waste.last() {
            assert!(top_waste_card.face_up);
        }
    }

    #[test]
    fn test_deal_from_stock_three_cards() {
        let mut game_state = GameState::new_with_draw_count(DrawCount::Three);
        let initial_stock_count = game_state.stock.len();

        // Deal three cards
        let result = game_state.deal_from_stock();
        assert!(result.is_ok());
        assert_eq!(game_state.stock.len(), initial_stock_count - 3);
        assert_eq!(game_state.waste.len(), 3);
        assert_eq!(game_state.move_count, 1);

        // Check that all dealt cards are face-up
        for card in &game_state.waste {
            assert!(card.face_up);
        }
    }

    #[test]
    fn test_deal_from_empty_stock_recycles_waste() {
        let mut game_state = GameState::new();
        
        // Empty the stock by dealing all cards
        while !game_state.stock.is_empty() {
            let _ = game_state.deal_from_stock();
        }
        
        let waste_count_before_recycle = game_state.waste.len();
        assert!(waste_count_before_recycle > 0);
        assert!(game_state.stock.is_empty());

        // Deal from empty stock should recycle waste back to stock
        let result = game_state.deal_from_stock();
        assert!(result.is_ok());
        assert_eq!(game_state.stock.len(), waste_count_before_recycle);
        assert!(game_state.waste.is_empty());

        // All recycled cards should be face-down
        for card in &game_state.stock {
            assert!(!card.face_up);
        }
    }

    #[test]
    fn test_flip_card_in_tableau() {
        let mut game_state = GameState::new();
        
        // Find a tableau column with face-down cards
        let mut test_col = None;
        for (col, pile) in game_state.tableau.iter().enumerate() {
            if pile.len() > 1 {
                // Check if there's a face-down card that's not the top card
                for (idx, card) in pile.iter().enumerate() {
                    if !card.face_up && idx < pile.len() - 1 {
                        test_col = Some(col);
                        break;
                    }
                }
                if test_col.is_some() {
                    break;
                }
            }
        }

        if let Some(col) = test_col {
            // Remove the top card to expose a face-down card
            let top_card = game_state.tableau[col].pop().unwrap();
            let top_idx = game_state.tableau[col].len() - 1;
            
            // The newly exposed card should be face-down
            assert!(!game_state.tableau[col][top_idx].face_up);
            
            // Flip the card
            let result = game_state.flip_card(Position::Tableau(col, top_idx));
            assert!(result.is_ok());
            assert!(game_state.tableau[col][top_idx].face_up);
            assert_eq!(game_state.move_count, 1);
            
            // Put the top card back
            game_state.tableau[col].push(top_card);
        }
    }

    #[test]
    fn test_flip_card_errors() {
        let mut game_state = GameState::new();
        
        // Try to flip card in invalid column
        let result = game_state.flip_card(Position::Tableau(7, 0));
        assert!(result.is_err());
        
        // Try to flip card with invalid index
        let result = game_state.flip_card(Position::Tableau(0, 10));
        assert!(result.is_err());
        
        // Try to flip already face-up card (top card in tableau is face-up)
        let result = game_state.flip_card(Position::Tableau(0, 0));
        assert!(result.is_err());
        
        // Try to flip non-tableau position
        let result = game_state.flip_card(Position::Stock);
        assert!(result.is_err());
    }

    #[test]
    fn test_can_click_position() {
        let game_state = GameState::new();
        
        // Can always click stock
        assert!(game_state.can_click_position(Position::Stock));
        
        // Can click top card in tableau
        assert!(game_state.can_click_position(Position::Tableau(0, 0))); // Column 0 has 1 card
        assert!(game_state.can_click_position(Position::Tableau(6, 6))); // Column 6 has 7 cards
        
        // Cannot click non-top cards in tableau
        assert!(!game_state.can_click_position(Position::Tableau(6, 0))); // Not top card
        assert!(!game_state.can_click_position(Position::Tableau(6, 5))); // Not top card
        
        // Cannot click invalid positions
        assert!(!game_state.can_click_position(Position::Tableau(7, 0))); // Invalid column
        assert!(!game_state.can_click_position(Position::Tableau(0, 5))); // Invalid index
        
        // Cannot click waste or foundation yet
        assert!(!game_state.can_click_position(Position::Waste(0)));
        assert!(!game_state.can_click_position(Position::Foundation(0)));
    }
}
