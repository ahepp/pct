use crate::state::State;
use crate::util::*;

pub struct EndGame;
impl State for EndGame {
    fn render(&self) {
        println!("Goodbye!");
        std::thread::sleep(std::time::Duration::from_millis(RENDER_DELAY_MS));
    }
    fn next_state(self: Box<Self>) -> Option<Box<dyn State>> {
        None
    }
}
