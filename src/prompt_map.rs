use crate::context::Context;
use crate::prompt_town::PromptTown;
use crate::state::State;

pub struct PromptMap {
    ctx: Context,
}
impl PromptMap {
    pub fn new(ctx: Context) -> Self {
        PromptMap { ctx }
    }
}
impl State for PromptMap {
    fn render(&self) {
        for town in &self.ctx.towns {
            let distance = town.mm - self.ctx.mm;
            if (distance >= 0.0) && (distance < 200.00) {
                println!("{:>7.2} mi: {}", distance, town.name);
            }
        }
    }
    fn next_state(self: Box<Self>) -> Option<Box<dyn State>> {
        Some(Box::new(PromptTown::new(self.ctx)))
    }
}
