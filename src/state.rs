use crate::states::*;

use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait State {
    fn render(&self, ctx: &Context);
    fn next_state(self, ctx: Context) -> (Option<Event>, Context);
}

#[enum_dispatch(State)]
pub enum Event {
    BeginGame(BeginGameState),
    Wake(WakeState),
}

pub struct Context {
    pub day: u32,
}
