use crate::states::*;

pub struct HikeState;
impl State for HikeState {
    fn render(&self, _ctx: &Context) {}
    fn next_state(self, ctx: Context) -> (Option<Event>, Context) {
        let mm = ctx.mm + (ctx.bedtime - ctx.waketime) * ctx.speed;
        let time = ctx.bedtime;

        let mut next_ctx = ctx;
        next_ctx.mm = mm;
        next_ctx.time = time;
        (Some(Event::Wake(WakeState)), next_ctx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_event_wake() {
        let ctx = Context::new();
        let state = Event::Hike(HikeState);
        match state.next_state(ctx) {
            (Some(Event::Wake(_)), _) => return,
            _ => panic!(),
        }
    }
    #[test]
    fn next_ctx_mm_increases() {
        let mm = 501.3;
        let waketime = 1.0;
        let bedtime = 3.3;
        let speed = 4.1;
        let expected_mm = mm + (bedtime - waketime) * speed;

        let mut ctx = Context::new();
        ctx.mm = mm;
        ctx.waketime = waketime;
        ctx.bedtime = bedtime;
        ctx.speed = speed;
        let state = Event::Hike(HikeState);
        let (_, ctx) = state.next_state(ctx);
        assert_eq!(ctx.mm, expected_mm);
    }
    #[test]
    fn next_state_ctx_time_is_bedtime() {
        let ctx = Context::new();
        let expected_time = ctx.bedtime;
        let state = Event::Hike(HikeState);
        let (_, ctx) = state.next_state(ctx);
        assert_eq!(ctx.time, expected_time);
    }
}
