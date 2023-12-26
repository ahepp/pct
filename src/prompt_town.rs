use crate::begin_hiking::BeginHiking;
use crate::context::Context;
use crate::end_game::EndGame;
use crate::state::State;
use crate::util::*;

pub struct PromptTown {
    ctx: Context,
}
impl PromptTown {
    pub fn new(ctx: Context) -> Self {
        PromptTown { ctx }
    }
}
impl State for PromptTown {
    fn render(&self) {
        match distance_to_town(&self.ctx) {
            Some(distance) => {
                let hours_to_town = distance / self.ctx.speed;
                let time_today = self.ctx.bedtime - self.ctx.time;
                let time_per_day = self.ctx.bedtime - self.ctx.waketime;
                let days_of_food = (hours_to_town - time_today)
                    .clamp(0.0, f32::INFINITY)
                    / time_per_day;
                println!(
                    "You are {} days from the next town",
                    days_of_food.ceil()
                );
            }
            None => {}
        }

        let days_of_food = self.ctx.food / self.ctx.food_per_day;
        println!("You are {} days from running out of food", days_of_food);

        let actions = vec![Action::Continue, Action::Quit];
        for (i, action) in actions.into_iter().enumerate() {
            println!("{}) {}", i, action);
        }
        println!("What do you want to do? ");
    }
    fn next_state(self: Box<Self>) -> Option<Box<dyn State>> {
        let actions = vec![Action::Continue, Action::Quit];
        let action = loop {
            let i = retry_usize();
            if i < actions.len() {
                break actions[i].clone();
            }
            println!("You can't do that");
        };
        match action {
            Action::Continue => Some(Box::new(BeginHiking::new(self.ctx))),
            Action::Quit => Some(Box::new(EndGame)),
        }
    }
}

#[derive(Clone)]
enum Action {
    Continue,
    Quit,
}
impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = match self {
            Action::Continue => "continue",
            Action::Quit => "quit",
        };
        write!(f, "{}", name)
    }
}
