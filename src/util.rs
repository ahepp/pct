use crate::context::Context;

pub const RENDER_DELAY_MS: u64 = 1000;

pub fn retry_line() -> String {
    let mut line = String::new();
    match std::io::stdin().read_line(&mut line) {
        Ok(_) => line,
        Err(_) => retry_line(),
    }
}

pub fn retry_usize() -> usize {
    match retry_line().trim().parse() {
        Ok(val) => val,
        Err(_) => retry_usize(),
    }
}

pub fn distance_to_town(ctx: &Context) -> Option<f32> {
    match ctx.town_idx < ctx.towns.len() {
        true => Some(ctx.towns[ctx.town_idx].mm - ctx.mm),
        false => None,
    }
}

pub fn distance_to_hike(ctx: &Context) -> (bool, f32) {
    let range = (ctx.bedtime - ctx.time) * ctx.speed;
    match distance_to_town(ctx) {
        Some(distance) => {
            if !(distance > range) {
                return (true, distance);
            }
        }
        None => {}
    }
    (false, range)
}
