use crate::states::*;

pub struct EnterTownState;
impl State for EnterTownState {
    fn render(&self, ctx: &Context) {
        let town = &ctx.towns[ctx.town_idx];

        crate::util::render_timestamp(ctx.time);
        println!("You entered town {} at mi {:.2}", town.name, town.mm);
        crate::util::render_delay();
    }
    fn next_state(self, ctx: Context) -> (Option<Event>, Context) {
        let town_idx = ctx.town_idx + 1;

        let mut next_ctx = ctx;
        next_ctx.town_idx = town_idx;
        (Some(Event::Hike(HikeState)), next_ctx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_event_hike() {
        let ctx = Context::new();
        let state = Event::EnterTown(EnterTownState);
        match state.next_state(ctx) {
            (Some(Event::Hike(_)), _) => return,
            _ => panic!(),
        }
    }
    #[test]
    fn next_ctx_town_idx_incremented() {
        let ctx = Context::new();

        let expected_town_idx = ctx.town_idx + 1;

        let state = Event::EnterTown(EnterTownState);
        let (_, ctx) = state.next_state(ctx);
        assert_eq!(ctx.town_idx, expected_town_idx);
    }
}
