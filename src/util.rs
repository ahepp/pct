pub fn render_timestamp(time: f32) {
    let hours = time as u32;
    let minutes = ((time * 60.0) as u32) % 60;
    print!("{:02}:{:02} ", hours, minutes);
}

pub fn render_delay() {
    std::thread::sleep(std::time::Duration::from_millis(1000));
}
