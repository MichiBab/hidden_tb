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

    pub fn is_hovering_on_tb(&self) -> bool {
        if let Some(taskbar_entry) = &self.taskbar_data.taskbar {
            if let Some(cursor_pos) = windows_calls::get_cursor_pos() {
                if let Ok(settings) = tb_settings::get_settings().read() {
                    if settings.get_autohide() && self.is_hidden {
                        let mut hidden_rect = taskbar_entry.rect.clone();
                        hidden_rect.top = hidden_rect.bottom - 1;
                        return windows_calls::get_point_in_rect(&hidden_rect, &cursor_pos);
                    }
                    return windows_calls::get_point_in_rect(&taskbar_entry.rect, &cursor_pos);
                }
            }
        }
        false
    }
}
