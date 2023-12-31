mod context;
mod scene;
mod scenes;
mod state;
mod states;

use crate::context::*;
use crate::state::State;
use crate::states::BeginGame;

fn main() {
    let hiking_opts = HikingOpts {
        bedtime: 20.0,
        speed: 2.0,
        waketime: 8.0,
    };
    let mut ctx = Context {
        day: 0,
        hiking_opts,
        mm: 0.0,
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
