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
            Action::Continue => {
                Some(Box::new(BeginHiking::new(self.ctx)))
            },
            Action::Quit => {
                Some(Box::new(EndGame))
            },
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
