use crate::config::{TIP_ARROW_LENGTH, TIP_ARROW_SPAN};
use crate::helper::draw::draw_arrow_tip;
use crate::logic::svg_exporter::{egui_color32_to_svg_rgb, egui_pos2_to_svg_point};
use crate::model::draw_command::command::DrawCommand;
use egui::{Color32, Painter, Pos2, Vec2};
use svg::{Document, Node};

pub struct PathDrawCommand {
    paths: Vec<Vec<Pos2>>,
    color: Color32,
    zoom_level: f32,
    pub do_start_arrow: bool,
    pub do_end_arrow: bool,
}

impl PathDrawCommand {
    pub fn new(
        paths: Vec<Vec<Pos2>>,
        color: Color32,
        zoom_level: f32,
        do_start_arrow: bool,
        do_end_arrow: bool,
    ) -> Self {
        Self {
            paths,
            color,
            zoom_level,
            do_start_arrow,
            do_end_arrow,
        }
    }

    fn get_svg_arrow_tip(p1: Pos2, p2: Pos2, color: Color32) -> svg::node::element::Polygon {
        let p2_to_p1 = crate::helper::draw::vec2_normalized(p1 - p2);
        let point_slightly_before_p2 = p2 + p2_to_p1 * TIP_ARROW_LENGTH;
        let p2_orthogonal_addition =
            crate::helper::draw::vec2_orthogonalized(p2_to_p1) * TIP_ARROW_SPAN;
        svg::node::element::Polygon::new()
            .set(
                "points",
                format!(
                    "{} {} {}",
                    egui_pos2_to_svg_point(p2),
                    egui_pos2_to_svg_point(point_slightly_before_p2 - p2_orthogonal_addition),
                    egui_pos2_to_svg_point(point_slightly_before_p2 + p2_orthogonal_addition)
                ),
            )
            .set("fill", egui_color32_to_svg_rgb(color))
            .set(
                "style",
                format!("fill-opacity:{}", color.a() as f32 / 255.0),
            )
    }
}

impl DrawCommand for PathDrawCommand {
    fn draw(&self, painter: &Painter) {
        let stroke = egui::Stroke::new(self.zoom_level, self.color);

        for result_path in &self.paths {
            painter.line(result_path.clone(), stroke);

            if result_path.len() >= 2 {
                if self.do_start_arrow {
                    draw_arrow_tip(
                        painter,
                        result_path[1],
                        result_path[0],
                        self.zoom_level,
                        self.color,
                    );
                }
                if self.do_end_arrow {
                    draw_arrow_tip(
                        painter,
                        result_path[result_path.len() - 2],
                        result_path[result_path.len() - 1],
                        self.zoom_level,
                        self.color,
                    );
                }
            }
        }
    }

    fn draw_svg(&self, document: &mut Document, offset: Vec2) {
        // == SVG [[path]] ==
        for result_path in &self.paths {
            // == SVG path for each end point ==
            document.append(
                svg::node::element::Polyline::new()
                    .set(
                        "points",
                        result_path.iter().fold(String::new(), |acc, vec| {
                            format!("{} {},{}", acc, vec.x - offset.x, vec.y - offset.y)
                        }),
                    )
                    .set("fill", "none")
                    .set("stroke", egui_color32_to_svg_rgb(self.color))
                    .set("stroke-width", "1")
                    .set(
                        "style",
                        format!("stroke-opacity:{}", self.color.a() as f32 / 255.0),
                    ),
            );

            if result_path.len() >= 2 {
                // == SVG start arrow ==
                if self.do_start_arrow {
                    document.append(Self::get_svg_arrow_tip(
                        result_path[1] - offset,
                        result_path[0] - offset,
                        self.color,
                    ))
                }
                // == SVG end arrow ==
                if self.do_end_arrow {
                    document.append(Self::get_svg_arrow_tip(
                        result_path[result_path.len() - 2] - offset,
                        result_path[result_path.len() - 1] - offset,
                        self.color,
                    ))
                }
            }
        }
    }
}
