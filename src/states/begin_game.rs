use crate::states::*;

pub struct BeginGameState;
impl State for BeginGameState {
    fn render(&self, _ctx: &Context) {
        println!("Welcome to the Pacific Crest Trail");
        crate::util::render_delay();
    }
    fn next_state(self, ctx: Context) -> (Option<Event>, Context) {
        (Some(Event::Wake(WakeState)), ctx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_event_none() {
        let ctx = Context { day: 0 };
        let state = Event::BeginGame(BeginGameState);
        match state.next_state(ctx) {
            (Some(Event::Wake(_)), _) => return,
            _ => panic!(),
        }
    }
}
