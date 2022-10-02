use std::sync::RwLock;

use once_cell::sync::OnceCell;

/* global settings that are loaded once on start. For any changes, restart. */
#[derive(Default, Debug, Clone)]
pub struct TbSettings {
    autohide: bool,
    sleep_time_in_ms: u64,
}

impl TbSettings {
    fn new() -> Self {
        //todo load in yaml file
        TbSettings {
            autohide: false,
            sleep_time_in_ms: 10,
        }
    }

    pub fn get_sleep_time_in_ms(&self) -> u64 {
        return self.sleep_time_in_ms;
    }

    pub fn get_autohide(&self) -> bool {
        return self.autohide;
    }
}

pub fn get_tb_settings() -> TbSettings {
    if let Ok(settings_lock) = get_settings_lock().read() {
        return settings_lock.clone();
    }
    TbSettings::default()
}

fn get_settings_lock() -> &'static RwLock<TbSettings> {
    static INSTANCE: OnceCell<RwLock<TbSettings>> = OnceCell::new();
    INSTANCE.get_or_init(|| RwLock::new(TbSettings::new()))
}
