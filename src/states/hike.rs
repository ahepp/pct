use crate::states::*;

pub struct HikeState;
impl State for HikeState {
    fn render(&self, _ctx: &Context) {}
    fn next_state(self, ctx: Context) -> (Option<Event>, Context) {
        let (is_town, distance) = distance_to_hike(&ctx);
        let mm = ctx.mm;
        let time = ctx.time + distance / ctx.speed;

        let mut next_ctx = ctx;
        next_ctx.mm = mm + distance;
        next_ctx.time = time;
        match is_town {
            true => (Some(Event::EnterTown(EnterTownState)), next_ctx),
            false => (Some(Event::Wake(WakeState)), next_ctx),
        }
    }
}

fn distance_to_town(ctx: &Context) -> Option<f32> {
    match ctx.town_idx < ctx.towns.len() {
        true => Some(ctx.towns[ctx.town_idx].mm - ctx.mm),
        false => None,
    }
}

fn distance_to_hike(ctx: &Context) -> (bool, f32) {
    let range = (ctx.bedtime - ctx.time) * ctx.speed;
    match distance_to_town(ctx) {
        Some(distance) => {
            if !(distance > range) {
                return (true, distance);
            }
        }
        None => {}
    }
    (false, range)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn context_with_town_in_range() -> Context {
        let towns = vec![Town {
            name: "tinseltown".to_string(),
            mm: 511.0,
        }];
        Context {
            bedtime: 10.4,
            day: 105,
            mm: 510.3,
            speed: 2.7,
            time: 4.9,
            town_idx: 0,
            towns,
            waketime: 2.0,
        }
    }
    fn context_without_town_in_range() -> Context {
        let towns = vec![Town {
            name: "tinseltown".to_string(),
            mm: 1000.0,
        }];
        Context {
            bedtime: 10.4,
            day: 105,
            mm: 510.3,
            speed: 2.7,
            time: 4.9,
            town_idx: 0,
            towns,
            waketime: 2.0,
        }
    }

    #[test]
    fn no_town_next_event_wake() {
        let ctx = context_without_town_in_range();
        let state = Event::Hike(HikeState);
        match state.next_state(ctx) {
            (Some(Event::Wake(_)), _) => return,
            _ => panic!(),
        }
    }
    #[test]
    fn no_town_next_ctx_mm_increases() {
        let ctx = context_without_town_in_range();

        let expected_mm = ctx.mm + (ctx.bedtime - ctx.time) * ctx.speed;

        let state = Event::Hike(HikeState);
        let (_, ctx) = state.next_state(ctx);
        assert_eq!(ctx.mm, expected_mm);
    }
    #[test]
    fn no_town_next_ctx_time_is_bedtime() {
        let ctx = context_without_town_in_range();

        let expected_time = ctx.bedtime;

        let state = Event::Hike(HikeState);
        let (_, ctx) = state.next_state(ctx);
        assert_eq!(ctx.time, expected_time);
    }
    #[test]
    fn town_next_event_enter_town() {
        let ctx = context_with_town_in_range();

        let state = Event::Hike(HikeState);
        match state.next_state(ctx) {
            (Some(Event::EnterTown(_)), _) => return,
            _ => panic!(),
        }
    }
    #[test]
    fn town_next_ctx_mm_is_town_mm() {
        let ctx = context_with_town_in_range();

        let expected_mm = ctx.towns[0].mm;

        let state = Event::Hike(HikeState);
        match state.next_state(ctx) {
            (_, ctx) => assert_eq!(ctx.mm, expected_mm),
        }
    }
    #[test]
    fn town_next_ctx_time_increases() {
        let ctx = context_with_town_in_range();

        let distance = ctx.towns[0].mm - ctx.mm;
        let expected_time = ctx.time + distance / ctx.speed;

        let state = Event::Hike(HikeState);
        match state.next_state(ctx) {
            (_, ctx) => assert_eq!(ctx.time, expected_time),
        }
    }
}
