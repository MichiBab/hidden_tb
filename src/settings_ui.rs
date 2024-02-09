use crate::tb_settings::TbSettings;
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
        initial_window_size: Some(egui::vec2(500.0, 800.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Hidden_TB Settings",
        options,
        Box::new(|cc| {
            let style = egui::Style {
                visuals: egui::Visuals::dark(),
                ..egui::Style::default()
            };
            cc.egui_ctx.set_style(style);
            Box::<MyApp>::default()
        }),
    );
}

pub struct TbAccessibleSettings {
    autohide: bool,
    workspace_offset_top: u32,
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
    dynamic_borders_show_tray_if_disabled_on_hover: bool,
    dynamic_borders_show_widgets: bool,
    dynamic_borders_show_widgets_if_disabled_on_hover: bool,
    rounded_corners_size: i32,
    margin_left: i32,
    margin_right: i32,
    margin_bottom: i32,
    margin_top: i32,
    margin_offset_left: i32,
    margin_offset_right: i32,
    windows_11_bugfix: bool,
}

impl TbAccessibleSettings {
    fn from(settings: &TbSettings) -> Self {
        Self {
            autohide: settings.get_autohide(),
            workspace_offset_top: settings.get_workspace_offset_top(),
            sleep_time_in_ms: settings.get_sleep_time_in_ms(),
            merge_tray: settings.get_merge_tray(),
            merge_widgets: settings.get_merge_widgets(),
            animation_time_in_ms: settings.get_animation_time_in_ms(),
            animation_steps: settings.get_animation_steps(),
            infrequent_count: settings.get_infrequent_count(),
            tb_rect_bottom_offset: settings.get_tb_rect_bottom_offset(),
            tb_rect_detection_size_in_pixel: settings.get_tb_rect_detection_size_in_pixel(),
            enable_dynamic_borders: settings.get_enable_dynamic_borders(),
            dynamic_borders_show_tray: settings.get_dynamic_borders_show_tray(),
            dynamic_borders_show_tray_if_disabled_on_hover: settings
                .get_dynamic_borders_show_tray_if_disabled_on_hover(),
            dynamic_borders_show_widgets: settings.get_dynamic_borders_show_widgets(),
            dynamic_borders_show_widgets_if_disabled_on_hover: settings
                .get_dynamic_borders_show_widgets_if_disabled_on_hover(),
            rounded_corners_size: settings.get_rounded_corners_size(),
            margin_left: settings.get_margin_left(),
            margin_right: settings.get_margin_right(),
            margin_bottom: settings.get_margin_bottom(),
            margin_top: settings.get_margin_top(),
            margin_offset_left: settings.get_margin_offset_left(),
            margin_offset_right: settings.get_margin_offset_right(),
            windows_11_bugfix: settings.get_windows_11_bugfix(),
        }
    }

