use crate::context::Context;
use crate::state::State;
use crate::states::StartHiking;

pub struct StartDay;
impl State for StartDay {
    fn next_state(&self, ctx: Context) -> (Option<Box<dyn State>>, Context) {
        let day = ctx.day + 1;

        let mut next_ctx = ctx;
        next_ctx.day = day;
        (Some(Box::new(StartHiking)), next_ctx)
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
    fn next_state_start_hiking() {
        let ctx = test_util::create_context();
        let state = StartDay;
        let state = match state.next_state(ctx) {
            (Some(state), _) => state,
            _ => panic!(),
        };
        assert!(state.as_any().is::<StartHiking>());
    }

    #[test]
    fn next_ctx_day_incremented() {
        let ctx = test_util::create_context();

        let expected_day = ctx.day + 1;

        let state = StartDay;
        let (_, ctx) = state.next_state(ctx);
        assert_eq!(ctx.day, expected_day);
    }
}
