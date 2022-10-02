use std::sync::RwLock;

use once_cell::sync::OnceCell;

/* global settings that are loaded once on start. For any changes, restart. */
#[derive(Default, Debug)]
pub struct TbSettings {
    autohide: bool,
    sleep_time_in_ms: u64,
}

impl TbSettings {
    fn new() -> Self {
        //todo load in yaml file
        TbSettings {
            autohide: false,
            sleep_time_in_ms: 1000,
        }
    }
}

fn get_settings() -> &'static RwLock<TbSettings> {
    static INSTANCE: OnceCell<RwLock<TbSettings>> = OnceCell::new();
    INSTANCE.get_or_init(|| RwLock::new(TbSettings::new()))
}

pub fn get_sleep_time_in_ms() -> u64 {
    if let Ok(settings) = get_settings().read() {
        return settings.sleep_time_in_ms;
    }
    100
}

pub fn get_autohide() -> bool {
    if let Ok(settings) = get_settings().read() {
        return settings.autohide;
    }
    false
}
