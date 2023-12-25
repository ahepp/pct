use crate::begin_hiking::BeginHiking;
use crate::context::Context;
use crate::state::State;

pub struct BeginTown {
    ctx: Context,
}
impl BeginTown {
    pub fn new(ctx: Context) -> Self {
        BeginTown { ctx }
    }
}
impl State for BeginTown {
    fn render(&self) {
        let time = self.ctx.time;
        let town = &self.ctx.towns[self.ctx.town_idx];
        println!(
            "{:>5.2} You entered town {} at mi {:.2}",
            time, town.name, town.mm
        );
    }
    fn next_state(self: Box<Self>) -> Option<Box<dyn State>> {
        let town_idx = self.ctx.town_idx + 1;

        let mut next_ctx = self.ctx;
        next_ctx.town_idx = town_idx;
        Some(Box::new(BeginHiking::new(next_ctx)))
    }
}
