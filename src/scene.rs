use crate::context::Context;

pub trait Scene {
    fn render(&self, ctx: &Context);
}
