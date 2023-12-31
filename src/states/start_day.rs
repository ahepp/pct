use crate::context::Context;
use crate::state::State;

pub struct StartDay;
impl State for StartDay {
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

    #[test]
    fn next_state_start_day() {
        let ctx = Context;
        let state = StartDay;
        let state = match state.next_state(ctx) {
            (Some(state), _) => state,
            _ => panic!(),
        };
        assert!(state.as_any().is::<StartDay>());
    }
}
