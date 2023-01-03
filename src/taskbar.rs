use crate::tb_settings::{self, TbSettings};
use crate::windows_calls::{self, TaskbarData, WantedHwnds, _ALWAYS_ON_TOP};
/*  */
#[derive(Default, Debug)]
pub struct Taskbar {
    settings: tb_settings::TbSettings,
    taskbar_data: windows_calls::TaskbarData,
    last_taskbar_data: windows_calls::TaskbarData,
    is_hidden: bool,
    step_value: u8,
    tray_shown_currently: bool,
}

impl Taskbar {
    pub fn new() -> Self {
        let settings = TbSettings::new();
        let step_value = 255 / settings.get_animation_steps();
        let wanted_hwnds = WantedHwnds::new(&settings);
        let tb_data = windows_calls::TaskbarData::new(&wanted_hwnds);
        Taskbar {
            last_taskbar_data: TaskbarData::default(),
            taskbar_data: tb_data,
            settings,
            step_value,
            is_hidden: false,
            tray_shown_currently: false,
        }
    }

    pub fn refresh_handles(&mut self) {
        let taskbar_data = windows_calls::TaskbarData::new(&WantedHwnds::new(&self.settings));
        self.taskbar_data = taskbar_data.clone();
        self.last_taskbar_data = taskbar_data;
    }

    pub fn fetch_new_handles(&self) -> TaskbarData {
        windows_calls::TaskbarData::new(&WantedHwnds::new(&self.settings))
    }

    /* calls on_new_handles to update all routines that have to react on new handles. */
    pub fn insert_handles(&mut self, new_tb_data: TaskbarData) {
        self.last_taskbar_data = self.taskbar_data.clone();
        self.taskbar_data = new_tb_data;
        self.on_new_handles();
    }

    pub fn refresh_area_and_set_on_top(&self) {
        if let Some(taskbar) = &self.taskbar_data.taskbar {
            windows_calls::set_handle_to_topmost(&taskbar.hwnd);
            windows_calls::set_app_bar_state(&taskbar.hwnd, _ALWAYS_ON_TOP);
        }
    }

    pub fn contains_none(&self) -> bool {
        self.taskbar_data.contains_none()
    }

    pub fn print_which_is_none(&self) {
        let mut none = "[ ".to_string();
        if self.taskbar_data.applist.is_none() && self.taskbar_data.wanted_hwnds.applist {
            none = format!("{}applist ", none);
        }
        if self.taskbar_data.apps.is_none() && self.taskbar_data.wanted_hwnds.apps {
            none = format!("{}; apps ", none);
        }
        if self.taskbar_data.rebar.is_none() && self.taskbar_data.wanted_hwnds.rebar {
            none = format!("{}; rebar ", none);
        }
        if self.taskbar_data.tray.is_none() && self.taskbar_data.wanted_hwnds.tray {
            none = format!("{}; tray ", none);
        }
        if self.taskbar_data.taskbar.is_none() && self.taskbar_data.wanted_hwnds.taskbar {
            none = format!("{}; taskbar", none);
        }
        none = format!("{} ]", none);
        println!("None: {}", none);
    }

    pub fn is_hovering_on_tray(&self) -> bool {
        if let Some(tray_entry) = &self.taskbar_data.tray {
            if let Some(cursor_pos) = windows_calls::get_cursor_pos() {
                let mut hidden_rect = tray_entry.rect;
                hidden_rect.bottom += self.settings.get_tb_rect_bottom_offset();
                if self.settings.get_autohide() && self.is_hidden {
                    hidden_rect.top = hidden_rect.bottom
                        - self.settings.get_tb_rect_detection_size_in_pixel()
                        - self.settings.get_tb_rect_bottom_offset();
                    return windows_calls::get_point_in_rect(&hidden_rect, &cursor_pos);
                }
                return windows_calls::get_point_in_rect(&hidden_rect, &cursor_pos);
            }
        }
        false
    }

    pub fn is_hovering_on_tb(&self) -> bool {
        let wanted_handle = match self.settings.get_enable_dynamic_borders() {
            true => &self.taskbar_data.applist,
            false => &self.taskbar_data.taskbar,
        };
        if let Some(wanted_entry) = &wanted_handle {
            if let Some(cursor_pos) = windows_calls::get_cursor_pos() {
                let mut hidden_rect = wanted_entry.rect;
                hidden_rect.bottom += self.settings.get_tb_rect_bottom_offset();
                if self.settings.get_enable_dynamic_borders() {
                    let tb_rect = match self.taskbar_data.taskbar {
                        Some(ref tb) => tb.rect,
                        None => return false,
                    };
                    //offset the left rect to include windows and search button etc, which is not contained in the applist handleTuesday, January 3rd, 17:06:29
                    hidden_rect.left = tb_rect.right - wanted_entry.rect.right;
                }
                if self.settings.get_autohide() && self.is_hidden {
                    hidden_rect.top = hidden_rect.bottom
                        - self.settings.get_tb_rect_detection_size_in_pixel()
                        - self.settings.get_tb_rect_bottom_offset();
                    return windows_calls::get_point_in_rect(&hidden_rect, &cursor_pos);
                }
                return windows_calls::get_point_in_rect(&hidden_rect, &cursor_pos);
            }
        }
        false
    }

