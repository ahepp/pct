use crate::begin_day::BeginDay;
use crate::context::Context;
use crate::end_game::EndGame;
use crate::state::State;
use crate::util::*;

pub struct BeginMeal {
    ctx: Context,
}
impl BeginMeal {
    pub fn new(ctx: Context) -> Self {
        BeginMeal { ctx }
    }
}
impl State for BeginMeal {
    fn render(&self) {
        let food = self.ctx.food - self.ctx.food_per_day;
        println!(
            "{:>5.2} You stopped for a meal at mi {:.2}",
            self.ctx.time, self.ctx.mm
        );
        if food < 0.0 {
            println!("You don't have enough food to eat!");
        } else {
            println!("You ate {:.2} food", self.ctx.food_per_day);
            println!("You have {:.2} food remaining", food);
        }
        std::thread::sleep(std::time::Duration::from_millis(RENDER_DELAY_MS));
    }
    fn next_state(self: Box<Self>) -> Option<Box<dyn State>> {
        let food = self.ctx.food - self.ctx.food_per_day;

        if food < 0.0 {
            Some(Box::new(EndGame))
        } else {
            let mut next_ctx = self.ctx;
            next_ctx.food = food;
            Some(Box::new(BeginDay::new(next_ctx)))
        }
    }
}
