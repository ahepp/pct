use crate::context::Context;
use crate::state::State;
use crate::states::StartDay;

pub struct BeginGame;
impl State for BeginGame {
    fn next_state(&self, ctx: Context) -> (Option<Box<dyn State>>, Context) {
        (Some(Box::new(StartDay)), ctx)
    }

    #[cfg(test)]
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::states::test_util;

    #[test]
    fn next_state_start_day() {
        let ctx = test_util::create_context();
        let state = BeginGame;
        let state = match state.next_state(ctx) {
            (Some(state), _) => state,
            _ => panic!(),
        };
        assert!(state.as_any().is::<StartDay>());
    }
}
