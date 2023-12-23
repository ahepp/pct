use crate::state::State;

pub struct EndGame;
impl State for EndGame {
    fn render(&self) {
        println!("Goodbye!");
    }
    fn next_state(self: Box<Self>) -> Option<Box<dyn State>> {
        None
    }
}
