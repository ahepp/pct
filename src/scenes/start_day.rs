use crate::context::Context;
use crate::scene::Scene;
use crate::states::StartDay;

impl Scene for StartDay {
    fn render(&self, ctx: &Context) {
        let day_number = ctx.day + 1;
        println!("You started day {} at {:.2}", day_number, ctx.mm);
        util::render_delay();
    }
}
