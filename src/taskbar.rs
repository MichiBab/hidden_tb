use crate::{
    tb_settings,
    windows_calls::{self},
};

/*  */
#[derive(Default, Debug)]
pub struct Taskbar {
    taskbar_data: windows_calls::TaskbarData,
    is_hidden: bool,
}

impl Taskbar {
    pub fn new() -> Self {
        Taskbar {
            taskbar_data: windows_calls::TaskbarData::new(),
            ..Taskbar::default()
        }
    }

    pub fn contains_none(&self) -> bool {
        self.taskbar_data.applist.is_none()
            || self.taskbar_data.apps.is_none()
            || self.taskbar_data.rebar.is_none()
            || self.taskbar_data.tray.is_none()
            || self.taskbar_data.taskbar.is_none()
    }

    pub fn is_hovering_on_tb(&self, autohide: &bool) -> bool {
        if let Some(taskbar_entry) = &self.taskbar_data.taskbar {
            if let Some(cursor_pos) = windows_calls::get_cursor_pos() {
                if *autohide && self.is_hidden {
                    let mut hidden_rect = taskbar_entry.rect.clone();
                    hidden_rect.top = hidden_rect.bottom - 1;
                    return windows_calls::get_point_in_rect(&hidden_rect, &cursor_pos);
                }
                return windows_calls::get_point_in_rect(&taskbar_entry.rect, &cursor_pos);
            }
        }
        false
    }
}
