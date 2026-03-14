use crate::model::pivot::Pivot;
use egui::{Pos2, Vec2, pos2};

pub struct CanvasNode {
    top_left: Pos2,
    bottom_right: Pos2,
    center: Pos2,
    // Another metric to determine node's z-value. This one is used while interacting with nodes through the canvas.
    // If more nodes are on top of each other on the z-axis, we need to determine one, that will be chosen e.g. on click.
    // Preferably the one that was drawn last in the imdrawlist channel with biggest layer.
    // With this number, bigger means better ("closer" to the cursor). This uses the same logic as in the Dear ImGui project,
    // but here we are creating draw commands in new pq, so, theoretically, this value might not correspond to the order in which the elements were drawn.
    pub z_mul: i64,
}

impl CanvasNode {
    pub fn new(top_left: Pos2, bottom_right: Pos2, center: Pos2, z_mul: i64) -> Self {
        Self {
            top_left,
            bottom_right,
            center,
            z_mul,
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
