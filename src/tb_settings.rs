use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

const FILE_NAME: &str = "settings.json";

/* global settings that are loaded once on start. For any changes, restart. */
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct TbSettings {
    autohide: bool,
    merge_tray: bool,
    merge_widgets: bool,
    sleep_time_in_ms: u64,
    animation_time_in_ms: u64,
    animation_steps: u8,
    infrequent_count: usize,
    tb_rect_bottom_offset: i32,
    tb_rect_detection_size_in_pixel: i32,
    enable_dynamic_borders: bool,
    dynamic_borders_show_tray: bool,
    dynamic_borders_show_widgets: bool,
    rounded_corners_size: i32,
    margin_left: i32,
    margin_right: i32,
    margin_bottom: i32,
    margin_top: i32,
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
            merge_tray: false,
            merge_widgets: false,
            sleep_time_in_ms: 15,
            animation_time_in_ms: 12,
            animation_steps: 6,
            infrequent_count: 60,
            tb_rect_detection_size_in_pixel: 2,
            tb_rect_bottom_offset: 1,
            enable_dynamic_borders: true,
            dynamic_borders_show_tray: false,
            dynamic_borders_show_widgets: false,
            rounded_corners_size: 0,
            margin_left: 0,
            margin_right: 0,
            margin_bottom: 0,
            margin_top: 0,
        }
    }

    pub fn get_margin_top(&self) -> i32 {
        self.margin_top
    }

    pub fn set_margin_top(&mut self, value: i32) {
        if self.margin_top == value {
            return;
        }
        self.margin_top = value;
        self.try_save();
    }

    pub fn get_margin_bottom(&self) -> i32 {
        self.margin_bottom
    }

    pub fn set_margin_bottom(&mut self, value: i32) {
        if self.margin_bottom == value {
            return;
        }
        self.margin_bottom = value;
        self.try_save();
    }

    pub fn get_margin_left(&self) -> i32 {
        self.margin_left
    }

    pub fn set_margin_left(&mut self, value: i32) {
        if self.margin_left == value {
            return;
        }
        self.margin_left = value;
        self.try_save();
    }

    pub fn get_margin_right(&self) -> i32 {
        self.margin_right
    }

    pub fn set_margin_right(&mut self, value: i32) {
        if self.margin_right == value {
            return;
        }
        self.margin_right = value;
        self.try_save();
    }

    pub fn get_enable_dynamic_borders(&self) -> bool {
        self.enable_dynamic_borders
    }

    pub fn set_enable_dynamic_borders(&mut self, value: bool) {
        if self.enable_dynamic_borders == value {
            return;
        }
        self.enable_dynamic_borders = value;
        self.try_save();
    }

    pub fn get_dynamic_borders_show_tray(&self) -> bool {
        self.dynamic_borders_show_tray
    }

    pub fn set_dynamic_borders_show_tray(&mut self, value: bool) {
        if self.dynamic_borders_show_tray == value {
            return;
        }
        self.dynamic_borders_show_tray = value;
        self.try_save();
    }

    pub fn get_dynamic_borders_show_widgets(&self) -> bool {
        self.dynamic_borders_show_widgets
    }

    pub fn set_dynamic_borders_show_widgets(&mut self, value: bool) {
        if self.dynamic_borders_show_widgets == value {
            return;
        }
        self.dynamic_borders_show_widgets = value;
        self.try_save();
    }

    pub fn get_rounded_corners_size(&self) -> i32 {
        self.rounded_corners_size
    }

    pub fn set_rounded_corners_size(&mut self, value: i32) {
        if self.rounded_corners_size == value {
            return;
        }
        self.rounded_corners_size = value;
        self.try_save();
    }

    pub fn get_animation_time_in_ms(&self) -> u64 {
        self.animation_time_in_ms
    }

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

    pub fn get_merge_tray(&self) -> bool {
        self.merge_tray
    }

    pub fn set_merge_tray(&mut self, value: bool) {
        if self.merge_tray == value {
            return;
        }
        self.merge_tray = value;
        self.try_save();
    }

    pub fn get_merge_widgets(&self) -> bool {
        self.merge_widgets
    }

    pub fn set_merge_widgets(&mut self, value: bool) {
        if self.merge_widgets == value {
            return;
        }
        self.merge_widgets = value;
        self.try_save();
    }

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
