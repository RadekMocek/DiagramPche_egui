use egui::{Painter, Pos2, Vec2};
use std::cmp::Ordering;

pub trait DrawCommand {
    fn draw(&self, painter: &Painter);
    fn draw_svg(&self, document: &mut svg::Document, origin: Pos2, offset: Vec2);
}

pub struct DrawCommandOrd {
    ord: i64,
    pub draw_command: Box<dyn DrawCommand>,
}

impl DrawCommandOrd {
    pub fn new(ord: i64, draw_command: Box<dyn DrawCommand>) -> Self {
        Self { ord, draw_command }
    }
}

impl Ord for DrawCommandOrd {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ord.cmp(&other.ord)
    }
}

impl PartialOrd for DrawCommandOrd {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for DrawCommandOrd {
    fn eq(&self, other: &Self) -> bool {
        self.ord == other.ord
    }
}

impl Eq for DrawCommandOrd {}
