use std::f32::consts::{PI, TAU};

use eframe::egui;
use eframe::emath::{Rot2, Vec2};
use eframe::epaint::{Shape, Stroke};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum AngleKnobOrientation {
    Right,
    Bottom,
    Left,
    Top,
    Custom(f32),
}

impl AngleKnobOrientation {
    pub fn rot2(&self) -> Rot2 {
        match *self {
            Self::Right => Rot2::from_angle(PI * 0.0),
            Self::Bottom => Rot2::from_angle(PI * 0.5),
            Self::Left => Rot2::from_angle(PI * 1.0),
            Self::Top => Rot2::from_angle(PI * 1.5),
            Self::Custom(angle) => Rot2::from_angle(angle),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum AngleKnobDirection {
    Clockwise,
    Counterclockwise,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum AngleKnobMode {
    Signed,
    Unsigned,
    SpinAround,
}

#[non_exhaustive]
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum AngleKnobPreset {
    AdobePhotoshop,
    AdobePremierePro,
    Gimp,
    GoogleChromeDevTools,
    Krita,
    LibreOffice,
    QtWidgets,
    // Software without knob widgets:
    // - Blender (no knobs but transform gizmo suggests Top/Clockwise/SpinAround)
    // - Inkscape
    // - Kdenlive
    // - MyPaint (no knobs but canvas rotation suggests Right/Clockwise/Signed)
}

impl AngleKnobPreset {
    fn properties(&self) -> (AngleKnobOrientation, AngleKnobDirection, AngleKnobMode) {
        match *self {
            AngleKnobPreset::AdobePhotoshop => (
                AngleKnobOrientation::Right,
                AngleKnobDirection::Counterclockwise,
                AngleKnobMode::Signed,
            ),
            AngleKnobPreset::AdobePremierePro => (
                AngleKnobOrientation::Top,
                AngleKnobDirection::Clockwise,
                AngleKnobMode::SpinAround,
            ),
            AngleKnobPreset::Gimp => (
                AngleKnobOrientation::Right,
                AngleKnobDirection::Counterclockwise,
                AngleKnobMode::Unsigned,
            ),
            AngleKnobPreset::GoogleChromeDevTools => (
                AngleKnobOrientation::Top,
                AngleKnobDirection::Clockwise,
                AngleKnobMode::Unsigned,
            ),
            // Knob widgets are a clusterfuck in Krita, however a significant
            // number of them follow what Photoshop does.
            AngleKnobPreset::Krita => (
                AngleKnobOrientation::Right,
                AngleKnobDirection::Counterclockwise,
                AngleKnobMode::Signed,
            ),
            AngleKnobPreset::LibreOffice => (
                AngleKnobOrientation::Right,
                AngleKnobDirection::Counterclockwise,
                AngleKnobMode::Unsigned,
            ),
            AngleKnobPreset::QtWidgets => (
                AngleKnobOrientation::Bottom,
                AngleKnobDirection::Clockwise,
                AngleKnobMode::Unsigned,
            ),
        }
    }
}

pub fn angle_knob(
    ui: &mut egui::Ui,
    diameter: f32,
    orientation: AngleKnobOrientation,
    direction: AngleKnobDirection,
    mode: AngleKnobMode,
    value: &mut f32,
    min: Option<f32>,
    max: Option<f32>,
    snap_angle: Option<f32>,
    shift_snap_angle: Option<f32>,
) -> egui::Response {
    let desired_size = Vec2::splat(diameter);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click_and_drag());

    let value_direction = match direction {
        AngleKnobDirection::Clockwise => 1.0,
        AngleKnobDirection::Counterclockwise => -1.0,
    };

    let rotation_matrix = orientation.rot2();

    if response.clicked() || response.dragged() {
        let mut new_value = (rotation_matrix.inverse()
            * (response.interact_pointer_pos().unwrap() - rect.center()))
        .angle()
            * value_direction;

        if mode == AngleKnobMode::Unsigned {
            new_value = (new_value + TAU) % TAU;
        }

        if mode == AngleKnobMode::SpinAround {
            let prev_turns = (*value / TAU).round();
            new_value += prev_turns * TAU;

            if new_value - *value > PI {
                new_value -= TAU;
            } else if new_value - *value < -PI {
                new_value += TAU;
            }
        }

        if let Some(angle) = if ui.input().modifiers.shift_only() {
            shift_snap_angle
        } else {
            snap_angle
        } {
            assert!(angle > 0.0, "non-positive snap angles are not supported");
            new_value = (new_value / angle).round() * angle;
        }

        if let Some(min) = min {
            new_value = new_value.max(min);
        }

        if let Some(max) = max {
            new_value = new_value.min(max);
        }

        *value = new_value;
        response.mark_changed();
    }

    if ui.is_rect_visible(rect) {
        let visuals = ui.style().interact(&response);
        let radius = diameter / 2.0;

        ui.painter()
            .circle(rect.center(), radius, visuals.bg_fill, visuals.fg_stroke);

        let paint_axis = |axis_direction| {
            let axis_vec2 = rotation_matrix * axis_direction * radius;

            ui.painter().add(Shape::dashed_line(
                &[rect.center() + axis_vec2, rect.center() - axis_vec2],
                ui.visuals().window_stroke(), // TODO: Semantically correct color
                1.0,
                1.0,
            ));
        };

        paint_axis(Vec2::DOWN);
        paint_axis(Vec2::RIGHT);

        let paint_stop = |stop_position: f32| {
            let stop_vec2 =
                rotation_matrix * Vec2::angled(stop_position * value_direction) * radius;

            let stop_alpha = 1.0
                - ((stop_position - *value).abs() / (PI * 1.5))
                    .clamp(0.0, 1.0)
                    .powf(5.0);

            // TODO: Semantically correct color
            let stop_stroke = Stroke::new(
                visuals.fg_stroke.width,
                visuals.fg_stroke.color.linear_multiply(stop_alpha),
            );

            ui.painter()
                .line_segment([rect.center(), rect.center() + stop_vec2], stop_stroke);
        };

        if let Some(min) = min {
            paint_stop(min);
        }

        if let Some(max) = max {
            paint_stop(max);
        }

        {
            let value_vec2 = rotation_matrix * Vec2::angled(*value * value_direction) * radius;

            ui.painter().line_segment(
                [rect.center(), rect.center() + value_vec2],
                visuals.fg_stroke, // TODO: Semantically correct color
            );

            ui.painter().circle(
                rect.center(),
                diameter / 24.0,
                visuals.text_color(), // TODO: Semantically correct color
                visuals.fg_stroke,    // TODO: Semantically correct color
            );

            ui.painter().circle(
                rect.center() + value_vec2,
                diameter / 24.0,
                visuals.text_color(), // TODO: Semantically correct color
                visuals.fg_stroke,    // TODO: Semantically correct color
            );
        }
    }

    response
}
