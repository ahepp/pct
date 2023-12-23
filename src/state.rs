pub trait State {
    fn render(&self);
    fn next_state(self: Box<Self>) -> Option<Box<dyn State>>;
}
