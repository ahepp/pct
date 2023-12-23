use crate::begin_game::BeginGame;
use crate::state::State;

pub struct BeginDay {
    day: u32,
}
impl State for BeginDay {
    fn render(&self) {
        let day_number = self.day + 1;
        println!("You began day {}", day_number);
    }
    fn next_state(self: Box<Self>) -> Option<Box<dyn State>> {
        Some(Box::new(BeginDay { day: self.day + 1 }))
    }
}
impl From<Box<BeginGame>> for BeginDay {
    fn from(_: Box<BeginGame>) -> BeginDay {
        BeginDay { day: 0 }
    }
}
