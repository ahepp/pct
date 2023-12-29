use crate::states::*;

pub fn run() {
    let mut ctx = Context {};
    let mut state = Event::BeginGame(BeginGameState {});
    loop {
        state.render(&ctx);
        (state, ctx) = match state.next_state(ctx) {
            (Some(state), ctx) => (state, ctx),
            _ => return,
        }
    }
}
