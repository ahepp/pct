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
