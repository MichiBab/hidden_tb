use once_cell::sync::OnceCell;
use std::error::Error;
use std::{fs, sync::RwLock};
type DynErr = Box<dyn Error>;

const FILE_NAME: &str = "settings.json";

/* global settings that are loaded once on start. For any changes, restart. */
#[derive(Default, Debug, Clone)]
pub struct TbSettings {
    autohide: bool,
    sleep_time_in_ms: u64,
    animation_time_in_ms: u64,
    animation_steps: u8,
    infrequent_count: usize,
    tb_rect_bottom_offset: i32,
    tb_rect_detection_size_in_pixel: i32,
}

impl TbSettings {
    fn new() -> Self {
        //todo load in yaml file
        TbSettings {
            autohide: true,
            sleep_time_in_ms: 15,
            animation_time_in_ms: 20,
            animation_steps: 4,
            infrequent_count: 60,
            tb_rect_detection_size_in_pixel: 2,
            tb_rect_bottom_offset: 1,
        }
    }

    pub fn get_animation_time_in_ms(&self) -> u64 {
        self.animation_time_in_ms
    }

    pub fn tb_rect_detection_size_in_pixel(&self) -> i32 {
        self.tb_rect_detection_size_in_pixel
    }

    pub fn get_tb_rect_bottom_offset(&self) -> i32 {
        self.tb_rect_bottom_offset
    }

    pub fn get_animation_steps(&self) -> u8 {
        self.animation_steps
    }

    pub fn get_infrequent_count(&self) -> usize {
        self.infrequent_count
    }

    pub fn get_autohide(&self) -> bool {
        self.autohide
    }

    pub fn get_sleep_time_in_ms(&self) -> u64 {
        self.sleep_time_in_ms
    }

    pub fn serialize(&self) -> Result<(), Box<dyn Error>> {
        //check if file exists
        //if it does, try to delete it
        //serialize settings as json

        Ok(())
    }

    pub fn load(&mut self) -> Result<(), DynErr> {
        //check if file exists
        //if it doesnt, load defaults and serialize them

        //if it does, load file to string
        //deserialize string to settings
        //if it fails, try to delete file
        //load defaults and serialize them
        Ok(())
    }

    pub fn load_defaults() -> TbSettings {
        let settings = TbSettings {
            autohide: true,
            sleep_time_in_ms: 15,
            animation_time_in_ms: 20,
            animation_steps: 4,
            infrequent_count: 60,
            tb_rect_detection_size_in_pixel: 2,
            tb_rect_bottom_offset: 1,
        };
        settings.serialize().ok();
        settings
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
