use crate::context::Context;
use crate::scene::Scene;
use crate::states::BeginGame;

impl Scene for BeginGame {
    fn render(&self, _ctx: &Context) {
        println!("Welcome to the Pacific Crest Trail!");
    }
}
