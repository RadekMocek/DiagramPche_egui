use crate::logic::svg_exporter::add_text_to_svg_document;
use crate::model::draw_command::command::DrawCommand;
use egui::{Color32, Galley, Painter, Pos2, Vec2};
use std::sync::Arc;
use svg::Document;

pub struct NodeTextDrawCommand {
    label_position: Pos2,
    label_galley: Arc<Galley>,
}

impl NodeTextDrawCommand {
    pub fn new(label_position: Pos2, label_galley: Arc<Galley>) -> Self {
        Self {
            label_position,
            label_galley,
        }
    }
}

impl DrawCommand for NodeTextDrawCommand {
    fn draw(&self, painter: &Painter) {
        painter.galley(
            self.label_position,
            self.label_galley.clone(),
            Color32::BLACK,
        );
    }

    fn draw_svg(&self, document: &mut Document, offset: Vec2) {
        add_text_to_svg_document(
            document,
            self.label_position,
            offset,
            Arc::clone(&self.label_galley),
        );
    }
}