    fn set_taskbar_alpha(&self, alpha: u8) -> bool {
        if let Some(taskbar_entry) = &self.taskbar_data.taskbar {
            return windows_calls::set_window_alpha(&taskbar_entry.hwnd, alpha);
        }
        false
    }

    pub fn check_and_set_taskbar_transparency_state(&self) -> bool {
        if let Some(taskbar_entry) = &self.taskbar_data.taskbar {
            return windows_calls::check_and_set_transparency_style(&taskbar_entry.hwnd);
        }
        false
    }

    pub fn hide_taskbar(&mut self) {
        let mut alpha: u8 = 255;
        let mut changed = true;
        for step in 0..self.settings.get_animation_steps() {
            alpha = alpha.saturating_sub(self.step_value);
            if step == self.settings.get_animation_steps() - 1 {
                alpha = 0;
            }
            changed = changed && self.set_taskbar_alpha(alpha);
            std::thread::sleep(std::time::Duration::from_millis(
                self.settings.get_animation_time_in_ms(),
            ));
        }
        if changed {
            self.is_hidden = true;
        }
    }

    pub fn show_taskbar(&mut self) {
        let mut alpha: u8 = 0;
        let mut changed = true;

        for step in 0..self.settings.get_animation_steps() {
            alpha = alpha.saturating_add(self.step_value);
            if step == self.settings.get_animation_steps() - 1 {
                alpha = 255;
            }
            changed = changed && self.set_taskbar_alpha(alpha);
            std::thread::sleep(std::time::Duration::from_millis(
                self.settings.get_animation_time_in_ms(),
            ));
        }
        if changed {
            self.is_hidden = false;
        }
    }

    fn merge_tray_with_applist(&mut self) {
        if let Some(tray_entry) = &self.taskbar_data.tray {
            if let Some(apps_entry) = &self.taskbar_data.applist {
                //Todo: maybe can call LockWindowUpdate so it doesnt update the window on pressing the up arrow button on the tray
                windows_calls::move_window_on_tb(&tray_entry.hwnd, apps_entry.rect.right, 0);
            }
        }
    }

    fn merge_widgets_with_applist(&mut self) {
        //TODO
    }

    fn update_dynamic_borders(&mut self) {
        //create region
        let mut nothing_changed = true;
        if let Some(last_applist) = &self.last_taskbar_data.applist {
            if let Some(current_applist) = &self.taskbar_data.applist {
                if last_applist.rect != current_applist.rect {
                    nothing_changed = false;
                }
            }
        }
        if self.settings.get_dynamic_borders_show_tray() {
            if let Some(last_tray) = &self.last_taskbar_data.tray {
                if let Some(current_tray) = &self.taskbar_data.tray {
                    if last_tray.rect != current_tray.rect {
                        nothing_changed = false;
                    }
                }
            }
        }

        if nothing_changed {
            return;
        }
        self.call_dynamic_update(self.is_hovering_on_tray(), false);
    }

    pub fn call_dynamic_update(&mut self, hovering_over_tray: bool, hovering_over_widgets: bool) {
        windows_calls::create_rounded_region(
            &self.settings,
            &self.taskbar_data,
            hovering_over_tray,
            hovering_over_widgets,
        );
    }

    pub fn on_new_handles(&mut self) {
        if self.settings.get_merge_tray() {
            self.merge_tray_with_applist();
        }
        if self.settings.get_merge_widgets() {
            self.merge_widgets_with_applist();
        }
        if self.settings.get_enable_dynamic_borders() {
            self.update_dynamic_borders();
        }
    }

    pub fn handle_taskbar_state(&mut self) {
        let start_menu_open = windows_calls::get_start_menu_open();

        /* for autohiding tray logic */
        if !self.settings.get_dynamic_borders_show_tray()
            && self
                .settings
                .get_dynamic_borders_show_tray_if_disabled_on_hover()
            && self.settings.get_enable_dynamic_borders()
        {
            if start_menu_open {
                self.tray_shown_currently = true;
                self.call_dynamic_update(true, false);
            } else if self.is_hovering_on_tray() {
                if !self.tray_shown_currently {
                    self.tray_shown_currently = true;
                    self.call_dynamic_update(true, false);
                }
            } else if self.tray_shown_currently {
                self.tray_shown_currently = false;
                self.call_dynamic_update(false, false);
            }
        }

        if !self.settings.get_autohide() {
            return;
        }

        let is_hovering = self.is_hovering_on_tb();

        if start_menu_open
            || is_hovering
            || (self.settings.get_enable_dynamic_borders() && self.is_hovering_on_tray())
        {
            if self.is_hidden {
                self.show_taskbar();
            }
        } else if !self.is_hidden {
            self.hide_taskbar();
        }
    }

    pub fn clean_up(&mut self) {
        if let Some(taskbar_data) = &self.taskbar_data.taskbar {
            windows_calls::reset_taskbar(&taskbar_data.hwnd, &taskbar_data.rect);
        }
    }
}
