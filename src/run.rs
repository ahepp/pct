use crate::begin_game::BeginGame;
use crate::state::State;

pub fn run() {
    let mut state: Box<dyn State> = Box::new(BeginGame::new());
    loop {
        state.render();
        state = match state.next_state() {
            None => return,
            Some(state) => state,
        };
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
