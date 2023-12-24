pub struct Context {
    pub day: u32,
    pub waketime: f32,
}
impl Context {
    pub fn new() -> Self {
        Context {
            day: 0,
            waketime: 8.0,
        }
    }
}
