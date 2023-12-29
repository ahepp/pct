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
    Hike(HikeState),
    Wake(WakeState),
}

pub struct Context {
    pub bedtime: f32,
    pub day: u32,
    pub mm: f32,
    pub speed: f32,
    pub waketime: f32,
}
impl Context {
    pub fn new() -> Self {
        Context {
            bedtime: 20.0,
            day: 0,
            mm: 0.0,
            speed: 2.0,
            waketime: 8.0,
        }
    }
}
