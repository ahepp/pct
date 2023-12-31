use crate::context::Context;
use crate::scene::Scene;
use crate::scenes::util;
use crate::states::StartHiking;

impl Scene for StartHiking {
    fn render(&self, _ctx: &Context) {
        println!("You started hiking");
        util::render_delay();
    }
}
