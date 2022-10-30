//#![windows_subsystem = "windows"]

use std::{ thread, time };
use taskbar::Taskbar;

use crate::tb_settings::TbSettings;
mod monitors;
mod settings_ui;
mod signaling;
mod taskbar;
mod tb_settings;
mod tray;
mod windows_calls;

#[inline(always)]
fn update_handles_of_tb(taskbar: &mut Taskbar) {
    let new_handles = taskbar.fetch_new_handles();
    if !new_handles.contains_none() {
        taskbar.insert_handles(new_handles);
    }
}

#[inline(always)]
fn infrequent_routine(
    settings: &TbSettings,
    taskbar: &mut Taskbar,
    update_handles_in_infrequent_routine: &bool
) {
    if settings.get_autohide() || settings.get_enable_dynamic_borders() {
        taskbar.check_and_set_taskbar_transparency_state();
        if settings.get_autohide() {
            windows_calls::check_and_update_workspace_region_for_autohide(&taskbar);
        }
        if settings.get_enable_dynamic_borders() {
            taskbar.call_dynamic_update(None, None);
        }
    }
    if *update_handles_in_infrequent_routine {
        update_handles_of_tb(taskbar);
    }
}

fn start_hidden_tb() {
    let settings = TbSettings::new();
    let dur = time::Duration::from_millis(settings.get_sleep_time_in_ms());
    let mut taskbar = Taskbar::new();
    let mut infrequent_counter: usize = 0;
    let signaling = signaling::get_signaling_struct();
    //spawn system tray icon
    let ui_handle = std::thread::spawn(
        || -> () {
            tray::start_tray_icon();
        }
    );
    // wait until all handles are available
    while taskbar.contains_none() && !signaling.get_exit_called() {
        taskbar.print_which_is_none();
        eprintln!("Waiting for handles...");
        thread::sleep(time::Duration::from_millis(100));
        taskbar.refresh_handles();
        continue;
    }

    println!("got handles, starting tb");

    //handles have to be updated on every loop if a merging option is enabled, to react to applist changes.
    let update_handles_in_infrequent_routine = !(
        settings.get_merge_tray() ||
        settings.get_merge_widgets() ||
        settings.get_enable_dynamic_borders()
    );

    loop {
        if signaling.get_exit_called() {
            break;
        }

        infrequent_counter %= settings.get_infrequent_count();
        if infrequent_counter == 0 {
            infrequent_routine(&settings, &mut taskbar, &update_handles_in_infrequent_routine);
        }

        if !update_handles_in_infrequent_routine {
            update_handles_of_tb(&mut taskbar);
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

fn main() {
    windows_calls::initialize_windows_calls();

    loop {
        start_hidden_tb();
        //check if settings were called with reset
        if !signaling::get_signaling_struct().get_reset_called() {
            return;
        } else {
            signaling::get_signaling_struct().set_reset_called(false);
            signaling::get_signaling_struct().set_exit_called(false);
            signaling::get_signaling_struct().set_settings_called(false);
        }
    }
}