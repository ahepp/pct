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
    EnterTown(EnterTownState),
    Hike(HikeState),
    Wake(WakeState),
}

pub struct Context {
    pub bedtime: f32,
    pub day: u32,
    pub mm: f32,
    pub speed: f32,
    pub time: f32,
    pub town_idx: usize,
    pub towns: Vec<Town>,
    pub waketime: f32,
}
impl Context {
    pub fn new() -> Self {
        let towns = vec![
            Town {
                name: "Campo".to_string(),
                mm: 1.4,
            },
            Town {
                name: "Lake Morena".to_string(),
                mm: 20.0,
            },
            Town {
                name: "Mount Laguna".to_string(),
                mm: 41.5,
            },
        ];
        Context {
            bedtime: 20.0,
            day: 0,
            mm: 0.0,
            speed: 2.0,
            time: 0.0,
            town_idx: 0,
            towns,
            waketime: 8.0,
        }
    }
}

pub struct Town {
    pub name: String,
    pub mm: f32,
}
