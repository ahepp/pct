use crate::end_game::EndGame;
use crate::state::State;

pub struct BeginGame;
impl BeginGame {
    pub fn new() -> Self {
        BeginGame
    }
}
impl State for BeginGame {
    fn render(&self) {
        println!("Welcome to the Pacific Crest Trail!");
    }
    fn next_state(self: Box<Self>) -> Option<Box<dyn State>> {
        Some(Box::new(EndGame))
    }
}
