use crate::model::path::Path;
use crate::model::pathpoint_type::PathpointType;
use crate::model::pivot::Pivot;
use crate::App;
use eframe::emath::Pos2;
use egui::{pos2, Painter, Stroke};
//TODO Proper arrows

impl App {
    pub(super) fn gui_canvas_draw_paths(&mut self, painter: &Painter, origin: &Pos2) {
        for path in &self.parser.result_paths {
            let stroke = Stroke::new(self.zoom_level, path.color.to_egui_color());
            let shift = path.shift;

            let mut start = pos2(
                path.start.x as f32 * self.zoom_level,
                path.start.y as f32 * self.zoom_level,
            );
            let mut is_start_arrow_satisfied = !path.do_start_arrow;
            if !path.start.parent_id.is_empty() {
                if let Some(parent_node) = self.canvas_nodes.get(&path.start.parent_id) {
                    start += parent_node
                        .get_exact_point_from_pivot(&path.start.parent_pivot)
                        .to_vec2();
                    if shift != 0 {
                        self.path_shift(
                            &mut start,
                            &mut is_start_arrow_satisfied,
                            path,
                            &path.start.parent_pivot,
                            painter,
                            origin,
                            &stroke,
                        );
                    }
                }
            }

            for path_end in &path.ends {
                let mut end = pos2(
                    path_end.x as f32 * self.zoom_level,
                    path_end.y as f32 * self.zoom_level,
                );
                let mut is_end_arrow_satisfied = !path.do_end_arrow;
                if !path_end.parent_id.is_empty() {
                    if let Some(parent_node) = self.canvas_nodes.get(&path_end.parent_id) {
                        end += parent_node
                            .get_exact_point_from_pivot(&path_end.parent_pivot)
                            .to_vec2();
                        if shift != 0 {
                            self.path_shift(
                                &mut end,
                                &mut is_end_arrow_satisfied,
                                path,
                                &path_end.parent_pivot,
                                painter,
                                origin,
                                &stroke,
                            );
                        }
                    }
                }

                let mut prev = start;
                for pathpoint in &path.pathpoints {
                    let mut curr = pos2(
                        pathpoint.x as f32 * self.zoom_level,
                        pathpoint.y as f32 * self.zoom_level,
                    );
                    match pathpoint.x_type {
                        PathpointType::Reference => {
                            if let Some(parent_node) = self.canvas_nodes.get(&pathpoint.x_parent_id)
                            {
                                curr.x += parent_node
                                    .get_exact_point_from_pivot(&pathpoint.x_parent_pivot)
                                    .x;
                            }
                        }
                        PathpointType::Absolute => {}
                        PathpointType::Start => curr.x += start.x,
                        PathpointType::End => curr.x += end.x,
                        PathpointType::Previous => curr.x += prev.x,
                    }
                    match pathpoint.y_type {
                        PathpointType::Reference => {
                            if let Some(parent_node) = self.canvas_nodes.get(&pathpoint.y_parent_id)
                            {
                                curr.y += parent_node
                                    .get_exact_point_from_pivot(&pathpoint.y_parent_pivot)
                                    .y;
                            }
                        }
                        PathpointType::Absolute => {}
                        PathpointType::Start => curr.y += start.y,
                        PathpointType::End => curr.y += end.y,
                        PathpointType::Previous => curr.y += prev.y,
                    }

                    let p1 = prev + origin.to_vec2();
                    let p2 = curr + origin.to_vec2();
                    painter.line_segment([p1, p2], stroke);

                    if !is_start_arrow_satisfied {
                        painter.arrow(p2, p1 - p2, stroke);
                        is_start_arrow_satisfied = true;
                    }

                    prev = curr;
                }

                let p1 = prev + origin.to_vec2();
                let p2 = end + origin.to_vec2();
                painter.line_segment([p1, p2], stroke);

                if !is_end_arrow_satisfied {
                    painter.arrow(p1, p2 - p1, stroke);
                    //is_end_arrow_satisfied = true; // Next iteration will set it to true so no need to do it here
                }

                if !is_start_arrow_satisfied {
                    painter.arrow(p2, p1 - p2, stroke);
                    is_start_arrow_satisfied = true;
                }
            }
        }
    }

    fn path_shift(
        &self,
        point: &mut Pos2,
        is_arrow_satisfied: &mut bool,
        path: &Path,
        pivot: &Pivot,
        painter: &Painter,
        origin: &Pos2,
        stroke: &Stroke,
    ) {
        // This will be the new position of the given point
        let shifted_start = *point + path.get_shift_direction(pivot, self.zoom_level);

        // Draw a line from the old position to the new position
        let p1 = *point + origin.to_vec2();
        let p2 = shifted_start + origin.to_vec2();
        painter.line_segment([p1, p2], *stroke);

        // Draw arrow if required
        if !*is_arrow_satisfied {
            painter.arrow(p2, p1 - p2, *stroke);
            *is_arrow_satisfied = true;
        }

        // Apply
        *point = shifted_start;
    }
}
