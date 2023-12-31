use crate::context::Context;
use crate::state::State;

pub struct StartDay;
impl State for StartDay {
    fn next_state(&self, ctx: Context) -> (Option<Box<dyn State>>, Context) {
        (None, ctx)
    }

    #[cfg(test)]
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
