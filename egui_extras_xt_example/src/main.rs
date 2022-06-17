use std::f32::consts::TAU;

use eframe::egui::{self, global_dark_light_mode_switch, DragValue};
use eframe::epaint::Color32;

use itertools::Itertools;

use egui_extras_xt::segmented_display::{
    DisplayKind, DisplayMetrics, DisplayStyle, DisplayStylePreset, SegmentedDisplayWidget,
    SevenSegment, SixteenSegment,
};
use egui_extras_xt::{
    AngleKnob, AudioKnob, CompassMarkerShape, LinearCompass, LinearCompassMarker, Orientation,
    PolarCompass, PolarCompassMarker, PolarCompassOverflow, WidgetShape, Winding, WrapMode,
};

struct EguiExtrasXtExampleApp {
    // Common properties
    common_orientation: Orientation,
    common_winding: Winding,
    common_wrap: WrapMode,
    common_animated: bool,
    common_snap: Option<f32>,
    common_shift_snap: Option<f32>,
    common_minimum_angle: Option<f32>,
    common_maximum_angle: Option<f32>,

    // AngleKnob
    angle_knob_value: f32,

    // AudioKnob
    audio_knob_value: f32,
    audio_knob_spread: f32,
    audio_knob_thickness: f32,

    // LinearCompass
    linear_compass_value: f32,
    linear_compass_spread: f32,
    linear_compass_show_cursor: bool,

    // PolarCompass
    polar_compass_overflow: PolarCompassOverflow,
    polar_compass_max_distance: f32,
    polar_compass_ring_count: usize,
    polar_compass_diameter: f32,
    polar_compass_label_height: f32,
    polar_compass_marker_near_size: f32,
    polar_compass_marker_far_size: f32,

    // SegmentedDisplayWidget
    segmented_display_display_kind: Box<dyn DisplayKind>,
    segmented_display_display_string: String,
    segmented_display_digit_height: f32,
    segmented_display_style: DisplayStyle,
    segmented_display_metrics: DisplayMetrics,
    segmented_display_show_dots: bool,
    segmented_display_show_colons: bool,
    segmented_display_show_apostrophes: bool,
}

impl Default for EguiExtrasXtExampleApp {
    fn default() -> Self {
        Self {
            // Common properties
            common_orientation: Orientation::Top,
            common_winding: Winding::Clockwise,
            common_wrap: WrapMode::Signed,
            common_animated: true,
            common_snap: None,
            common_shift_snap: None,
            common_minimum_angle: None,
            common_maximum_angle: None,

            // AngleKnob
            angle_knob_value: TAU / 18.0,

            // AudioKnob
            audio_knob_value: 0.75,
            audio_knob_spread: 1.0,
            audio_knob_thickness: 0.66,

            // LinearCompass
            linear_compass_value: 0.0,
            linear_compass_spread: TAU / 2.0,
            linear_compass_show_cursor: true,

            // PolarCompass
            polar_compass_overflow: PolarCompassOverflow::Saturate,
            polar_compass_max_distance: 10000.0,
            polar_compass_ring_count: 4,
            polar_compass_diameter: 256.0,
            polar_compass_label_height: 24.0,
            polar_compass_marker_near_size: 16.0,
            polar_compass_marker_far_size: 8.0,

            // SegmentedDisplayWidget
            segmented_display_display_kind: Box::new(SixteenSegment),
            segmented_display_display_string: String::from("12.34:5' HELLO"),
            segmented_display_digit_height: 128.0,
            segmented_display_style: DisplayStylePreset::NintendoGameBoy.style(),
            segmented_display_metrics: DisplayMetrics::default(),
            segmented_display_show_dots: true,
            segmented_display_show_colons: true,
            segmented_display_show_apostrophes: true,
        }
    }
}

