#![windows_subsystem = "windows"]

use std::{thread, time};
use taskbar::Taskbar;
mod monitors;
mod signaling;
mod taskbar;
mod tb_settings;
mod tray;
mod windows_calls;

fn main() {
    windows_calls::initialize_windows_calls();
    let settings = tb_settings::get_tb_settings();
    let dur = time::Duration::from_millis(settings.get_sleep_time_in_ms());
    let mut taskbar = Taskbar::new();
    let ui_handle = std::thread::spawn(|| -> () {
        tray::start_tray_icon();
    });
    let mut infrequent_counter: usize = 0;
    loop {
        if signaling::get_exit_called() {
            break;
        }
        thread::sleep(dur);
        if taskbar.contains_none() {
            taskbar.refresh_handles();
            continue;
        }

        infrequent_counter %= settings.get_infrequent_count();

        if infrequent_counter == 0 {
            if settings.get_autohide() {
                taskbar.check_and_set_taskbar_transparency_state();
                windows_calls::check_and_update_workspace_region_for_autohide(&taskbar);
            }
            taskbar.refresh_handles();
            if taskbar.contains_none() {
                taskbar.refresh_handles();
                continue;
            }
        }

        taskbar.handle_taskbar_state();

        infrequent_counter += 1;
    }
    taskbar.clean_up();
    ui_handle.join().expect("ui thread finished");
}