    fn is_equal(&self, settings: &TbSettings) -> bool {
        self.autohide == settings.get_autohide()
            && self.merge_tray == settings.get_merge_tray()
            && self.merge_widgets == settings.get_merge_widgets()
            && self.sleep_time_in_ms == settings.get_sleep_time_in_ms()
            && self.animation_time_in_ms == settings.get_animation_time_in_ms()
            && self.animation_steps == settings.get_animation_steps()
            && self.infrequent_count == settings.get_infrequent_count()
            && self.tb_rect_bottom_offset == settings.get_tb_rect_bottom_offset()
            && self.tb_rect_detection_size_in_pixel
                == settings.get_tb_rect_detection_size_in_pixel()
            && self.enable_dynamic_borders == settings.get_enable_dynamic_borders()
            && self.dynamic_borders_show_tray == settings.get_dynamic_borders_show_tray()
            && self.dynamic_borders_show_tray_if_disabled_on_hover
                == settings.get_dynamic_borders_show_tray_if_disabled_on_hover()
            && self.dynamic_borders_show_widgets == settings.get_dynamic_borders_show_widgets()
            && self.dynamic_borders_show_widgets_if_disabled_on_hover
                == settings.get_dynamic_borders_show_widgets_if_disabled_on_hover()
            && self.rounded_corners_size == settings.get_rounded_corners_size()
            && self.margin_left == settings.get_margin_left()
            && self.margin_right == settings.get_margin_right()
            && self.margin_bottom == settings.get_margin_bottom()
            && self.margin_top == settings.get_margin_top()
            && self.margin_offset_left == settings.get_margin_offset_left()
            && self.margin_offset_right == settings.get_margin_offset_right()
            && self.workspace_offset_top == settings.get_workspace_offset_top()
            && self.windows_11_bugfix == settings.get_windows_11_bugfix()
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
        let tb_settings = TbSettings::new();
        dbg!(&tb_settings);
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
    fn call_settings_update(&mut self) {
        self.global_settings
            .set_merge_tray(self.settings.merge_tray);
        self.global_settings
            .set_workspace_offset_top(self.settings.workspace_offset_top);
        self.global_settings
            .set_merge_widgets(self.settings.merge_widgets);
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
        self.global_settings
            .set_tb_rect_detection_size_in_pixel(self.settings.tb_rect_detection_size_in_pixel);
        self.global_settings
            .set_enable_dynamic_borders(self.settings.enable_dynamic_borders);
        self.global_settings
            .set_dynamic_borders_show_tray(self.settings.dynamic_borders_show_tray);
        self.global_settings
            .set_dynamic_borders_show_widgets_if_disabled_on_hover(
                self.settings
                    .dynamic_borders_show_widgets_if_disabled_on_hover,
            );
        self.global_settings
            .set_dynamic_borders_show_tray_if_disabled_on_hover(
                self.settings.dynamic_borders_show_tray_if_disabled_on_hover,
            );
        self.global_settings
            .set_dynamic_borders_show_widgets(self.settings.dynamic_borders_show_widgets);
        self.global_settings
            .set_rounded_corners_size(self.settings.rounded_corners_size);
        self.global_settings
            .set_margin_left(self.settings.margin_left);
        self.global_settings
            .set_margin_right(self.settings.margin_right);
        self.global_settings
            .set_margin_bottom(self.settings.margin_bottom);
        self.global_settings
            .set_margin_top(self.settings.margin_top);
        self.global_settings
            .set_margin_offset_left(self.settings.margin_offset_left);
        self.global_settings
            .set_margin_offset_right(self.settings.margin_offset_right);
        self.global_settings
            .set_windows_11_bugfix(self.settings.windows_11_bugfix);
    }

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
            if !self.settings.is_equal(&self.global_settings) {
                self.info_string = self.formatted_small_string("Currently unsaved settings");
            }
            let core_scroll_area = egui::ScrollArea
                ::new([true, true])
                .max_height(800.0)
                .id_source("options_scroll_area");
            core_scroll_area.show(ui, |ui| {
                ui.vertical(|ui| {
                    let options_scroll_area = egui::ScrollArea
                        ::new([true, true])
                        .max_height(750.0)
                        .id_source("options_scroll_area");
                    options_scroll_area.show(ui, |ui| {
                        ui.vertical(|ui| {
                            let autohide_scroll_area = egui::ScrollArea
                                ::new([true, true])
                                .max_height(200.0)
                                .id_source("autohide_scroll_area");
                            autohide_scroll_area.show(ui, |ui| {
                                ui.vertical(|ui| {
                                    let check_box_text = self.formatted_string("autohide");
                                    ui.checkbox(&mut self.settings.autohide, check_box_text);
                                    ui.add_space(SPACING);
                                    if self.settings.autohide {
                                        ui.label(
                                            self.formatted_string(
                                                "Animation Steps on hover and fade away:"
                                            )
                                        );
                                        ui.add(
                                            egui::Slider
                                                ::new(&mut self.settings.animation_steps, 0..=20)
                                                .step_by(1.0)
                                        );
                                        ui.add_space(SPACING);
                                        ui.label(
                                            self.formatted_string("Animation Step Time in MS:")
                                        );
                                        ui.add(
                                            egui::Slider
                                                ::new(
                                                    &mut self.settings.animation_time_in_ms,
                                                    0..=100
                                                )
                                                .step_by(1.0)
                                        );
                                        ui.label(
                                            self.formatted_small_string(
                                                format!(
                                                    "Current hide/show animation time in seconds: {} s",
                                                    (
                                                        (self.settings.animation_time_in_ms *
                                                            (
                                                                self.settings.animation_steps as u64
                                                            )) as f64
                                                    ) / 1000.0
                                                ).as_str()
                                            )
                                        );
                                        ui.add_space(SPACING);
                                        ui.label(
                                            self.formatted_string("Detection Size of Hidden_TB:")
                                        );
                                        ui.add(
                                            egui::Slider
                                                ::new(
                                                    &mut self.settings.tb_rect_detection_size_in_pixel,
                                                    1..=100
                                                )
                                                .step_by(1.0)
                                        );
                                        ui.add_space(SPACING);
                                        ui.label(self.formatted_string("Bottom Rect Offset:"));
                                        ui.label(
                                            self.formatted_small_string(
                                                "Leave this value at 1 or 0. If the tb isn't detected when your mouse hits the bottom of the screen, increase it by one and test again."
                                            )
                                        );
                                        ui.add(
                                            egui::Slider
                                                ::new(
                                                    &mut self.settings.tb_rect_bottom_offset,
                                                    0..=5
                                                )
                                                .step_by(1.0)
                                        );
                                        ui.add_space(SPACING);
                                        ui.label(
                                            self.formatted_string("Window maximizing Offset Top:")
                                        );
                                        ui.label(
                                            self.formatted_small_string(
                                                "Offset to change the maximize window behavior. Leave it on 0 to fill the screen on maximizing a window, and bigger then 0 to use a top bar like rainmeter, so it will always show even when maximizing a window."
                                            )
                                        );
                                        ui.add(
                                            egui::Slider
                                                ::new(
                                                    &mut self.settings.workspace_offset_top,
                                                    0..=200
                                                )
                                                .step_by(1.0)
                                        );
                                    }
                                });
                            });
                            ui.separator();

                            let scroll_area: egui::ScrollArea = egui::ScrollArea::new([true, true]).max_height(200.0);

                            scroll_area.show(ui, |ui| {
                                ui.vertical(|ui| {
                                    let check_box_text =
                                        self.formatted_string("enable dynamic borders");
                                    ui.checkbox(
                                        &mut self.settings.enable_dynamic_borders,
                                        check_box_text
                                    );
                                    ui.add_space(SPACING);
                                    if self.settings.enable_dynamic_borders {
                                        let check_box_text = self.formatted_string(
                                            "dynamic borders show tray"
                                        );
                                        ui.checkbox(
                                            &mut self.settings.dynamic_borders_show_tray,
                                            check_box_text
                                        );
                                        ui.add_space(SPACING);

                                        let check_box_text = self.formatted_string(
                                            "dynamic borders show tray on hover if disabled"
                                        );
                                        ui.checkbox(
                                            &mut self.settings.dynamic_borders_show_tray_if_disabled_on_hover,
                                            check_box_text
                                        );
                                        ui.add_space(SPACING);
                                        /* //TODO enable if implemented 
                                        let check_box_text = self.formatted_string(
                                            "dynamic borders show widgets"
                                        );
                                        ui.checkbox(
                                            &mut self.settings.dynamic_borders_show_widgets,
                                            check_box_text
                                        );
                                        ui.add_space(SPACING);

                                        let check_box_text = self.formatted_string(
                                            "dynamic borders show widgets on hover if disabled"
                                        );
                                        ui.checkbox(
                                            &mut self.settings.dynamic_borders_show_widgets_if_disabled_on_hover,
                                            check_box_text
                                        );
                                        ui.add_space(SPACING);
                                        */
                                        ui.label(
                                            self.formatted_string(
                                                "Dynamic borders rounded corner size:"
                                            )
                                        );
                                        ui.add(
                                            egui::Slider
                                                ::new(
                                                    &mut self.settings.rounded_corners_size,
                                                    0..=30
                                                )
                                                .step_by(1.0)
                                        );
                                        ui.add_space(SPACING);

                                        ui.label(
                                            self.formatted_string("Dynamic borders margin top:")
                                        );
                                        ui.add(
                                            egui::Slider
                                                ::new(&mut self.settings.margin_top, -5..=20)
                                                .step_by(1.0)
                                        );
                                        ui.add_space(SPACING);

                                        ui.label(
                                            self.formatted_string("Dynamic borders margin bottom:")
                                        );
                                        ui.add(
                                            egui::Slider
                                                ::new(&mut self.settings.margin_bottom, -5..=20)
                                                .step_by(1.0)
                                        );
                                        ui.add_space(SPACING);

                                        ui.label(
                                            self.formatted_string("Dynamic borders margin left:")
                                        );
                                        ui.add(
                                            egui::Slider
                                                ::new(&mut self.settings.margin_left, -5..=20)
                                                .step_by(1.0)
                                        );
                                        ui.add_space(SPACING);

                                        ui.label(
                                            self.formatted_string("Dynamic borders margin right:")
                                        );
                                        ui.add(
                                            egui::Slider
                                                ::new(&mut self.settings.margin_right, -5..=20)
                                                .step_by(1.0)
                                        );

                                        ui.label(
                                            self.formatted_string("Dynamic app borders margin left offset:")
                                        );
                                        ui.add(
                                            egui::Slider
                                                ::new(&mut self.settings.margin_offset_left, -1000..=1000)
                                                .step_by(1.0)
                                        );
                                        ui.label(
                                            self.formatted_string("Dynamic app borders margin right offset:")
                                        );
                                        ui.add(
                                            egui::Slider
                                                ::new(&mut self.settings.margin_offset_right, -1000..=1000)
                                                .step_by(1.0)
                                        );

                                        ui.add_space(SPACING);
                                        ui.label(
                                            self.formatted_small_string(
                                                "This fixes the taskbar size on newer windows 11 builds. If your Taskbar looks wrong, disable this and try again."
                                            )
                                        );
                                        let check_box_text = self.formatted_string("Windows 11 Bugfix");
                                        ui.checkbox(&mut self.settings.windows_11_bugfix, check_box_text);
                                        
                                    }
                                });
                            });
                            ui.separator();
                            ui.label(self.formatted_string("Hidden_TB Refresh Time in MS:"));
                            ui.add(
                                egui::Slider
                                    ::new(&mut self.settings.sleep_time_in_ms, 0..=200)
                                    .step_by(5.0)
                            );
                            ui.add_space(SPACING);

                            ui.label(self.formatted_string("Infrequent Work Refresh Skips:"));
                            ui.label(
                                self.formatted_small_string(
                                    "Keep it to check at around one second (time is 'Infrequent Work Refresh Skips' * 'Hidden_TB Refresh Time in MS'). It handles the checks for lower priority changes like a new display attached."
                                )
                            );
                            ui.label(
                                self.formatted_small_string(
                                    format!(
                                        "Current infrequent check timeout in seconds: {} s",
                                        (
                                            (self.settings.infrequent_count *
                                                (self.settings.sleep_time_in_ms as usize)) as f64
                                        ) / 1000.0
                                    ).as_str()
                                )
                            );
                            ui.add(
                                egui::Slider
                                    ::new(&mut self.settings.infrequent_count, 1..=360)
                                    .step_by(1.0)
                            );

                            ui.separator();

                            ui.add_space(SPACING);
                            /* Too buggy atm... */
                            //let check_box_text = self.formatted_string("merge tray with applist");
                            //ui.checkbox(&mut self.settings.merge_tray, check_box_text);
                            //ui.add_space(SPACING);
                            /* //TODO enable if implemented 
                            let check_box_text = self.formatted_string("merge widget with applist");
                            ui.checkbox(&mut self.settings.merge_widgets, check_box_text);
                            ui.add_space(SPACING);*/
                        });
                    });
                    ui.separator();

                    ui.add_space(SPACING);

                    ui.add_space(SPACING);

                    if ui.button(self.formatted_string("Save Settings")).clicked() {
                        self.call_settings_update();
                        self.info_string = self.formatted_small_string(
                            "Settings saved. Close the gui and start hidden_tb.exe again with the applied changes."
                        );
                    }

                    /*
                    if
                        ui
                            .button(
                                self.formatted_small_string("Save and Set to Restart on Close.")
                            )
                            .clicked()
                    {
                        self.call_settings_update();
                        self.info_string = self.formatted_small_string(
                            "Settings applied. Close this window to restart the taskbar."
                        );
                        signaling::get_signaling_struct().set_reset_called(true);
                    } */

                    ui.label(self.info_string.clone());
                    ui.separator();
                });
            });
        });
    }
}
