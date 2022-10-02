use std::{thread, time};
use taskbar::Taskbar;

mod taskbar;
mod tb_settings;
mod windows_calls;
fn main() -> ! {
    windows_calls::initialize_windows_calls();
    let settings = tb_settings::get_tb_settings();
    let sleep_in_ms = time::Duration::from_millis(settings.get_sleep_time_in_ms());

    loop {
        thread::sleep(sleep_in_ms);
        let taskbar = Taskbar::new();
        if taskbar.contains_none() {
            /* the tb refreshed. repeat loop. */
            continue;
        }

        let is_hovering = taskbar.is_hovering_on_tb(&settings.get_autohide());
        let start_menu_open = windows_calls::get_start_menu_open();
        println!("{is_hovering}");
        println!("{start_menu_open}");
        println!("------------------");
    }
}
