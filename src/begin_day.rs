use crate::context::Context;
use crate::state::State;

pub struct BeginDay {
    ctx: Context,
}
impl BeginDay {
    pub fn new(ctx: Context) -> Self {
        BeginDay { ctx }
    }
}
impl State for BeginDay {
    fn render(&self) {
        let day_number = self.ctx.day + 1;
        println!("\n{:>5.2} You began day {}", self.ctx.waketime, day_number);
    }
    fn next_state(self: Box<Self>) -> Option<Box<dyn State>> {
        let day = self.ctx.day + 1;

        let mut next_ctx = self.ctx;
        next_ctx.day = day;
        Some(Box::new(BeginDay::new(next_ctx)))
    }
}
