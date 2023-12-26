use crate::context::Context;
use crate::prompt_town::PromptTown;
use crate::state::State;
use crate::util::*;

pub struct PromptFood {
    ctx: Context,
}
impl PromptFood {
    pub fn new(ctx: Context) -> Self {
        PromptFood { ctx }
    }
}
impl State for PromptFood {
    fn render(&self) {
        println!("You have {:.2} food", self.ctx.food);
        println!("You can carry {:.2} food", self.ctx.max_food);
        println!("You eat {:.2} food each day", self.ctx.food_per_day);
        println!("You have ${:.2}", self.ctx.cash);
        println!("You can buy food for ${:.2} each", self.ctx.cash_per_food);
        println!("How much food would you like to buy?");
    }
    fn next_state(self: Box<Self>) -> Option<Box<dyn State>> {
        let (qty, cost) = loop {
            let qty = retry_f32();
            if qty < 0.0 || qty.is_nan() {
                println!("You can't do that!");
                continue;
            }
            if self.ctx.food + qty > self.ctx.max_food {
                println!("You can't carry that much food");
                continue;
            }

            let cost = qty * self.ctx.cash_per_food;
            if cost > self.ctx.cash {
                println!("You can't afford ${:.2} worth of food!", cost);
                continue;
            }
            break (qty, cost);
        };
        let food = self.ctx.food + qty;
        let cash = self.ctx.cash - cost;

        let mut next_ctx = self.ctx;
        next_ctx.food = food;
        next_ctx.cash = cash;
        Some(Box::new(PromptTown::new(next_ctx)))
    }
}
