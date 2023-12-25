use crate::begin_hiking::BeginHiking;
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
        let time = self.ctx.waketime;
        let day_num = self.ctx.day + 1;
        let mm = self.ctx.mm;
        println!("\n{:>5.2} You began day {} at mi {:.2}", time, day_num, mm);
    }
    fn next_state(self: Box<Self>) -> Option<Box<dyn State>> {
        let day = self.ctx.day + 1;

        let mut next_ctx = self.ctx;
        next_ctx.day = day;
        Some(Box::new(BeginHiking::new(next_ctx)))
    }
}
