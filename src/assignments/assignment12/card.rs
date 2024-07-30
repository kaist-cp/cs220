//! Flipping card game.
//!
//! HINT: For this assignment, you have to see `play` function in `card_grade.rs` file.
//! Multiple threads will be created and they will run as enemy bots(`bot_threads`).
//! Strategy of the enemy bots is implemented in the closure of the `thread::spawn` function.
//! Your goal is to beat them so that there are more white cards than blue cards in the ground.
//! Write your strategy in the `flip_card_strategy` function of the `Player` struct.
//!
//! Have fun!

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Color represents the color of the card.
/// The color of a card can be either Blue or White.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    /// blue
    Blue,

    /// white
    White,
}

/// Player struct represents a player in the card game.
/// Each player has a memory which is represented as a HashMap.
#[derive(Debug)]
pub struct Player {
    memory: HashMap<isize, isize>,
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

impl Player {
    /// Creates a new player with an empty memory.
    pub fn new() -> Self {
        Self {
            memory: HashMap::new(),
        }
    }

    /// This function should return the index of the card to flip and the color to change to.
    pub fn flip_card_strategy(&mut self) -> (usize, Color) {
        todo!()
    }
}
