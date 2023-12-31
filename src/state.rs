use crate::context::Context;
use crate::scene::Scene;

pub trait State: Scene {
    fn next_state(&self, ctx: Context) -> (Option<Box<dyn State>>, Context);
}
