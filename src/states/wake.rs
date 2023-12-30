use crate::states::*;

pub struct WakeState;
impl State for WakeState {
    fn render(&self, ctx: &Context) {
        let day_number = ctx.day + 1;

        crate::util::render_timestamp(ctx.waketime);
        println!("You woke up on day {} at mile {:.2}", day_number, ctx.mm);
        crate::util::render_delay();
    }
    fn next_state(self, ctx: Context) -> (Option<Event>, Context) {
        let day = ctx.day + 1;
        let time = ctx.waketime;

        let mut next_ctx = ctx;
        next_ctx.day = day;
        next_ctx.time = time;
        (Some(Event::Hike(HikeState)), next_ctx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_event_wake() {
        let ctx = Context::new();
        let state = Event::Wake(WakeState);
        match state.next_state(ctx) {
            (Some(Event::Hike(_)), _) => return,
            _ => panic!(),
        }
    }
    #[test]
    fn next_ctx_day_incremented() {
        let day = 10;

        let mut ctx = Context::new();
        ctx.day = day;

        let state = Event::Wake(WakeState);
        let (_, ctx) = state.next_state(ctx);
        assert_eq!(ctx.day, day + 1);
    }
    #[test]
    fn wake_next_state_sets_time_to_waketime() {
        let waketime = 9.5;

        let mut ctx = Context::new();
        ctx.waketime = waketime;

        let state = Event::Wake(WakeState);
        let (_, ctx) = state.next_state(ctx);
        assert_eq!(ctx.time, waketime);
    }
}
