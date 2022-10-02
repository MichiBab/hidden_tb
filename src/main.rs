use std::{thread, time};
use taskbar::Taskbar;

mod taskbar;
mod tb_settings;
mod windows_calls;
fn main() {
    windows_calls::initialize_windows_calls();
    let taskbar = Taskbar::new();
    let sleep_in_ms = time::Duration::from_millis(tb_settings::get_sleep_time_in_ms());

    loop {
        thread::sleep(sleep_in_ms);
        let is_hovering = taskbar.is_hovering_on_tb();
        let start_menu_open = windows_calls::get_start_menu_open();
        println!("{is_hovering}");
        println!("{start_menu_open}");
        println!("------------------");
    }
}
