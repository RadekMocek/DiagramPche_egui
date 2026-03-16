use crate::App;
use crate::helper::draw::{vec2_normalized, vec2_orthogonalized};
use crate::helper::draw_layer::{DLPriority, dl_user_channel_to_real_channel};
use crate::model::draw_command::command::DrawCommandOrd;
use crate::model::draw_command::node_rectangle::NodeRectangleDrawCommand;
use crate::model::draw_command::path::PathDrawCommand;
use crate::model::pathpoint_type::PathpointType;
use egui::{Painter, Pos2, pos2};
use std::sync::Arc;

impl App {
    pub(super) fn gui_canvas_prepare_paths(&mut self, painter: &Painter, origin: &Pos2) {
        for path in &self.parser.result_paths {
            // ---- Prepare for possible path label(s) --- --- --- --- --- --- --- --- --- ---
            // (There may be multiple path labels on the same path,if it has multiple ends,
            // but their text and background color is always the same)
            let path_label_galley = painter.layout_no_wrap(
                path.label_value.clone(),
                egui::FontId::monospace(self.canvas_font_size as f32),
                egui::Color32::PLACEHOLDER,
            );
            let path_label_rect_size = path_label_galley.rect.size();
            // --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- ---

            // Prepare the start point
            let mut start = pos2(
                path.start.x as f32 * self.zoom_level,
                path.start.y as f32 * self.zoom_level,
            );

            let mut do_start_shift = false;
            if !path.start.parent_id.is_empty()
                && let Some(parent_node) = self.canvas_nodes.get(&path.start.parent_id)
            {
                // Move start point so it's relative to parent's pivot
                start += parent_node
                    .get_exact_point_from_pivot(&path.start.parent_pivot)
                    .to_vec2();
                // Path shift makes sense only when the start/end point is relative to some node
                if path.shift_start != 0 {
                    do_start_shift = true;
                }
            }

            // Start is now "originated" (takes canvas origin into account)
            start += origin.to_vec2();

            // One path can have multiple ends defined by the user => one [[path]] can define multiple result_paths.
            // Every result_path will be defined as vector of points Vec<Pos2> (start point, maybe some Pathpoints, end point).
            // Vector of these vectors will be given to the draw command.
            let mut result_paths: Vec<Vec<Pos2>>;

            // Each inner vector starts with the start point; or, if shift != 0 && start point is relative, with OG start point followed by a shifted start point
            if !do_start_shift {
                result_paths = vec![vec![start]; path.ends.len()]
            } else {
                let shifted_start =
                    start + path.get_shift_vector(&path.start.parent_pivot, self.zoom_level, true);

                result_paths = vec![vec![start, shifted_start]; path.ends.len()];

                start = shifted_start; // Do this so Pathpoints relative to start are relative to this
            };

            // Foreach end point
            for (index, path_end) in path.ends.iter().enumerate() {
                let result_pathpoints = &mut result_paths[index];

                // Ready the current end point
                let mut end = pos2(
                    path_end.x as f32 * self.zoom_level,
                    path_end.y as f32 * self.zoom_level,
                );

                let mut do_end_shift = false;
                if !path_end.parent_id.is_empty()
                    && let Some(parent_node) = self.canvas_nodes.get(&path_end.parent_id)
                {
                    end += parent_node
                        .get_exact_point_from_pivot(&path_end.parent_pivot)
                        .to_vec2();
                    if path.shift_end != 0 {
                        do_end_shift = true;
                    }
                }

                // End is now "originated"
                end += origin.to_vec2();

                // `shifted_end` is the end point, that all the Pathpoints relate to
                let shifted_end = if !do_end_shift {
                    // If there is no a shift, it is just the original end
                    end
                } else {
                    // If there is a shift, we apply it; we still remember the original end and in this case it will be the last point added to current collection
                    end + path.get_shift_vector(&path_end.parent_pivot, self.zoom_level, false)
                };

                // Pathpoints (defined as a collection [[path]].points) are points between start and end.
                // They are not mandatory: if no Pathpoints are specified, then path is just a single line from start to end.
                // If there are some, we iterate them and add them to the result collection.
                let mut prev = start; // This is for the "prev" to work (Pathpoint relative to previous Pathpoint)

                for pathpoint in &path.pathpoints {
                    // Currently processed Pathpoint (not "originated" yet)
                    let mut curr = pos2(
                        pathpoint.x as f32 * self.zoom_level,
                        pathpoint.y as f32 * self.zoom_level,
                    );
                    // Apply the Pathpoint type for both coordinates
                    // X
                    match pathpoint.x_type {
                        // AABRs in `self.canvas_nodes` are stored "zoomed and absolute", so they take zoom_level into account, but not origin.
                        // So we have to add origin here, later code depends on "originated" Pathpoint.
                        PathpointType::Reference => {
                            if let Some(parent_node) = self.canvas_nodes.get(&pathpoint.x_parent_id)
                            {
                                curr.x += parent_node
                                    .get_exact_point_from_pivot(&pathpoint.x_parent_pivot)
                                    .x
                                    + origin.x;
                            }
                        }
                        // Absolute coordinates are, by definition, not "originated"
                        PathpointType::Absolute => curr.x += origin.x,
                        // Start, End, and Prev are "originated", so we mustn't add origin here
                        PathpointType::Start => curr.x += start.x,
                        PathpointType::End => curr.x += shifted_end.x,
                        PathpointType::Previous => curr.x += prev.x,
                    }
                    // Same for Y
                    match pathpoint.y_type {
                        PathpointType::Reference => {
                            if let Some(parent_node) = self.canvas_nodes.get(&pathpoint.y_parent_id)
                            {
                                curr.y += parent_node
                                    .get_exact_point_from_pivot(&pathpoint.y_parent_pivot)
                                    .y
                                    + origin.y;
                            }
                        }
                        PathpointType::Absolute => curr.y += origin.y,
                        PathpointType::Start => curr.y += start.y,
                        PathpointType::End => curr.y += shifted_end.y,
                        PathpointType::Previous => curr.y += prev.y,
                    }

                    // Pathpoint is ready now
                    result_pathpoints.push(curr);

                    // Ready for the next iteration
                    prev = curr;
                }

                // After the Pathpoints are ready add the endpoint(s)
                result_pathpoints.push(shifted_end);
                if do_end_shift {
                    result_pathpoints.push(end);
                }

                // Path label (`label=` && `label_bg=`)
                let n_pathpoints = result_pathpoints.len();
                if n_pathpoints > 2 && !path.label_value.is_empty() {
                    // Path label is set in TOML as [string(1), int(2), int(3), int(4)]
                    // (1) is the label's text
                    // (2) is the point of the path on which the label is placed, use modulo to not get out of bounds
                    let label_point_curr_idx = path.label_point as usize % n_pathpoints;
                    // (3) is the shift of the label position to the next point on the path, get next point using modulo as well
                    let label_point_next_idx = (path.label_point as usize + 1) % n_pathpoints;
                    let label_shift = path.label_shift as f32;
                    // (4) is shift in a direction orthogonal to (3), so user can fine-tune the placement of the label on the path
                    let label_shift_orth = path.label_shift_orthogonal as f32;

                    // Get chosen point ("curr") and next point ("next"), and get the direction vector from curr to next
                    let label_point_curr = result_pathpoints[label_point_curr_idx];
                    let label_point_next = result_pathpoints[label_point_next_idx];
                    let direction = vec2_normalized(label_point_next - label_point_curr);

                    // Using the direction vector, we can apply the shifts
                    let label_position = label_point_curr
                        + (label_shift * self.zoom_level * direction)
                        + (label_shift_orth * self.zoom_level * vec2_orthogonalized(direction))
                        - (path_label_rect_size / 2.0);

                    // `label_bg=` can be set with color value to give background to the path label; background rectangle size == label size
                    // Make a draw command, we will use NodeRectangle for this
                    self.draw_commands_ord.push(DrawCommandOrd::new(
                        dl_user_channel_to_real_channel(path.z, DLPriority::PathLabel),
                        Box::new(NodeRectangleDrawCommand::new(
                            label_position,
                            label_position + path_label_rect_size,
                            path.label_bg_color.to_egui_color(),
                            egui::Color32::TRANSPARENT,
                            self.zoom_level,
                            label_position,
                            Arc::clone(&path_label_galley),
                        )),
                    ));
                }
            }

            // SVG export?
            if self.do_svg_export_this_iter {
                for result_path in &result_paths {
                    for result_point in result_path {
                        self.svg_exporter.update_boundaries(
                            result_point.x,
                            result_point.y,
                            result_point.x,
                            result_point.y,
                        );
                    }
                }
            }

            // Make a draw command
            self.draw_commands_ord.push(DrawCommandOrd::new(
                dl_user_channel_to_real_channel(path.z, DLPriority::Path),
                Box::new(PathDrawCommand::new(
                    result_paths,
                    path.color.to_egui_color(),
                    self.zoom_level,
                    path.do_start_arrow,
                    path.do_end_arrow,
                )),
            ));
        }
    }
}
