use crate::game::card::Position;

#[derive(Debug, Clone, PartialEq)]
pub enum GameAction {
    /// Move card(s) from one position to another
    MoveCard { from: Position, to: Position },
    /// Flip a face-down card to face-up
    FlipCard(Position),
    /// Deal cards from stock to waste pile
    DealFromStock,
    /// Start a new game
    NewGame,
    /// Undo the last move
    Undo,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DrawCount {
    One,   // Deal 1 card at a time from stock (easier)
    Three, // Deal 3 cards at a time from stock (harder)
}
