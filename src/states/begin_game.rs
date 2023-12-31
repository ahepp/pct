use crate::context::Context;
use crate::state::State;

pub struct BeginGame;
impl State for BeginGame {
    fn next_state(&self, ctx: Context) -> (Option<Box<dyn State>>, Context) {
        (None, ctx)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn next_state_none() {
        let ctx = Context;
        let state = BeginGame;
        match state.next_state(ctx) {
            (None, _) => return,
            _ => panic!(),
        }
    }
}
