use crate::context::*;

pub fn create_context() -> Context {
    let hiking_opts = HikingOpts {
        bedtime: 10.1,
        speed: 2.3,
        waketime: 1.9,
    };
    Context {
        day: 501,
        hiking_opts,
        mm: 102.1,
    }
}
