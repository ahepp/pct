pub struct Context {
    pub mm: f32,
    pub day: u32,
    pub speed: f32,
    pub waketime: f32,
    pub bedtime: f32,
}
impl Context {
    pub fn new() -> Self {
        Context {
            mm: 0.0,
            day: 0,
            speed: 1.0,
            waketime: 8.0,
            bedtime: 20.0,
        }
    }
}
