use crate::App;
use crate::helper::draw_layer::dl_user_channel_to_real_channel;
use crate::model::canvas_node::CanvasNode;
use crate::model::draw_command::command::DrawCommandOrd;
use crate::model::draw_command::node_rectangle::NodeRectangleDrawCommand;
use crate::model::pivot::Pivot;
use egui::{Painter, Pos2, pos2};

impl App {
    pub(super) fn gui_canvas_prepare_nodes(&mut self, painter: &Painter, origin: &Pos2) {
        let node_padding = crate::config::NODE_BORDER_OFFSET_BASE * self.zoom_level;

        while !self.parser.result_order.is_empty() {
            if let Some(order_pair) = self.parser.result_order.pop()
                && let Some(node) = self.parser.result_nodes.get(&order_pair.1)
            {
                // Determine size of the label
                // (Lay out the text, ready it for drawing. After this we can get the size of label, which will be drawed.)
                let label_galley = painter.layout_no_wrap(
                    node.value.clone(),
                    egui::FontId::monospace(self.canvas_font_size as f32),
                    egui::Color32::PLACEHOLDER,
                );
                let label_rect = label_galley.rect;
                let label_size_x = label_rect.width();
                let label_size_y = label_rect.height();

                // Get explicit or calculate implicit node size
                let node_width = if node.width > 0 {
                    node.width as f32 * self.zoom_level
                } else {
                    label_size_x + 2.0 * node_padding
                };

                let node_height = if node.height > 0 {
                    node.height as f32 * self.zoom_level
                } else {
                    label_size_y + 2.0 * node_padding
                };

                // Get node position, this is from the line `xy = [number, number]`
                let node_x = node.position.x as f32 * self.zoom_level;
                let node_y = node.position.y as f32 * self.zoom_level;

                // Move node according to its parent, if the user had set some
                let parent_offset = if !node.position.parent_id.is_empty()
                    && let Some(parent_node) = self.canvas_nodes.get(&node.position.parent_id)
                {
                    parent_node.get_exact_point_from_pivot(&node.position.parent_pivot)
                } else {
                    pos2(0.0, 0.0)
                };

                // Move node according to its `pivot`, if the user had set some
                let pivot_offset = match node.pivot {
                    Pivot::TopLeft => pos2(0.0, 0.0),
                    Pivot::Top => pos2(-node_width / 2.0, 0.0),
                    Pivot::TopRight => pos2(-node_width, 0.0),
                    Pivot::Right => pos2(-node_width, -node_height / 2.0),
                    Pivot::BottomRight => pos2(-node_width, -node_height),
                    Pivot::Bottom => pos2(-node_width / 2.0, -node_height),
                    Pivot::BottomLeft => pos2(0.0, -node_height),
                    Pivot::Left => pos2(0.0, -node_height / 2.0),
                    Pivot::Center => pos2(-node_width / 2.0, -node_height / 2.0),
                };

                // Calculate and store the AABR
                let aabr_top_left = pos2(
                    node_x + parent_offset.x + pivot_offset.x,
                    node_y + parent_offset.y + pivot_offset.y,
                );
                let aabr_bottom_right =
                    pos2(aabr_top_left.x + node_width, aabr_top_left.y + node_height);
                let aabr_center = pos2(
                    aabr_top_left.x + node_width / 2.0,
                    aabr_top_left.y + node_height / 2.0,
                );

                self.canvas_nodes.insert(
                    node.id.clone(),
                    CanvasNode::new(aabr_top_left, aabr_bottom_right, aabr_center),
                );

                // By adding origin (canvas position in window + scrolling) to AABR we get proper drawing coordinates
                let draw_top_left = *origin + aabr_top_left.to_vec2();
                let draw_bottom_right = *origin + aabr_bottom_right.to_vec2();
                let draw_center = *origin + aabr_center.to_vec2(); // Helper variable for custom label placement inside a node

                // Do the drawing preparation of the rectangle
                // Prepare the label
                let label_left_x = draw_top_left.x + node_padding;
                let label_top_y = draw_top_left.y + node_padding;
                let draw_label_position_default = pos2(label_left_x, label_top_y);
                let draw_label_position = if node_width > 0.0 || node_height > 0.0 {
                    match node.label_position {
                        Pivot::TopLeft => draw_label_position_default,
                        Pivot::Top => pos2(draw_center.x - label_size_x / 2.0, label_top_y),
                        Pivot::TopRight => pos2(
                            draw_bottom_right.x - label_size_x - node_padding,
                            label_top_y,
                        ),
                        Pivot::Right => pos2(
                            draw_bottom_right.x - label_size_x - node_padding,
                            draw_center.y - label_size_y / 2.0,
                        ),
                        Pivot::BottomRight => pos2(
                            draw_bottom_right.x - label_size_x - node_padding,
                            draw_bottom_right.y - label_size_y - node_padding,
                        ),
                        Pivot::Bottom => pos2(
                            draw_center.x - label_size_x / 2.0,
                            draw_bottom_right.y - label_size_y - node_padding,
                        ),
                        Pivot::BottomLeft => pos2(
                            label_left_x,
                            draw_bottom_right.y - label_size_y - node_padding,
                        ),
                        Pivot::Left => pos2(label_left_x, draw_center.y - label_size_y / 2.0),
                        Pivot::Center => pos2(
                            draw_center.x - label_size_x / 2.0,
                            draw_center.y - label_size_y / 2.0,
                        ),
                    }
                } else {
                    draw_label_position_default
                };

                self.draw_commands_ord.push(DrawCommandOrd::new(
                    dl_user_channel_to_real_channel(node.z, true),
                    Box::new(NodeRectangleDrawCommand::new(
                        draw_top_left,
                        draw_bottom_right,
                        node.color.to_egui_color(),
                        self.zoom_level,
                        draw_label_position,
                        label_galley,
                    )),
                ));

                if self.do_svg_export_this_iter {
                    self.svg_exporter.update_boundaries(
                        aabr_top_left.x,
                        aabr_top_left.y,
                        aabr_bottom_right.x,
                        aabr_bottom_right.y,
                    );
                }
            }
        }
    }
}
