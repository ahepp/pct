use crate::begin_day::BeginDay;
use crate::context::Context;
use crate::state::State;

pub struct BeginHiking {
    ctx: Context,
}
impl BeginHiking {
    pub fn new(ctx: Context) -> Self {
        BeginHiking { ctx }
    }
}
impl State for BeginHiking {
    fn render(&self) {
    }
    fn next_state(self: Box<Self>) -> Option<Box<dyn State>> {
        let distance = (self.ctx.bedtime - self.ctx.waketime) * self.ctx.speed;
        let mm = self.ctx.mm + distance;

        let mut next_ctx = self.ctx;
        next_ctx.mm = mm;
        Some(Box::new(BeginDay::new(next_ctx)))
    }
}
