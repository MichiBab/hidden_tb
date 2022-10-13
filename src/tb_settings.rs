use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

const FILE_NAME: &str = "settings.json";

/* global settings that are loaded once on start. For any changes, restart. */
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
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
    pub fn new() -> Self {
        match Self::try_load() {
            Some(settings) => {
                println!("settings loaded from file");
                return settings;
            }
            None => {
                let defaults = Self::load_defaults();
                defaults.save().ok();
                return defaults;
            }
        }
    }

    fn load_defaults() -> TbSettings {
        TbSettings {
            autohide: true,
            sleep_time_in_ms: 15,
            animation_time_in_ms: 12,
            animation_steps: 6,
            infrequent_count: 60,
            tb_rect_detection_size_in_pixel: 2,
            tb_rect_bottom_offset: 1,
        }
    }

    pub fn get_animation_time_in_ms(&self) -> u64 {
        self.animation_time_in_ms
    }

    #[allow(dead_code)]
    pub fn set_animation_time_in_ms(&mut self, value: u64) {
        if self.animation_time_in_ms == value {
            return;
        }
        self.animation_time_in_ms = value;
        self.try_save();
    }

    pub fn get_tb_rect_detection_size_in_pixel(&self) -> i32 {
        self.tb_rect_detection_size_in_pixel
    }

    #[allow(dead_code)]
    pub fn set_tb_rect_detection_size_in_pixel(&mut self, value: i32) {
        if self.tb_rect_detection_size_in_pixel == value {
            return;
        }
        self.tb_rect_detection_size_in_pixel = value;
        self.try_save();
    }

    pub fn get_tb_rect_bottom_offset(&self) -> i32 {
        self.tb_rect_bottom_offset
    }

    #[allow(dead_code)]
    pub fn set_tb_rect_bottom_offset(&mut self, value: i32) {
        if self.tb_rect_bottom_offset == value {
            return;
        }
        self.tb_rect_bottom_offset = value;
        self.try_save();
    }

    pub fn get_animation_steps(&self) -> u8 {
        self.animation_steps
    }

    #[allow(dead_code)]
    pub fn set_animation_steps(&mut self, value: u8) {
        if self.animation_steps == value {
            return;
        }
        self.animation_steps = value;
        self.try_save();
    }

    pub fn get_infrequent_count(&self) -> usize {
        self.infrequent_count
    }

    #[allow(dead_code)]
    pub fn set_infrequent_count(&mut self, value: usize) {
        if self.infrequent_count == value {
            return;
        }
        self.infrequent_count = value;
        self.try_save();
    }

    pub fn get_autohide(&self) -> bool {
        self.autohide
    }

    #[allow(dead_code)]
    pub fn set_autohide(&mut self, value: bool) {
        if self.autohide == value {
            return;
        }
        self.autohide = value;
        self.try_save();
    }

    pub fn get_sleep_time_in_ms(&self) -> u64 {
        self.sleep_time_in_ms
    }

    #[allow(dead_code)]
    pub fn set_sleep_time_in_ms(&mut self, value: u64) {
        if self.sleep_time_in_ms == value {
            return;
        }
        self.sleep_time_in_ms = value;
        self.try_save();
    }

    fn try_save(&self) {
        self.save().ok();
    }

    fn try_load() -> Option<TbSettings> {
        if TbSettings::check_if_file_exists() {
            match Self::load() {
                Ok(val) => return Some(val),
                Err(_) => Self::delete_file(),
            };
        }
        None
    }

    fn delete_file() {
        std::fs::remove_file(FILE_NAME).ok();
    }

    fn save(&self) -> Result<(), Box<dyn Error>> {
        let path = Self::get_path();
        let file = File::create(path)?;
        serde_json::to_writer_pretty(file, &self)?;
        println!("settings saved");
        Ok(())
    }

    fn load() -> Result<TbSettings, Box<dyn Error>> {
        let path = Self::get_path();
        let file = File::open(path)?;
        let settings: TbSettings = serde_json::from_reader(file)?;
        Ok(settings)
    }

    fn check_if_file_exists() -> bool {
        let path = Self::get_path();
        path.exists()
    }

    fn get_path() -> PathBuf {
        let mut rsrc_dir = std::env::current_exe().expect("Can't find path to executable");
        rsrc_dir.pop();
        rsrc_dir.push(FILE_NAME);
        rsrc_dir
    }
}
