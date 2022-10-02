use std::sync::RwLock;

use once_cell::sync::OnceCell;

/* global settings that are loaded once on start. For any changes, restart. */
#[derive(Default, Debug)]
pub struct TbSettings {
    autohide: bool,
}

impl TbSettings {
    fn new() -> Self {
        //todo load in yaml file
        TbSettings::default()
    }

    pub fn get_autohide(&self) -> bool {
        self.autohide
    }
}

pub fn get_settings() -> &'static RwLock<TbSettings> {
    static INSTANCE: OnceCell<RwLock<TbSettings>> = OnceCell::new();
    INSTANCE.get_or_init(|| RwLock::new(TbSettings::new()))
}
