#![windows_subsystem = "windows"]

use std::{thread, time};
use taskbar::Taskbar;
mod monitors;
mod settings_ui;
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
    let mut infrequent_counter: usize = 0;
    let signaling = signaling::get_signaling_struct();
    //spawn system tray icon
    let ui_handle = std::thread::spawn(|| -> () {
        tray::start_tray_icon();
    });
    // wait until all handles are available
    while taskbar.contains_none() && !signaling.get_exit_called() {
        taskbar.print_which_is_none();
        eprintln!("Waiting for handles...");
        thread::sleep(time::Duration::from_millis(100));
        taskbar.refresh_handles();
        continue;
    }

    println!("got handles, starting tb");

    loop {
        if signaling.get_exit_called() {
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
        thread::sleep(dur);
    }
    taskbar.clean_up();
    ui_handle.join().expect("tray thread finished");

    if signaling.get_settings_called() {
        settings_ui::open_ui();
    }
}
