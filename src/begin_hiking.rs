use crate::begin_day::BeginDay;
use crate::begin_town::BeginTown;
use crate::context::Context;
use crate::state::State;

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

pub struct BeginHiking {
    ctx: Context,
}
impl BeginHiking {
    pub fn new(ctx: Context) -> Self {
        BeginHiking { ctx }
    }
}
impl State for BeginHiking {
    fn render(&self) {}
    fn next_state(self: Box<Self>) -> Option<Box<dyn State>> {
        let (is_town, distance) = distance_to_hike(&self.ctx);
        let mm = self.ctx.mm;
        let time = self.ctx.time + distance / self.ctx.speed;

        let mut next_ctx = self.ctx;
        next_ctx.mm = mm + distance;
        next_ctx.time = time;
        match is_town {
            true => Some(Box::new(BeginTown::new(next_ctx))),
            false => Some(Box::new(BeginDay::new(next_ctx))),
        }
    }
}
