use crate::logic::svg_exporter::{add_text_to_svg_document, egui_color32_to_svg_rgb};
use crate::model::draw_command::command::DrawCommand;
use egui::epaint::EllipseShape;
use egui::{Color32, Galley, Painter, Pos2, Stroke, Vec2};
use std::sync::Arc;
use svg::{Document, Node};

pub struct NodeEllipseDrawCommand {
    center: Pos2,
    radius: Vec2,
    color_fill: Color32,
    color_edge: Color32,
    zoom_level: f32,
    label_position: Pos2,
    label_galley: Arc<Galley>,
}

impl NodeEllipseDrawCommand {
    pub fn new(
        center: Pos2,
        radius: Vec2,
        color_fill: Color32,
        color_edge: Color32,
        zoom_level: f32,
        label_position: Pos2,
        label_galley: Arc<Galley>,
    ) -> Self {
        Self {
            center,
            radius,
            color_fill,
            color_edge,
            zoom_level,
            label_position,
            label_galley,
        }
    }
}

impl DrawCommand for NodeEllipseDrawCommand {
    fn draw(&self, painter: &Painter) {
        painter.add(EllipseShape::filled(
            self.center,
            self.radius,
            self.color_fill,
        ));

        painter.add(EllipseShape::stroke(
            self.center,
            self.radius,
            Stroke::new(self.zoom_level, self.color_edge),
        ));

        painter.galley(
            self.label_position,
            self.label_galley.clone(),
            Color32::BLACK,
        );
    }

    fn draw_svg(&self, document: &mut Document, offset: Vec2) {
        // == SVG ellipse ==
        let center = self.center - offset;
        let radius = self.radius;

        document.append(
            svg::node::element::Ellipse::new()
                .set("cx", center.x)
                .set("cy", center.y)
                .set("rx", radius.x)
                .set("ry", radius.y)
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
