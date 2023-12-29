use crate::states::*;

pub struct WakeState;
impl State for WakeState {
    fn render(&self, ctx: &Context) {
        let day_number = ctx.day + 1;
        println!("You woke up on day {}", day_number);
        crate::util::render_delay();
    }
    fn next_state(self, ctx: Context) -> (Option<Event>, Context) {
        let day = ctx.day + 1;

        let mut next_ctx = ctx;
        next_ctx.day = day;
        (Some(Event::Wake(WakeState)), next_ctx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_event_wake() {
        let ctx = Context { day: 0 };
        let state = Event::Wake(WakeState);
        match state.next_state(ctx) {
            (Some(Event::Wake(_)), _) => return,
            _ => panic!(),
        }
    }
    #[test]
    fn next_ctx_day_incremented() {
        let day = 10;
        let ctx = Context { day };
        let state = Event::Wake(WakeState);
        let (_, ctx) = state.next_state(ctx);
        assert_eq!(ctx.day, day + 1);
    }
}
