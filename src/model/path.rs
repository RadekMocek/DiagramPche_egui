use crate::helper::draw_layer::*;
use crate::model::color::Color;
use crate::model::pathpoint::Pathpoint;
use crate::model::pivot::Pivot;
use crate::model::point::Point;
use egui::{Vec2, vec2};

pub struct Path {
    pub start: Point,
    pub ends: Vec<Point>,
    pub pathpoints: Vec<Pathpoint>,

    pub shift_start: i64,
    pub shift_end: i64,

    pub color: Color,

    pub do_start_arrow: bool,
    pub do_end_arrow: bool,

    pub z: i64,

    pub label_value: String,
    pub label_point: i64,
    pub label_shift: i64,
    pub label_shift_orthogonal: i64,
    pub label_bg_color: Color,
}

impl Default for Path {
    fn default() -> Self {
        Self {
            start: Point::default(),
            ends: Vec::new(),
            pathpoints: Vec::new(),

            shift_start: 0,
            shift_end: 0,

            color: Color::new(0, 0, 0, 255),

            do_start_arrow: false,
            do_end_arrow: true,

            z: DL_USER_CHANNEL_DEFAULT_PATH,

            label_value: String::from(""),
            label_point: 0,
            label_shift: 0,
            label_shift_orthogonal: 0,
            label_bg_color: Color::new(0, 0, 0, 0),
        }
    }
}

impl Path {
    pub fn get_shift_vector(&self, pivot: &Pivot, zoom_level: f32, is_start: bool) -> Vec2 {
        let sf = if is_start {
            self.shift_start
        } else {
            self.shift_end
        } as f32
            * zoom_level;

        match pivot {
            Pivot::TopLeft => vec2(-sf, -sf),
            Pivot::Top => vec2(0.0, -sf),
            Pivot::TopRight => vec2(sf, -sf),
            Pivot::Right => vec2(sf, 0.0),
            Pivot::BottomRight => vec2(sf, sf),
            Pivot::Bottom => vec2(0.0, sf),
            Pivot::BottomLeft => vec2(-sf, sf),
            Pivot::Left => vec2(-sf, 0.0),
            Pivot::Center => vec2(0.0, 0.0),
        }
    }
}
