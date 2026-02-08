use crate::model::pivot::Pivot;
use egui::{pos2, Pos2, Vec2};

pub struct CanvasNode {
    top_left: Pos2,
    bottom_right: Pos2,
    center: Pos2,
}

impl CanvasNode {
    pub fn new(top_left: Pos2, bottom_right: Pos2, center: Pos2) -> Self {
        Self {
            top_left,
            bottom_right,
            center,
        }
    }

    pub fn get_exact_point_from_pivot(&self, pivot: &Pivot) -> Pos2 {
        match pivot {
            Pivot::TopLeft => self.top_left,
            Pivot::Top => pos2(self.center.x, self.top_left.y),
            Pivot::TopRight => pos2(self.bottom_right.x, self.top_left.y),
            Pivot::Right => pos2(self.bottom_right.x, self.center.y),
            Pivot::BottomRight => self.bottom_right,
            Pivot::Bottom => pos2(self.center.x, self.bottom_right.y),
            Pivot::BottomLeft => pos2(self.top_left.x, self.bottom_right.y),
            Pivot::Left => pos2(self.top_left.x, self.center.y),
            Pivot::Center => self.center,
        }
    }

    pub fn is_point_inside_incl(&self, point: Vec2) -> bool {
        point.x >= self.top_left.x
            && point.x <= self.bottom_right.x
            && point.y >= self.top_left.y
            && point.y <= self.bottom_right.y
    }
}
