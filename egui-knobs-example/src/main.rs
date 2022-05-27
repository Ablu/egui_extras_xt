use std::f32::consts::TAU;

use eframe::egui::{self, global_dark_light_mode_switch, DragValue};
use eframe::epaint::Color32;

use itertools::Itertools;

use egui_knobs::{
    AngleKnob, AudioKnob, CompassKnob, CompassKnobMarker, CompassKnobMarkerShape, KnobDirection,
    KnobMode, KnobOrientation, KnobShape,
};

struct EguiKnobsExampleApp {
    // Common properties
    common_orientation: KnobOrientation,
    common_direction: KnobDirection,
    common_mode: KnobMode,
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

    // CompassKnob
    compass_knob_value: f32,
    compass_knob_spread: f32,
    compass_knob_show_cursor: bool,
}

impl Default for EguiKnobsExampleApp {
    fn default() -> Self {
        Self {
            // Common properties
            common_orientation: KnobOrientation::Top,
            common_direction: KnobDirection::Clockwise,
            common_mode: KnobMode::Signed,
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

            // CompassKnob
            compass_knob_value: 0.0,
            compass_knob_spread: TAU / 2.0,
            compass_knob_show_cursor: true,
        }
    }
}

impl eframe::App for EguiKnobsExampleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                global_dark_light_mode_switch(ui);
                ui.heading("Knobs");
            });

            ui.separator();

            ui.heading("Common properties");
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.common_orientation, KnobOrientation::Top, "⬆ Top");
                ui.selectable_value(
                    &mut self.common_orientation,
                    KnobOrientation::Right,
                    "➡ Right",
                );
                ui.selectable_value(
                    &mut self.common_orientation,
                    KnobOrientation::Bottom,
                    "⬇ Bottom",
                );
                ui.selectable_value(
                    &mut self.common_orientation,
                    KnobOrientation::Left,
                    "⬅ Left",
                );

                {
                    let mut is_custom_orientation =
                        matches!(self.common_orientation, KnobOrientation::Custom(..));

                    ui.selectable_value(&mut is_custom_orientation, true, "✏ Custom(..)");

                    if is_custom_orientation
                        && !matches!(self.common_orientation, KnobOrientation::Custom(..))
                    {
                        self.common_orientation = KnobOrientation::Custom(0.0);
                    }

                    if let KnobOrientation::Custom(value) = &mut self.common_orientation {
                        ui.drag_angle(value);
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.common_direction,
                    KnobDirection::Clockwise,
                    "⟳ Clockwise",
                );
                ui.selectable_value(
                    &mut self.common_direction,
                    KnobDirection::Counterclockwise,
                    "⟲ Counterclockwise",
                );
            });

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.common_mode, KnobMode::Signed, "± Signed");

                ui.selectable_value(&mut self.common_mode, KnobMode::Unsigned, "+ Unsigned");

                ui.selectable_value(&mut self.common_mode, KnobMode::SpinAround, "🔃 SpinAround");
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
                                .direction(self.common_direction)
                                .spread(self.audio_knob_spread)
                                .thickness(self.audio_knob_thickness)
                                .shape(KnobShape::Squircle(4.0))
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
                                .direction(self.common_direction)
                                .shape(KnobShape::Circle)
                                .mode(self.common_mode)
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

                ui.heading("CompassKnob");
                ui.add_space(8.0);

                ui.drag_angle(&mut self.compass_knob_value);
                ui.drag_angle(&mut self.compass_knob_spread);
                ui.checkbox(&mut self.compass_knob_show_cursor, "Show cursor");
                ui.add_space(8.0);

                ui.add(
                    CompassKnob::new(&mut self.compass_knob_value)
                        .mode(self.common_mode)
                        .direction(self.common_direction)
                        .width(512.0)
                        .height(48.0)
                        .spread(self.compass_knob_spread)
                        .labels(["N", "E", "S", "W"])
                        .snap(self.common_snap)
                        .shift_snap(self.common_shift_snap)
                        .min(self.common_minimum_angle)
                        .max(self.common_maximum_angle)
                        .animated(self.common_animated)
                        .show_cursor(self.compass_knob_show_cursor)
                        .markers(&[
                            CompassKnobMarker::new(0.0f32.to_radians())
                                .shape(CompassKnobMarkerShape::Star(5, 0.5))
                                .label("Test")
                                .color(Color32::from_rgb(0x00, 0xA0, 0x00)),
                            // Grand Theft Auto style markers
                            CompassKnobMarker::new(100.0f32.to_radians())
                                .shape(CompassKnobMarkerShape::Square)
                                .label("Sweet")
                                .color(Color32::from_rgb(0x00, 0x00, 0xFF)),
                            CompassKnobMarker::new(120.0f32.to_radians())
                                .shape(CompassKnobMarkerShape::DownArrow)
                                .label("Reece's")
                                .color(Color32::from_rgb(0xFF, 0xFF, 0x00)),
                            CompassKnobMarker::new(140.0f32.to_radians())
                                .shape(CompassKnobMarkerShape::UpArrow)
                                .label("Big Smoke")
                                .color(Color32::from_rgb(0xFF, 0x00, 0x00)),
                            // Emoji markers
                            CompassKnobMarker::new(553.0f32.to_radians())
                                .shape(CompassKnobMarkerShape::Emoji('🐱'))
                                .label("Cat")
                                .color(Color32::from_rgb(0xF8, 0xE9, 0xFF)),
                            CompassKnobMarker::new(563.0f32.to_radians())
                                .shape(CompassKnobMarkerShape::Emoji('🐶'))
                                .label("Dog")
                                .color(Color32::from_rgb(0xC0, 0x8C, 0x85)),
                            // All marker shapes
                            CompassKnobMarker::new(240.0f32.to_radians())
                                .shape(CompassKnobMarkerShape::Square),
                            CompassKnobMarker::new(250.0f32.to_radians())
                                .shape(CompassKnobMarkerShape::Circle),
                            CompassKnobMarker::new(260.0f32.to_radians())
                                .shape(CompassKnobMarkerShape::RightArrow),
                            CompassKnobMarker::new(270.0f32.to_radians())
                                .shape(CompassKnobMarkerShape::UpArrow),
                            CompassKnobMarker::new(280.0f32.to_radians())
                                .shape(CompassKnobMarkerShape::LeftArrow),
                            CompassKnobMarker::new(290.0f32.to_radians())
                                .shape(CompassKnobMarkerShape::DownArrow),
                            CompassKnobMarker::new(300.0f32.to_radians())
                                .shape(CompassKnobMarkerShape::Diamond),
                            CompassKnobMarker::new(310.0f32.to_radians())
                                .shape(CompassKnobMarkerShape::Star(5, 0.5)),
                            CompassKnobMarker::new(320.0f32.to_radians())
                                .shape(CompassKnobMarkerShape::Emoji('🗿')),
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
        Box::new(|_cc| Box::new(EguiKnobsExampleApp::default())),
    );
}