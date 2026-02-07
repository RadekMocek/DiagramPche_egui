use crate::helper::draw::draw_arrow_tip;
use crate::model::draw_command::command::DrawCommand;
use egui::{Color32, Painter, Pos2};

pub struct PathDrawCommand {
    paths: Vec<Vec<Pos2>>,
    color: Color32,
    pub do_start_arrow: bool,
    pub do_end_arrow: bool,
}

impl PathDrawCommand {
    pub fn new(
        paths: Vec<Vec<Pos2>>,
        color: Color32,
        do_start_arrow: bool,
        do_end_arrow: bool,
    ) -> Self {
        Self {
            paths,
            color,
            do_start_arrow,
            do_end_arrow,
        }
    }
}

impl DrawCommand for PathDrawCommand {
    fn draw(&self, painter: &Painter, zoom_level: f32) {
        let stroke = egui::Stroke::new(zoom_level, self.color);

        for result_path in &self.paths {
            painter.line(result_path.clone(), stroke);

            if self.do_end_arrow && result_path.len() >= 2 {
                draw_arrow_tip(
                    painter,
                    result_path[result_path.len() - 2],
                    result_path[result_path.len() - 1],
                    zoom_level,
                    self.color,
                );
            }
        }

        if self.do_start_arrow && self.paths.len() >= 1 && self.paths[0].len() >= 2 {
            draw_arrow_tip(
                painter,
                self.paths[0][1],
                self.paths[0][0],
                zoom_level,
                self.color,
            );
        }
    }
}
