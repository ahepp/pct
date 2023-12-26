use crate::begin_meal::BeginMeal;
use crate::begin_town::BeginTown;
use crate::context::Context;
use crate::state::State;
use crate::util::*;

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
            false => Some(Box::new(BeginMeal::new(next_ctx))),
        }
    }
}
