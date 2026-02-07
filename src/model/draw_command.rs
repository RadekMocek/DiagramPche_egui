use egui::{Color32, Galley, Painter, Pos2};
use std::cmp::Ordering;
use std::sync::Arc;

pub struct DrawCommandOrd {
    pub ord: i64,
    pub draw_command: Box<dyn DrawCommand>,
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

pub trait DrawCommand {
    fn draw(&self, painter: &Painter, zoom_level: f32);
}

pub struct NodeRectangleDrawCommand {
    pub rect_top_left: Pos2,
    pub rect_bottom_right: Pos2,
    pub rect_color: Color32,

    pub label_position: Pos2,
    pub label_galley: Arc<Galley>,
}

impl DrawCommand for NodeRectangleDrawCommand {
    fn draw(&self, painter: &Painter, zoom_level: f32) {
        painter.rect(
            egui::Rect::from_min_max(self.rect_top_left, self.rect_bottom_right),
            0,
            self.rect_color,
            egui::Stroke::new(zoom_level, Color32::BLACK),
            egui::StrokeKind::Inside,
        );

        painter.galley(
            self.label_position,
            self.label_galley.clone(),
            Color32::BLACK,
        );
    }
}
