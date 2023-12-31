pub struct Context {
    pub day: u32,
    pub hiking_opts: HikingOpts,
    pub mm: f32,
}

pub struct HikingOpts {
    pub bedtime: f32,
    pub waketime: f32,
    pub speed: f32,
}
