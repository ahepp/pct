use crate::begin_day::BeginDay;
use crate::context::Context;
use crate::state::State;
use crate::util::*;

pub struct BeginGame;
impl BeginGame {
    pub fn new() -> Self {
        BeginGame
    }
}
impl State for BeginGame {
    fn render(&self) {
        println!("Welcome to the Pacific Crest Trail!");
        std::thread::sleep(std::time::Duration::from_millis(RENDER_DELAY_MS));
    }
    fn next_state(self: Box<Self>) -> Option<Box<dyn State>> {
        Some(Box::new(BeginDay::new(Context::new())))
    }
}