impl eframe::App for EguiExtrasXtExampleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                global_dark_light_mode_switch(ui);
                ui.heading("Knobs");

                if ui.button("Reset").clicked() {
                    *self = Self::default();
                }
            });

            ui.separator();

            ui.heading("Common properties");
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.common_orientation, Orientation::Top, "⬆ Top");
                ui.selectable_value(&mut self.common_orientation, Orientation::Right, "➡ Right");
                ui.selectable_value(
                    &mut self.common_orientation,
                    Orientation::Bottom,
                    "⬇ Bottom",
                );
                ui.selectable_value(&mut self.common_orientation, Orientation::Left, "⬅ Left");

                {
                    let mut is_custom_orientation =
                        matches!(self.common_orientation, Orientation::Custom(..));

                    ui.selectable_value(&mut is_custom_orientation, true, "✏ Custom(..)");

                    if is_custom_orientation
                        && !matches!(self.common_orientation, Orientation::Custom(..))
                    {
                        self.common_orientation = Orientation::Custom(0.0);
                    }

                    if let Orientation::Custom(value) = &mut self.common_orientation {
                        ui.drag_angle(value);
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.common_winding, Winding::Clockwise, "⟳ Clockwise");
                ui.selectable_value(
                    &mut self.common_winding,
                    Winding::Counterclockwise,
                    "⟲ Counterclockwise",
                );
            });

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.common_wrap, WrapMode::None, "🔃 None");

                ui.selectable_value(&mut self.common_wrap, WrapMode::Signed, "± Signed");

                ui.selectable_value(&mut self.common_wrap, WrapMode::Unsigned, "+ Unsigned");
            });

            ui.horizontal(|ui| {
                {
                    let mut snap_enabled = self.common_snap.is_some();
                    ui.toggle_value(&mut snap_enabled, "Snap");

                    self.common_snap = match (snap_enabled, self.common_snap) {
                        (true, None) => Some(TAU / 24.0),
                        (false, Some(_)) => None,
                        _ => self.common_snap,
                    };

                    if let Some(value) = &mut self.common_snap {
                        ui.drag_angle(value);
                        ui.add(DragValue::new(value).speed(0.1));
                        *value = value.max(0.0);
                    }
                }

                {
                    let mut shift_snap_enabled = self.common_shift_snap.is_some();
                    ui.toggle_value(&mut shift_snap_enabled, "Shift snap");

                    self.common_shift_snap = match (shift_snap_enabled, self.common_shift_snap) {
                        (true, None) => Some(TAU / 24.0),
                        (false, Some(_)) => None,
                        _ => self.common_shift_snap,
                    };

                    if let Some(value) = &mut self.common_shift_snap {
                        ui.drag_angle(value);
                        ui.add(DragValue::new(value).speed(0.1));
                        *value = value.max(0.0);
                    }
                }
            });

            ui.horizontal(|ui| {
                {
                    let mut minimum_enabled = self.common_minimum_angle.is_some();
                    ui.toggle_value(&mut minimum_enabled, "Minimum");

                    self.common_minimum_angle = match (minimum_enabled, self.common_minimum_angle) {
                        (true, None) => Some(-TAU),
                        (false, Some(_)) => None,
                        _ => self.common_minimum_angle,
                    };

                    if let Some(value) = &mut self.common_minimum_angle {
                        ui.drag_angle(value);
                    }
                }

                {
                    let mut maximum_enabled = self.common_maximum_angle.is_some();
                    ui.toggle_value(&mut maximum_enabled, "Maximum");

                    self.common_maximum_angle = match (maximum_enabled, self.common_maximum_angle) {
                        (true, None) => Some(TAU),
                        (false, Some(_)) => None,
                        _ => self.common_maximum_angle,
                    };

                    if let Some(value) = &mut self.common_maximum_angle {
                        ui.drag_angle(value);
                    }
                }
            });

            ui.checkbox(&mut self.common_animated, "Animated");
            ui.add_space(8.0);
            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("PolarCompass");
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.selectable_value(
                        &mut self.polar_compass_overflow,
                        PolarCompassOverflow::Clip,
                        "✂ Clip",
                    );
                    ui.selectable_value(
                        &mut self.polar_compass_overflow,
                        PolarCompassOverflow::Saturate,
                        "➡| Saturate",
                    );
                });

                ui.horizontal(|ui| {
                    ui.add(DragValue::new(&mut self.polar_compass_max_distance));
                    ui.add(DragValue::new(&mut self.polar_compass_ring_count));
                });

                ui.horizontal(|ui| {
                    ui.add(DragValue::new(&mut self.polar_compass_diameter));
                    ui.add(DragValue::new(&mut self.polar_compass_label_height));
                });

                ui.horizontal(|ui| {
                    ui.add(DragValue::new(&mut self.polar_compass_marker_near_size));
                    ui.add(DragValue::new(&mut self.polar_compass_marker_far_size));
                });

                ui.add_space(8.0);

                ui.add(
                    PolarCompass::new()
                        .orientation(self.common_orientation)
                        .winding(self.common_winding)
                        .overflow(self.polar_compass_overflow)
                        .diameter(self.polar_compass_diameter)
                        .labels(["N", "E", "S", "W"])
                        .label_height(self.polar_compass_label_height)
                        .max_distance(self.polar_compass_max_distance)
                        .ring_count(self.polar_compass_ring_count)
                        .marker_near_size(self.polar_compass_marker_near_size)
                        .marker_far_size(self.polar_compass_marker_far_size)
                        .markers(&[
                            PolarCompassMarker::new(-15.0f32.to_radians(), 1.0)
                                .color(Color32::KHAKI)
                                .shape(CompassMarkerShape::Circle),
                            PolarCompassMarker::new(0.0f32.to_radians(), 10.0)
                                .color(Color32::RED)
                                .shape(CompassMarkerShape::Circle),
                            PolarCompassMarker::new(15.0f32.to_radians(), 100.0)
                                .color(Color32::GREEN)
                                .shape(CompassMarkerShape::Square),
                            PolarCompassMarker::new(30.0f32.to_radians(), 1000.0)
                                .color(Color32::BLUE)
                                .shape(CompassMarkerShape::UpArrow),
                            PolarCompassMarker::new(45.0f32.to_radians(), 10000.0)
                                .color(Color32::GOLD)
                                .shape(CompassMarkerShape::DownArrow),
                        ]),
                );

                ui.add_space(8.0);
                ui.separator();

                ui.heading("SegmentedDisplayWidget");
                ui.add_space(8.0);

                ui.add(egui::Slider::new(
                    &mut self.segmented_display_metrics.segment_thickness,
                    0.0..=1.0,
                ));
                ui.add(egui::Slider::new(
                    &mut self.segmented_display_metrics.segment_spacing,
                    0.0..=1.0,
                ));
                ui.add(egui::Slider::new(
                    &mut self.segmented_display_metrics.digit_shearing,
                    -1.0..=1.0,
                ));
                ui.add(egui::Slider::new(
                    &mut self.segmented_display_digit_height,
                    16.0..=256.0,
                ));
                ui.add(egui::Slider::new(
                    &mut self.segmented_display_metrics.digit_ratio,
                    0.25..=1.0,
                ));
                ui.add(egui::Slider::new(
                    &mut self.segmented_display_metrics.digit_spacing,
                    0.0..=1.0,
                ));
                ui.add(egui::Slider::new(
                    &mut self.segmented_display_metrics.digit_median,
                    -1.0..=1.0,
                ));
                ui.add(egui::Slider::new(
                    &mut self.segmented_display_metrics.margin_horizontal,
                    0.0..=1.0,
                ));
                ui.add(egui::Slider::new(
                    &mut self.segmented_display_metrics.margin_vertical,
                    0.0..=1.0,
                ));
                ui.add(egui::Slider::new(
                    &mut self.segmented_display_metrics.colon_separation,
                    0.0..=1.0,
                ));

                ui.color_edit_button_srgba(&mut self.segmented_display_style.background_color);

                ui.horizontal(|ui| {
                    ui.add(
                        DragValue::new(&mut self.segmented_display_style.segment_on_stroke.width)
                            .speed(0.1),
                    );
                    ui.color_edit_button_srgba(
                        &mut self.segmented_display_style.segment_on_stroke.color,
                    );
                    ui.color_edit_button_srgba(&mut self.segmented_display_style.segment_on_color);
                });

                ui.horizontal(|ui| {
                    ui.add(
                        DragValue::new(&mut self.segmented_display_style.segment_off_stroke.width)
                            .speed(0.1),
                    );
                    ui.color_edit_button_srgba(
                        &mut self.segmented_display_style.segment_off_stroke.color,
                    );
                    ui.color_edit_button_srgba(&mut self.segmented_display_style.segment_off_color);
                });

                ui.horizontal(|ui| {
                    ui.toggle_value(&mut self.segmented_display_show_dots, "Dots");
                    ui.toggle_value(&mut self.segmented_display_show_colons, "Colons");
                    ui.toggle_value(&mut self.segmented_display_show_apostrophes, "Apostrophes");
                });

                ui.horizontal(|ui| {
                    if ui.button("7-segment").clicked() {
                        self.segmented_display_display_kind = Box::new(SevenSegment);
                    }

                    if ui.button("16-segment").clicked() {
                        self.segmented_display_display_kind = Box::new(SixteenSegment);
                    }
                });

                ui.add(egui::TextEdit::singleline(
                    &mut self.segmented_display_display_string,
                ));

                ui.add_space(8.0);

                ui.add(
                    SegmentedDisplayWidget::new(Box::new(SixteenSegment))
                        .style(self.segmented_display_style)
                        .metrics(self.segmented_display_metrics)
                        .digit_height(self.segmented_display_digit_height)
                        .show_dots(self.segmented_display_show_dots)
                        .show_colons(self.segmented_display_show_colons)
                        .show_apostrophes(self.segmented_display_show_apostrophes)
                        .push_string(&self.segmented_display_display_string),
                );

                ui.add_space(8.0);
                ui.separator();

                ui.heading("AudioKnob");
                ui.add_space(8.0);
                ui.add(egui::Slider::new(&mut self.audio_knob_value, -1.0..=1.0));
                ui.add(egui::Slider::new(&mut self.audio_knob_spread, 0.0..=1.0));
                ui.add(egui::Slider::new(&mut self.audio_knob_thickness, 0.0..=1.0));

                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    for (audio_knob_range, audio_knob_size) in [0.0..=1.0, -1.0..=1.0]
                        .into_iter()
                        .cartesian_product([64.0, 32.0])
                    {
                        ui.add(
                            AudioKnob::new(&mut self.audio_knob_value, audio_knob_range)
                                .diameter(audio_knob_size)
                                .orientation(self.common_orientation)
                                .winding(self.common_winding)
                                .spread(self.audio_knob_spread)
                                .thickness(self.audio_knob_thickness)
                                .shape(WidgetShape::Squircle(4.0))
                                .animated(self.common_animated)
                                .snap(self.common_snap)
                                .shift_snap(self.common_shift_snap),
                        );
                    }
                });

                ui.add_space(8.0);
                ui.separator();

                ui.heading("AngleKnob");
                ui.add_space(8.0);

                ui.drag_angle(&mut self.angle_knob_value);
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    for angle_knob_size in [64.0, 32.0] {
                        ui.add(
                            AngleKnob::new(&mut self.angle_knob_value)
                                .diameter(angle_knob_size)
                                .orientation(self.common_orientation)
                                .winding(self.common_winding)
                                .shape(WidgetShape::Circle)
                                .wrap(self.common_wrap)
                                .min(self.common_minimum_angle)
                                .max(self.common_maximum_angle)
                                .snap(self.common_snap)
                                .shift_snap(self.common_shift_snap)
                                .show_axes(true)
                                .axis_count(4),
                        );
                    }
                });

                ui.add_space(8.0);
                ui.separator();

                ui.heading("LinearCompass");
                ui.add_space(8.0);

                ui.drag_angle(&mut self.linear_compass_value);
                ui.drag_angle(&mut self.linear_compass_spread);
                ui.checkbox(&mut self.linear_compass_show_cursor, "Show cursor");
                ui.add_space(8.0);

                ui.add(
                    LinearCompass::new(&mut self.linear_compass_value)
                        .wrap(self.common_wrap)
                        .winding(self.common_winding)
                        .width(512.0)
                        .height(48.0)
                        .spread(self.linear_compass_spread)
                        .labels(["N", "E", "S", "W"])
                        .snap(self.common_snap)
                        .shift_snap(self.common_shift_snap)
                        .min(self.common_minimum_angle)
                        .max(self.common_maximum_angle)
                        .animated(self.common_animated)
                        .show_cursor(self.linear_compass_show_cursor)
                        .markers(&[
                            LinearCompassMarker::new(0.0f32.to_radians())
                                .shape(CompassMarkerShape::Star(5, 0.5))
                                .label("Test")
                                .color(Color32::from_rgb(0x00, 0xA0, 0x00)),
                            // Grand Theft Auto style markers
                            LinearCompassMarker::new(70.0f32.to_radians())
                                .shape(CompassMarkerShape::Square)
                                .label("Sweet")
                                .color(Color32::from_rgb(0x00, 0x00, 0xFF)),
                            LinearCompassMarker::new(85.0f32.to_radians())
                                .shape(CompassMarkerShape::DownArrow)
                                .label("Reece's")
                                .color(Color32::from_rgb(0xFF, 0xFF, 0x00)),
                            LinearCompassMarker::new(100.0f32.to_radians())
                                .shape(CompassMarkerShape::UpArrow)
                                .label("Big Smoke")
                                .color(Color32::from_rgb(0xFF, 0x00, 0x00)),
                            // Emoji markers
                            LinearCompassMarker::new(553.0f32.to_radians())
                                .shape(CompassMarkerShape::Emoji('🐱'))
                                .label("Cat")
                                .color(Color32::from_rgb(0xF8, 0xE9, 0xFF)),
                            LinearCompassMarker::new(563.0f32.to_radians())
                                .shape(CompassMarkerShape::Emoji('🐶'))
                                .label("Dog")
                                .color(Color32::from_rgb(0xC0, 0x8C, 0x85)),
                            // All marker shapes
                            LinearCompassMarker::new(240.0f32.to_radians())
                                .shape(CompassMarkerShape::Square),
                            LinearCompassMarker::new(250.0f32.to_radians())
                                .shape(CompassMarkerShape::Circle),
                            LinearCompassMarker::new(260.0f32.to_radians())
                                .shape(CompassMarkerShape::RightArrow),
                            LinearCompassMarker::new(270.0f32.to_radians())
                                .shape(CompassMarkerShape::UpArrow),
                            LinearCompassMarker::new(280.0f32.to_radians())
                                .shape(CompassMarkerShape::LeftArrow),
                            LinearCompassMarker::new(290.0f32.to_radians())
                                .shape(CompassMarkerShape::DownArrow),
                            LinearCompassMarker::new(300.0f32.to_radians())
                                .shape(CompassMarkerShape::Diamond),
                            LinearCompassMarker::new(310.0f32.to_radians())
                                .shape(CompassMarkerShape::Star(5, 0.5)),
                            LinearCompassMarker::new(320.0f32.to_radians())
                                .shape(CompassMarkerShape::Emoji('🗿')),
                            // Transparent colors
                            LinearCompassMarker::new(30.0f32.to_radians())
                                .shape(CompassMarkerShape::Square)
                                .label("Near")
                                .color(Color32::from_rgb(0x40, 0x80, 0x80).linear_multiply(1.0)),
                            LinearCompassMarker::new(40.0f32.to_radians())
                                .shape(CompassMarkerShape::Square)
                                .label("Far")
                                .color(Color32::from_rgb(0x40, 0x80, 0x80).linear_multiply(0.5)),
                            LinearCompassMarker::new(50.0f32.to_radians())
                                .shape(CompassMarkerShape::Square)
                                .label("Very far")
                                .color(Color32::from_rgb(0x40, 0x80, 0x80).linear_multiply(0.25)),
                        ]),
                );

                ui.add_space(8.0);
                ui.separator();
            });
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Knobs",
        options,
        Box::new(|_cc| Box::new(EguiExtrasXtExampleApp::default())),
    );
}