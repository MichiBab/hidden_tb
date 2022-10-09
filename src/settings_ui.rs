use crate::tb_settings::{self, TbSettings};
use eframe::egui;
use egui::FontId;

fn load_icon(path: &str) -> eframe::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    eframe::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}

pub fn open_ui() {
    let options = eframe::NativeOptions {
        icon_data: Some(load_icon("hidden_tb.ico")),
        transparent: true,
        max_window_size: Some(egui::vec2(500.0, 700.0)),
        min_window_size: Some(egui::vec2(500.0, 700.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Hidden_TB Settings",
        options,
        Box::new(|cc| {
            let style = egui::Style {
                visuals: egui::Visuals::light(),
                ..egui::Style::default()
            };
            cc.egui_ctx.set_style(style);
            Box::new(MyApp::default())
        }),
    );
}

pub struct TbAccessibleSettings {
    autohide: bool,
    sleep_time_in_ms: u64,
    animation_time_in_ms: u64,
    animation_steps: u8,
    infrequent_count: usize,
    tb_rect_bottom_offset: i32,
    tb_rect_detection_size_in_pixel: i32,
}

impl TbAccessibleSettings {
    fn from(settings: &TbSettings) -> Self {
        Self {
            autohide: settings.get_autohide(),
            sleep_time_in_ms: settings.get_sleep_time_in_ms(),
            animation_time_in_ms: settings.get_animation_time_in_ms(),
            animation_steps: settings.get_animation_steps(),
            infrequent_count: settings.get_infrequent_count(),
            tb_rect_bottom_offset: settings.get_tb_rect_bottom_offset(),
            tb_rect_detection_size_in_pixel: settings.get_tb_rect_detection_size_in_pixel(),
        }
    }

    fn is_equal(&self, settings: &TbSettings) -> bool {
        self.autohide == settings.get_autohide()
            && self.sleep_time_in_ms == settings.get_sleep_time_in_ms()
            && self.animation_time_in_ms == settings.get_animation_time_in_ms()
            && self.animation_steps == settings.get_animation_steps()
            && self.infrequent_count == settings.get_infrequent_count()
            && self.tb_rect_bottom_offset == settings.get_tb_rect_bottom_offset()
            && self.tb_rect_detection_size_in_pixel
                == settings.get_tb_rect_detection_size_in_pixel()
    }
}

struct MyApp {
    global_settings: TbSettings,
    settings: TbAccessibleSettings,
    font_id: FontId,
    small_font_id: FontId,
    info_string: egui::widget_text::RichText,
}

impl Default for MyApp {
    fn default() -> Self {
        let tb_settings = tb_settings::get_tb_settings();
        Self {
            global_settings: tb_settings.clone(),
            settings: TbAccessibleSettings::from(&tb_settings),
            font_id: FontId::monospace(22.0),
            small_font_id: FontId::monospace(17.0),
            info_string: egui::widget_text::RichText::default(),
        }
    }
}

impl MyApp {
    fn formatted_string(&self, str: &str) -> egui::widget_text::RichText {
        egui::RichText::new(str).font(self.font_id.clone())
    }

    fn formatted_small_string(&self, str: &str) -> egui::widget_text::RichText {
        egui::RichText::new(str).font(self.small_font_id.clone())
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        const SPACING: f32 = 10.0;
        egui::CentralPanel::default().show(ctx, |ui| {
            if !self.settings.is_equal(&self.global_settings){
                self.info_string = self.formatted_small_string("Currently unsaved settings");
            }


            ui.separator();
            ui.add_space(SPACING);
            ui.add_space(SPACING);
            let check_box_text = self.formatted_string("autohide");
            ui.checkbox(&mut self.settings.autohide, check_box_text);
            ui.add_space(SPACING);
            ui.label(self.formatted_string("Animation Steps on hover and fade away:",));
            ui.add(egui::Slider::new(
                &mut self.settings.animation_steps,
                0..=20,
            ).step_by(1.0));
            ui.add_space(SPACING);
            ui.label(self.formatted_string("Animation Step Time in MS:",));
            ui.add(egui::Slider::new(
                &mut self.settings.animation_time_in_ms,
                0..=100,
            ).step_by(1.0));
            ui.label(self.formatted_small_string(format!("Current hide/show animation time in seconds: {} s", ((self.settings.animation_time_in_ms * self.settings.animation_steps as u64) as f64)/1000.0).as_str()));

            ui.add_space(SPACING);
            ui.label(self.formatted_string("Hidden_TB Refresh Time in MS:",));
            ui.add(egui::Slider::new(
                &mut self.settings.sleep_time_in_ms,
                0..=200,
            ).step_by(5.0));
            ui.add_space(SPACING);
            ui.label(self.formatted_string("Detection Size of Hidden_TB:",));
            ui.add(egui::Slider::new(
                &mut self.settings.tb_rect_detection_size_in_pixel,
                1..=100,
            ).step_by(1.0));
            ui.add_space(SPACING);
            ui.label(self.formatted_string("Infrequent Work Refresh Skips:",));
            ui.label(self.formatted_small_string("Keep it to check at around one second (time is 'Infrequent Work Refresh Skips' * 'Hidden_TB Refresh Time in MS'). It handles the checks for lower priority changes like a new display attached.",));
            ui.label(self.formatted_small_string(format!("Current infrequent check timeout in seconds: {} s", ((self.settings.infrequent_count * self.settings.sleep_time_in_ms as usize) as f64)/1000.0).as_str()));
            ui.add(egui::Slider::new(
                &mut self.settings.infrequent_count,
                0..=360,
            ).step_by(1.0));
            ui.add_space(SPACING);
            ui.label(self.formatted_string("Bottom Rect Offset:",));
            ui.label(self.formatted_small_string("Leave this value at 1 or 0. If the tb isn't detected when your mouse hits the bottom of the screen, increase it by one and test again.",));
            ui.add(egui::Slider::new(
                &mut self.settings.tb_rect_bottom_offset,
                0..=5,
            ).step_by(1.0));
            ui.add_space(SPACING);
            ui.separator();
            if ui
                .button(self.formatted_string("Save Settings"))
                .clicked()
            {
                self.global_settings.set_autohide(self.settings.autohide);
                self.global_settings
                    .set_animation_steps(self.settings.animation_steps);
                self.global_settings
                    .set_animation_time_in_ms(self.settings.animation_time_in_ms);
                self.global_settings
                    .set_sleep_time_in_ms(self.settings.sleep_time_in_ms);
                self.global_settings
                    .set_infrequent_count(self.settings.infrequent_count);
                self.global_settings
                    .set_tb_rect_bottom_offset(self.settings.tb_rect_bottom_offset);
                self.global_settings.set_tb_rect_detection_size_in_pixel(
                    self.settings.tb_rect_detection_size_in_pixel,
                );
                self.info_string = self.formatted_small_string("Settings applied. Close the Settings and start hidden_tb.exe with the new config applied.");
            }

            ui.label(self.info_string.clone());
        });
    }
}
