use crate::tb_settings;
use crate::windows_calls::{self, _ALWAYS_ON_TOP};
/*  */
#[derive(Default, Debug)]
pub struct Taskbar {
    settings: tb_settings::TbSettings,
    taskbar_data: windows_calls::TaskbarData,
    is_hidden: bool,
    step_value: u8,
}

impl Taskbar {
    pub fn new() -> Self {
        let settings = tb_settings::get_tb_settings();
        let step_value = 255 / settings.get_animation_steps();

        Taskbar {
            taskbar_data: windows_calls::TaskbarData::new(),
            settings,
            step_value,
            is_hidden: false,
        }
    }

    pub fn refresh_handles(&mut self) {
        self.taskbar_data = windows_calls::TaskbarData::new();
    }

    pub fn refresh_area_and_set_on_top(&self) {
        if let Some(taskbar) = &self.taskbar_data.taskbar {
            windows_calls::set_handle_to_topmost(&taskbar.hwnd);
            windows_calls::set_app_bar_state(&taskbar.hwnd, _ALWAYS_ON_TOP);
        }
    }

    pub fn contains_none(&self) -> bool {
        self.taskbar_data.applist.is_none()
            || self.taskbar_data.apps.is_none()
            || self.taskbar_data.rebar.is_none()
            || self.taskbar_data.tray.is_none()
            || self.taskbar_data.taskbar.is_none()
    }

    pub fn is_hovering_on_tb(&self) -> bool {
        if let Some(taskbar_entry) = &self.taskbar_data.taskbar {
            if let Some(cursor_pos) = windows_calls::get_cursor_pos() {
                if self.settings.get_autohide() && self.is_hidden {
                    let mut hidden_rect = taskbar_entry.rect.clone();
                    hidden_rect.top = hidden_rect.bottom - 1;
                    return windows_calls::get_point_in_rect(&hidden_rect, &cursor_pos);
                }
                return windows_calls::get_point_in_rect(&taskbar_entry.rect, &cursor_pos);
            }
        }
        false
    }

    fn set_taskbar_alpha(&self, alpha: u8) {
        if let Some(taskbar_entry) = &self.taskbar_data.taskbar {
            windows_calls::set_window_alpha(&taskbar_entry.hwnd, alpha);
        }
    }

    pub fn hide_taskbar(&mut self) {
        let mut alpha: u8 = 255;
        for step in 0..self.settings.get_animation_steps() {
            alpha = alpha.saturating_sub(self.step_value);
            if step == self.settings.get_animation_steps() - 1 {
                alpha = 0;
            }
            self.set_taskbar_alpha(alpha);
            std::thread::sleep(std::time::Duration::from_millis(
                self.settings.get_animation_time_in_ms(),
            ));
        }
        self.is_hidden = true;
    }

    pub fn show_taskbar(&mut self) {
        let mut alpha: u8 = 0;
        for step in 0..self.settings.get_animation_steps() {
            alpha = alpha.saturating_add(self.step_value);
            if step == self.settings.get_animation_steps() - 1 {
                alpha = 255;
            }
            self.set_taskbar_alpha(alpha);
            std::thread::sleep(std::time::Duration::from_millis(
                self.settings.get_animation_time_in_ms(),
            ));
        }
        self.is_hidden = false;
    }

    pub fn handle_taskbar_state(&mut self) {
        if !self.settings.get_autohide() {
            return;
        }

        let start_menu_open = windows_calls::get_start_menu_open();
        let is_hovering = self.is_hovering_on_tb();

        if start_menu_open || is_hovering {
            if self.is_hidden {
                self.show_taskbar();
            }
        } else {
            if !self.is_hidden {
                self.hide_taskbar();
            }
        }
    }
}
