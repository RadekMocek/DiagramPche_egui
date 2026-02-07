use crate::helper::draw_layer::dl_user_channel_to_real_channel;
use crate::model::draw_command::command::DrawCommandOrd;
use crate::model::draw_command::path::PathDrawCommand;
use crate::model::pathpoint_type::PathpointType;
use crate::App;
use egui::{pos2, Pos2};

impl App {
    pub(super) fn gui_canvas_prepare_paths(&mut self, origin: &Pos2) {
        for path in &self.parser.result_paths {
            // Get the "simple" values from path
            let shift = path.shift;

            // Prepare the start point
            let mut start = pos2(
                path.start.x as f32 * self.zoom_level,
                path.start.y as f32 * self.zoom_level,
            );

            let mut do_start_shift = false;
            if !path.start.parent_id.is_empty()
                && let Some(parent_node) = self.canvas_nodes.get(&path.start.parent_id)
            {
                start += parent_node
                    .get_exact_point_from_pivot(&path.start.parent_pivot)
                    .to_vec2();
                if shift != 0 {
                    do_start_shift = true;
                }
            }

            // Start is now "originated" (takes canvas origin into account)
            start += origin.to_vec2();

            // One path can have multiple ends defined by the user => one [[path]] can define multiple result_paths.
            // Every result_path will be defined as vector of points Vec<Pos2> (start point, maybe some Pathpoints, end point).
            // Vector of these vectors will be given to the draw command.
            let mut result_paths: Vec<Vec<Pos2>>;

            // Each inner vector starts with the start point, or OG start point followed by a shifted start point, if shift != 0 && start point is relative
            if !do_start_shift {
                result_paths = vec![vec![start]; path.ends.len()]
            } else {
                let shifted_start =
                    start + path.get_shift_vector(&path.start.parent_pivot, self.zoom_level);

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
                    if shift != 0 {
                        do_end_shift = true;
                    }
                }

                // End is now "originated"
                end += origin.to_vec2();

                let shifted_end = if !do_end_shift {
                    end
                } else {
                    end + path.get_shift_vector(&path_end.parent_pivot, self.zoom_level)
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

                    // Pathpoint is prepared
                    result_pathpoints.push(curr);

                    // Ready for the next iteration
                    prev = curr;
                }

                // After the Pathpoints are ready...

                // This is the end point Pathpoints related to
                result_pathpoints.push(shifted_end);

                // If there was a shift at play, we still have a "real" end point to push
                if do_end_shift {
                    result_pathpoints.push(end);
                }
            }

            // Make a draw command
            self.draw_commands_ord.push(DrawCommandOrd::new(
                dl_user_channel_to_real_channel(path.z, false),
                Box::new(PathDrawCommand::new(
                    result_paths,
                    path.color.to_egui_color(),
                    path.do_start_arrow,
                    path.do_end_arrow,
                )),
            ));
        }
    }
}
