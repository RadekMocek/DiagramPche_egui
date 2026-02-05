use crate::model::color::Color;
use crate::model::pathpoint::Pathpoint;
use crate::model::point::Point;

pub struct Path {
    pub start: Point,
    pub ends: Vec<Point>,
    pub pathpoints: Vec<Pathpoint>,

    pub shift: i64,

    pub color: Color,

    pub do_start_arrow: bool,
    pub do_end_arrow: bool,
}

impl Default for Path {
    fn default() -> Self {
        Self {
            start: Point::default(),
            ends: Vec::new(),
            pathpoints: Vec::new(),

            shift: 0,

            color: Color::new(0, 0, 0, 255),

            do_start_arrow: false,
            do_end_arrow: true,
        }
    }
}
