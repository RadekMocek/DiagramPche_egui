use egui::epaint::{PathShape, PathStroke};
use egui::{Color32, Painter, Pos2, Vec2, vec2};

fn vec2_normalized(vec: Vec2) -> Vec2 {
    let magnitude = ((vec.x * vec.x) + (vec.y * vec.y)).sqrt();
    vec2(vec.x / magnitude, vec.y / magnitude)
}

fn vec2_orthogonalized(vec: Vec2) -> Vec2 {
    vec2(-vec.y, vec.x)
}

pub fn draw_arrow_tip(painter: &Painter, p1: Pos2, p2: Pos2, zoom_level: f32, color: Color32) {
    // TODO magic numbers
    let p2_to_p1 = vec2_normalized(p1 - p2);
    let point_slightly_before_p2 = p2 + p2_to_p1 * 12.0 * zoom_level;
    let p2_orthogonal_addition = vec2_orthogonalized(p2_to_p1) * 4.0 * zoom_level;
    painter.add(PathShape::convex_polygon(
        vec![
            p2,
            point_slightly_before_p2 - p2_orthogonal_addition,
            point_slightly_before_p2 + p2_orthogonal_addition,
        ],
        color,
        PathStroke::NONE,
    ));
}
