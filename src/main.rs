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
    // wait until all handles are available
    while taskbar.contains_none() {
        taskbar.refresh_handles();
        thread::sleep(dur);
        continue;
    }

    //spawn system tray icon
    let ui_handle = std::thread::spawn(|| -> () {
        tray::start_tray_icon();
    });

    let mut infrequent_counter: usize = 0;
    loop {
        if signaling::get_exit_called() {
            break;
        }

        infrequent_counter %= settings.get_infrequent_count();
        if infrequent_counter == 0 {
            if settings.get_autohide() {
                taskbar.check_and_set_taskbar_transparency_state();
                windows_calls::check_and_update_workspace_region_for_autohide(&taskbar);
            }
            let new_handles = Taskbar::new();
            if !new_handles.contains_none() {
                taskbar.insert_handles(new_handles);
            }
        }

        taskbar.handle_taskbar_state();

        infrequent_counter += 1;
    }
    taskbar.clean_up();
    ui_handle.join().expect("ui thread finished");
}
