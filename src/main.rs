mod context;
mod scene;
mod scenes;
mod state;
mod states;

use crate::context::Context;
use crate::state::State;
use crate::states::BeginGame;

fn main() {
    let mut ctx = Context {
        day: 0,
    };
    let mut state: Box<dyn State> = Box::new(BeginGame);
    loop {
        state.render(&ctx);
        (state, ctx) = match state.next_state(ctx) {
            (Some(state), ctx) => (state, ctx),
            _ => return,
        }
    }
}
