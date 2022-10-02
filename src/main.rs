use std::{thread, time};
use taskbar::Taskbar;

mod taskbar;
mod tb_settings;
mod windows_calls;
fn main() {
    let taskbar = Taskbar::new();
    let sleep_in_ms = time::Duration::from_millis(tb_settings::get_sleep_time_in_ms());

    loop {
        thread::sleep(sleep_in_ms);
        let is_hovering = taskbar.is_hovering_on_tb();
        println!("{is_hovering}");
    }
}
