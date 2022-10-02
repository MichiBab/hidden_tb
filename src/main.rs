use std::{thread, time};
use taskbar::Taskbar;

mod taskbar;
mod tb_settings;
mod windows_calls;
fn main() {
    let taskbar = Taskbar::new();

    loop {
        let ten_millis = time::Duration::from_millis(100);
        thread::sleep(ten_millis);
        let is_hovering = taskbar.is_hovering_on_tb();
        println!("{is_hovering}");
    }
}
