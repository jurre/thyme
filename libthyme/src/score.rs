extern crate cards;

use board::{Position,HPosition,VPosition};
use game::MoveType;

pub struct Score {
    /// The increase in score attained by playing this hand
    pub value: i32,
    /// A score bonus from playing this hand, such as from clearing a stack
    pub bonus: i32,
    /// Score multiplier from playing a lucky hand
    pub multiplier: i32,
}

pub struct Play {
    /// Cards played
    pub cards: Vec<cards::card::Card>,
    /// All positions/stacks cleared by this play
    pub cleared_positions: Vec<Position>,
    /// Type of play
    pub hand: MoveType,
}

pub trait Scorer {

    /// Creates a new scorer. Scores may take the lucky card into account, so
    /// it is provided as a helper.
    fn new(lucky_card: cards::card::Card) -> Self;

    /// Compute the score of a potential play
    fn check_play(&self, play: Play) -> Score;

    /// Update the score with information about the last play
    fn add_play(&mut self, play: Play);

    /// Bonus awarded for clearing a position
    fn bonus(&self, position: Position) -> i32;

    /// The in-game total score
    fn running_total(&self) -> i32;

    /// The final score including any completion bonuses or multipliers, etc
    fn final_total(&self, completion: bool) -> i32;

    /// Format a value as a score
    fn format_as_score(&self, value: i32) -> String;
}

/// Standard (no fifteens) scoring hands, double bonus awarded for cards
/// with the lucky card's suit
pub struct StandardScorer {
    lucky_suit: cards::card::Suit,
    total: i32,
}

impl Scorer for StandardScorer {

    fn new(lucky_card: cards::card::Card) -> StandardScorer {
        StandardScorer {
            total: 0,
            lucky_suit: lucky_card.suit,
        }
    }

    fn check_play(&self, play: Play) -> Score {
        Score {
            value: 0,
            bonus: 0,
            multiplier: 0,
        }
    }

    fn bonus(&self, position: Position) -> i32 {
        return 0
    }

    fn add_play(&mut self, play: Play) {
    }

    fn running_total(&self) -> i32 {
        return self.total
    }

    fn final_total(&self, completion: bool) -> i32 {
        return self.total
    }

    fn format_as_score(&self, value: i32) -> String {
        return format!("{}", value)
    }
}

