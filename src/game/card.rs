use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
    pub face_up: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rank {
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    Tableau(usize, usize), // column, index in column
    Foundation(usize),     // foundation pile index (0-3)
    Stock,
    Waste(usize),         // index in waste pile
}

impl Card {
    pub fn new(suit: Suit, rank: Rank, face_up: bool) -> Self {
        Card { suit, rank, face_up }
    }

    /// Check if this card is red (Hearts or Diamonds)
    pub fn is_red(&self) -> bool {
        matches!(self.suit, Suit::Hearts | Suit::Diamonds)
    }

    /// Check if this card is black (Clubs or Spades)
    pub fn is_black(&self) -> bool {
        matches!(self.suit, Suit::Clubs | Suit::Spades)
    }

    /// Check if this card can be placed on another card in tableau (alternating colors, descending rank)
    pub fn can_place_on_tableau(&self, other: &Card) -> bool {
        if !other.face_up {
            return false;
        }
        
        // Must be alternating colors
        let colors_alternate = (self.is_red() && other.is_black()) || (self.is_black() && other.is_red());
        
        // Must be one rank lower
        let rank_valid = (self.rank as u8) == (other.rank as u8) - 1;
        
        colors_alternate && rank_valid
    }

    /// Check if this card can be placed on a foundation pile
    pub fn can_place_on_foundation(&self, foundation_top: Option<&Card>) -> bool {
        match foundation_top {
            None => self.rank == Rank::Ace, // Only Ace can start a foundation
            Some(top) => {
                // Must be same suit and one rank higher
                self.suit == top.suit && (self.rank as u8) == (top.rank as u8) + 1
            }
        }
    }

    /// Flip the card (change face_up state)
    pub fn flip(&mut self) {
        self.face_up = !self.face_up;
    }

    /// Get a flipped version of this card
    pub fn flipped(&self) -> Self {
        Card {
            suit: self.suit,
            rank: self.rank,
            face_up: !self.face_up,
        }
    }
}

impl Suit {
    /// Get all suits in order
    pub fn all() -> [Suit; 4] {
        [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades]
    }

    /// Get the symbol for this suit
    pub fn symbol(&self) -> &'static str {
        match self {
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
        }
    }
}

impl Rank {
    /// Get all ranks in order
    pub fn all() -> [Rank; 13] {
        [
            Rank::Ace, Rank::Two, Rank::Three, Rank::Four, Rank::Five,
            Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten,
            Rank::Jack, Rank::Queen, Rank::King,
        ]
    }

