use crate::context::*;
use crate::state::State;
use crate::states::StartDay;

pub struct StartHiking;
impl State for StartHiking {
    fn next_state(&self, ctx: Context) -> (Option<Box<dyn State>>, Context) {
        let bedtime = ctx.hiking_opts.bedtime;
        let waketime = ctx.hiking_opts.waketime;
        let speed = ctx.hiking_opts.speed;
        let mm = ctx.mm + (bedtime - waketime) * speed;

        let mut next_ctx = ctx;
        next_ctx.mm = mm;
        (Some(Box::new(StartDay)), next_ctx)
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
        let state = StartHiking;
        let state = match state.next_state(ctx) {
            (Some(state), _) => state,
            _ => panic!(),
        };
        assert!(state.as_any().is::<StartDay>());
    }

    #[test]
    fn next_ctx_mm_increased_duration_x_speed() {
        let ctx = test_util::create_context();
        let bedtime = ctx.hiking_opts.bedtime;
        let waketime = ctx.hiking_opts.waketime;
        let speed = ctx.hiking_opts.speed;

        let expected_mm = ctx.mm + (bedtime - waketime) * speed;

        let state = StartHiking;
        let (_, ctx) = state.next_state(ctx);
        assert_eq!(ctx.mm, expected_mm);
    }
}
