use crate::logic::svg_exporter::{
    add_text_to_svg_document, egui_color32_to_svg_rgb, egui_pos2_vec_to_svg_points_string,
};
use crate::model::draw_command::command::DrawCommand;
use egui::{Color32, Galley, Painter, Pos2, Stroke, Vec2};
use std::sync::Arc;
use svg::{Document, Node};

pub struct NodeDiamondDrawCommand {
    points: [Pos2; 4],
    color_fill: Color32,
    color_edge: Color32,
    zoom_level: f32,
    label_position: Pos2,
    label_galley: Arc<Galley>,
}

impl NodeDiamondDrawCommand {
    pub fn new(
        points: [Pos2; 4],
        color_fill: Color32,
        color_edge: Color32,
        zoom_level: f32,
        label_position: Pos2,
        label_galley: Arc<Galley>,
    ) -> Self {
        Self {
            points,
            color_fill,
            color_edge,
            zoom_level,
            label_position,
            label_galley,
        }
    }
}

impl DrawCommand for NodeDiamondDrawCommand {
    fn draw(&self, painter: &Painter) {
        painter.add(egui::epaint::PathShape::convex_polygon(
            Vec::from(self.points),
            self.color_fill,
            Stroke::new(self.zoom_level, self.color_edge),
        ));

        painter.galley(
            self.label_position,
            self.label_galley.clone(),
            Color32::BLACK,
        );
    }

    fn draw_svg(&self, document: &mut Document, offset: Vec2) {
        // == SVG diamond ==
        document.append(
            svg::node::element::Polygon::new()
                .set(
                    "points",
                    egui_pos2_vec_to_svg_points_string(&Vec::from(self.points), offset),
                )
                .set("fill", egui_color32_to_svg_rgb(self.color_fill))
                .set("stroke", egui_color32_to_svg_rgb(self.color_edge))
                .set("stroke-width", "1")
                .set(
                    "style",
                    format!(
                        "fill-opacity:{}; stroke-opacity:{}",
                        self.color_fill.a() as f32 / 255.0,
                        self.color_edge.a() as f32 / 255.0
                    ),
                ),
        );

        // == SVG text ==
        add_text_to_svg_document(
            document,
            self.label_position,
            offset,
            Arc::clone(&self.label_galley),
        );
    }
}