    /// Get the display string for this rank
    pub fn display(&self) -> &'static str {
        match self {
            Rank::Ace => "A",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.face_up {
            write!(f, "{}{}", self.rank.display(), self.suit.symbol())
        } else {
            write!(f, "🂠") // Card back symbol
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display())
    }
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

/// Create a standard 52-card deck
pub fn create_deck() -> Vec<Card> {
    let mut deck = Vec::with_capacity(52);
    
    for suit in Suit::all() {
        for rank in Rank::all() {
            deck.push(Card::new(suit, rank, false)); // All cards start face down
        }
    }
    
    deck
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_creation() {
        let card = Card::new(Suit::Hearts, Rank::Ace, true);
        assert_eq!(card.suit, Suit::Hearts);
        assert_eq!(card.rank, Rank::Ace);
        assert_eq!(card.face_up, true);
    }

    #[test]
    fn test_card_colors() {
        let red_card = Card::new(Suit::Hearts, Rank::King, true);
        let black_card = Card::new(Suit::Spades, Rank::Queen, true);
        
        assert!(red_card.is_red());
        assert!(!red_card.is_black());
        assert!(black_card.is_black());
        assert!(!black_card.is_red());
    }

    #[test]
    fn test_tableau_placement_rules() {
        let red_king = Card::new(Suit::Hearts, Rank::King, true);
        let black_queen = Card::new(Suit::Spades, Rank::Queen, true);
        let red_queen = Card::new(Suit::Diamonds, Rank::Queen, true);
        let black_jack = Card::new(Suit::Clubs, Rank::Jack, true);
        
        // Black Queen can go on Red King (alternating colors, descending rank)
        assert!(black_queen.can_place_on_tableau(&red_king));
        
        // Red Queen cannot go on Red King (same color)
        assert!(!red_queen.can_place_on_tableau(&red_king));
        
        // Black Jack can go on Red Queen
        assert!(black_jack.can_place_on_tableau(&red_queen));
        
        // Black Queen cannot go on Black Jack (wrong rank order)
        assert!(!black_queen.can_place_on_tableau(&black_jack));
    }

    #[test]
    fn test_foundation_placement_rules() {
        let ace_hearts = Card::new(Suit::Hearts, Rank::Ace, true);
        let two_hearts = Card::new(Suit::Hearts, Rank::Two, true);
        let two_spades = Card::new(Suit::Spades, Rank::Two, true);
        let three_hearts = Card::new(Suit::Hearts, Rank::Three, true);
        
        // Ace can start a foundation
        assert!(ace_hearts.can_place_on_foundation(None));
        
        // Two of Hearts can go on Ace of Hearts
        assert!(two_hearts.can_place_on_foundation(Some(&ace_hearts)));
        
        // Two of Spades cannot go on Ace of Hearts (wrong suit)
        assert!(!two_spades.can_place_on_foundation(Some(&ace_hearts)));
        
        // Three of Hearts cannot go on Ace of Hearts (wrong rank)
        assert!(!three_hearts.can_place_on_foundation(Some(&ace_hearts)));
        
        // Only Ace can start foundation
        assert!(!two_hearts.can_place_on_foundation(None));
    }

    #[test]
    fn test_card_flipping() {
        let mut card = Card::new(Suit::Clubs, Rank::Seven, false);
        assert!(!card.face_up);
        
        card.flip();
        assert!(card.face_up);
        
        card.flip();
        assert!(!card.face_up);
        
        let flipped = card.flipped();
        assert!(flipped.face_up);
        assert!(!card.face_up); // Original unchanged
    }

    #[test]
    fn test_deck_creation() {
        let deck = create_deck();
        assert_eq!(deck.len(), 52);
        
        // Check we have all suits and ranks
        let mut hearts_count = 0;
        let mut aces_count = 0;
        
        for card in &deck {
            if card.suit == Suit::Hearts {
                hearts_count += 1;
            }
            if card.rank == Rank::Ace {
                aces_count += 1;
            }
            // All cards should start face down
            assert!(!card.face_up);
        }
        
        assert_eq!(hearts_count, 13); // 13 hearts
        assert_eq!(aces_count, 4);    // 4 aces
    }

    #[test]
    fn test_rank_ordering() {
        assert!(Rank::Ace < Rank::Two);
        assert!(Rank::Two < Rank::Three);
        assert!(Rank::Queen < Rank::King);
        assert_eq!(Rank::Ace as u8, 1);
        assert_eq!(Rank::King as u8, 13);
    }

    #[test]
    fn test_card_display() {
        let face_up_card = Card::new(Suit::Hearts, Rank::Ace, true);
        let face_down_card = Card::new(Suit::Spades, Rank::King, false);
        
        assert_eq!(format!("{}", face_up_card), "A♥");
        assert_eq!(format!("{}", face_down_card), "🂠");
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
    fn test_suit_and_rank_symbols() {
        assert_eq!(Suit::Hearts.symbol(), "♥");
        assert_eq!(Suit::Diamonds.symbol(), "♦");
        assert_eq!(Suit::Clubs.symbol(), "♣");
        assert_eq!(Suit::Spades.symbol(), "♠");
        
        assert_eq!(Rank::Ace.display(), "A");
        assert_eq!(Rank::Ten.display(), "10");
        assert_eq!(Rank::Jack.display(), "J");
        assert_eq!(Rank::Queen.display(), "Q");
        assert_eq!(Rank::King.display(), "K");
    }
}