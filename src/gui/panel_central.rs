use crate::App;
use crate::gui::widget;
use crate::model::color::get_rgba_hex_quoted_from_u8arr;
use crate::model::node_type::NodeType;
use crate::model::node_type::{NODE_TYPE_CHOICES, get_node_type_quoted_string_from_usize};

impl App {
    pub fn gui_panel_central(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |mut ui| {
            let available_space = ui.available_rect_before_wrap();
            let split_position =
                available_space.left() + available_space.width() * self.central_split_ratio;

            const SEPARATOR_HALF_WIDTH: f32 = 8.0 / 2.0;

            // Left panel :: text editor
            let left_rect = egui::Rect::from_min_max(
                available_space.min,
                egui::pos2(split_position - SEPARATOR_HALF_WIDTH, available_space.max.y),
            );
            // Right panel :: canvas
            let right_rect = egui::Rect::from_min_max(
                egui::pos2(split_position + SEPARATOR_HALF_WIDTH, available_space.min.y),
                available_space.max,
            );
            // Separator
            let separator_rect = egui::Rect::from_min_max(
                egui::pos2(split_position - SEPARATOR_HALF_WIDTH, available_space.min.y),
                egui::pos2(split_position + SEPARATOR_HALF_WIDTH, available_space.max.y),
            );
            let separator_response = ui.interact(
                separator_rect,
                ui.id().with("separator"),
                egui::Sense::drag(),
            );

            // Handle separator dragging
            if separator_response.dragged() {
                self.central_split_ratio = (self.central_split_ratio
                    + separator_response.drag_delta().x / available_space.width())
                .clamp(0.1, 0.9);
            }

            // Change cursor when hovering separator
            if separator_response.hovered() || separator_response.dragged() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
            }

            // Draw left panel (text editor)
            let mut left_ui = ui.new_child(egui::UiBuilder::new().max_rect(left_rect));
            left_ui.set_clip_rect(left_rect);

            // Left toolbar
            if self.do_show_toolbar {
                left_ui.horizontal(|mut ui| {
                    ui.add_space(widget::TINYSKIP);
                    self.widget_text_editor_font_size_setup(&mut ui);
                    ui.separator();
                    ui.label(format!(
                        "Cursor pos: {}, {}",
                        self.editor_cursor_line, self.editor_cursor_column
                    ));
                });

                left_ui.add_space(widget::TINYSKIP);
            }

            // Text editor
            egui::ScrollArea::both()
                .id_salt("source")
                .auto_shrink(false)
                .show(&mut left_ui, |ui| {
                    if !self.do_use_alt_editor {
                        self.gui_text_editor(ui);
                    } else {
                        self.gui_text_editor_alt(ui);
                    }
                });

            // Draw right panel (canvas)
            let mut right_ui = ui.new_child(egui::UiBuilder::new().max_rect(right_rect));

            // Right toolbar
            if self.do_show_toolbar {
                let node_span;
                let mut color;
                let color_span;
                let node_type;
                let node_type_span;
                let label_value;

                if self.is_canvas_node_selected
                    && let Some(node) = self.parser.result_nodes.get(&self.selected_canvas_node_key)
                {
                    node_span = &node.node_span;
                    color = node.color.to_picker_arr();
                    color_span = &node.color_span;
                    node_type = &node.node_type;
                    node_type_span = &node.type_span;
                    label_value = &self.selected_canvas_node_key;
                } else {
                    if let Some(hover_key) = &self.selected_or_hovered_canvas_node_key
                        && let Some(node) = self.parser.result_nodes.get(hover_key)
                    {
                        label_value = &node.id;
                        color = node.color.to_picker_arr();
                        node_type = &node.node_type;
                    } else {
                        label_value = &self.no_node_hovered_string;
                        color = [240, 240, 240, 255];
                        node_type = &NodeType::Rectangle;
                    }
                    node_span = &None;
                    color_span = &None;
                    node_type_span = &None;
                }

                right_ui.horizontal(|ui| {
                    ui.add_enabled_ui(self.is_canvas_node_selected, |ui| {
                        ui.add_space(widget::TINYSKIP);
                        // .: Color picker :.
                        ui.label("Node color:");

                        let color_response = ui.color_edit_button_srgba_unmultiplied(&mut color);
                        if color_response.changed() {
                            if let Some(color_span) = color_span {
                                self.source.replace_range(
                                    color_span.clone(),
                                    &get_rgba_hex_quoted_from_u8arr(color),
                                );
                            } else if let Some(node_span) = node_span {
                                self.source.insert_str(
                                    node_span.end,
                                    &format!("\ncolor = {}", get_rgba_hex_quoted_from_u8arr(color)),
                                );
                            }
                        }

                        ui.separator();

                        // .: Node type combo :.
                        ui.label("Type:");
                        let previous_choice_idx = node_type.as_usize();
                        let mut current_choice_idx = previous_choice_idx;
                        egui::ComboBox::from_id_salt("NodeTypeCombo")
                            .selected_text(NODE_TYPE_CHOICES[current_choice_idx])
                            .show_ui(ui, |ui| {
                                for (i, node_type) in NODE_TYPE_CHOICES.iter().enumerate() {
                                    ui.selectable_value(&mut current_choice_idx, i, *node_type);
                                }
                            });

                        if previous_choice_idx != current_choice_idx {
                            let type_string =
                                get_node_type_quoted_string_from_usize(current_choice_idx);

                            if let Some(node_type_span) = node_type_span {
                                self.source
                                    .replace_range(node_type_span.clone(), &type_string);
                            } else if let Some(node_span) = node_span {
                                self.source
                                    .insert_str(node_span.end, &format!("\ntype = {type_string}"));
                            }
                        }

                        ui.separator();

                        // .: Node ID label :.
                        ui.label(format!("ID: {}", label_value));
                    });
                });
                right_ui.add_space(widget::TINYSKIP);
            }

            // Canvas
            let do_fill_canvas = self.style_is_light_mode || self.style_do_force_light_canvas;

            egui::Frame::canvas(&right_ui.style())
                .fill(if do_fill_canvas {
                    crate::config::COLOR_CANVAS_BACKGROUND
                } else {
                    egui::Color32::TRANSPARENT
                })
                .show(&mut right_ui, |ui| {
                    self.gui_canvas(ui);
                });

            // --- --- --- --- --- ---

            // Modeless windows logic
            self.gui_window(&mut ui);

            // Modals logic
            self.gui_modal(&mut ui);
        });
    }
}
