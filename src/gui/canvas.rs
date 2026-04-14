use crate::App;
use crate::config;
use crate::gui::modal::ActionAfterExport;
use crate::gui::widget;
use crate::logic::app_file::open_file;
use crate::model::node_type::NODE_TYPES;

impl App {
    pub(super) fn gui_canvas(&mut self, ui: &mut egui::Ui) -> egui::Response {
        const CANVAS_SECONDARY_TOOLBAR_HEIGHT: f32 = 26.0;

        let is_benchmark_running = self.benchmark_data.is_running;

        // .: Canvas init :.
        // .:=============:.
        // Painter is our canvas
        let mut canvas_size = ui.available_size();
        if self.do_show_secondary_canvas_toolbar {
            canvas_size.y -= CANVAS_SECONDARY_TOOLBAR_HEIGHT;
        }
        let (response, painter) = ui.allocate_painter(canvas_size, egui::Sense::click_and_drag());
        let response_rect = response.rect;

        // .: User interaction :.
        // .:==================:.
        // RMB to move canvas ("scrolling")
        if !is_benchmark_running && response.dragged_by(egui::PointerButton::Secondary) {
            self.scrolling += response.drag_delta();
            ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
        }

        // Origin ([0,0]) of the canvas in screen space coordinates, which painter uses
        let mut origin = response_rect.min + self.scrolling.to_vec2();

        let (mut pointer_pos_in_canvas, is_pointer_in_canvas) =
            if let Some(pointer_pos) = response.hover_pos() {
                (pointer_pos - origin, true)
            } else {
                (egui::Vec2::default(), false)
            };

        if !is_benchmark_running && response.hovered() {
            // MW to zoom
            let scroll = ui.input(|i| {
                i.events.iter().find_map(|e| match e {
                    egui::Event::MouseWheel {
                        unit: _,
                        delta,
                        modifiers: _,
                    } => Some(*delta),
                    _ => None,
                })
            });
            if let Some(scroll) = scroll {
                let old_zoom = self.zoom_level;

                self.set_canvas_font_size_and_zoom(
                    self.canvas_font_size.saturating_add_signed(
                        scroll.y as i32 * config::FONT_SIZE_CANVAS_STEP as i32,
                    ),
                );

                // Zoom anchor under mouse
                if old_zoom != self.zoom_level {
                    let ratio = self.zoom_level / old_zoom;
                    self.scrolling += pointer_pos_in_canvas * (1.0 - ratio);
                    // Scrolling has been changed, we have to update origin and pointer_pos for later use
                    origin = response_rect.min + self.scrolling.to_vec2();
                    if let Some(pointer_pos) = response.hover_pos() {
                        pointer_pos_in_canvas = pointer_pos - origin;
                    }
                }
            }
        }

        // If we are creating a SVG this frame, we reset scrolling and zoom_level here so we don't have have to "revert it" in the SVG.
        // This is the place to do it because we already handled the user interaction this frame (RMB scroll and MW zoom).
        if self.do_svg_export_this_iter {
            self.reset_canvas_scrolling_and_zoom();
        }

        // .: Draw on canvas :.
        // .:================:.
        // == Draw grid ==
        if self.do_show_grid {
            let grid_step = config::GRID_STEP_BASE * self.zoom_level;
            let grid_stroke = egui::Stroke::new(1.0, config::COLOR_GRID_LINE);

            let mut x = self.scrolling.x.rem_euclid(grid_step);
            while x < response_rect.width() {
                painter.vline(
                    response_rect.left() + x,
                    response_rect.y_range(),
                    grid_stroke,
                );
                x += grid_step;
            }

            let mut y = self.scrolling.y.rem_euclid(grid_step);
            while y < response_rect.height() {
                painter.hline(
                    response_rect.x_range(),
                    response_rect.top() + y,
                    grid_stroke,
                );
                y += grid_step;
            }
        }

        // == Draw diagram ==
        self.canvas_nodes.clear();
        self.gui_canvas_prepare_nodes(&painter, &origin);
        self.gui_canvas_prepare_paths(&painter, &origin);

        if self.do_svg_export_this_iter {
            self.svg_exporter.apply_boundaries();
        }

        while !self.draw_commands_ord.is_empty() {
            if let Some(draw_command_ord) = self.draw_commands_ord.pop() {
                draw_command_ord.draw_command.draw(&painter);

                if self.do_svg_export_this_iter {
                    draw_command_ord.draw_command.draw_svg(
                        &mut self.svg_exporter.svg_document,
                        self.svg_exporter.offset,
                    );
                }
            }
        }

        // Everything is ready for SVG export, if user pressed Export in previous iteration
        if self.do_svg_export_this_iter {
            self.do_svg_export_this_iter = false;

            if let Err(err) = self.svg_exporter.save(&self.modal_export_path) {
                self.show_error_modal(&err.to_string());
            } else {
                match self.modal_export_action_choice {
                    ActionAfterExport::DoNothing => (),
                    ActionAfterExport::OpenFolder => {
                        showfile::show_path_in_file_manager(&self.modal_export_path);
                    }
                    ActionAfterExport::OpenFile => {
                        if let Err(err) = open_file(&self.modal_export_path) {
                            self.show_error_modal(&err.to_string());
                        }
                    }
                };
            }
        }

        // .: User AABR interaction :.
        // .:=======================:.

        // == Clicking on nodes in canvas ==
        // If selected node is removed from the TOML source, unselect it
        if self.is_canvas_node_selected
            && let None = self.parser.result_nodes.get(&self.selected_canvas_node_key)
        {
            self.is_canvas_node_selected = false;
        }
        // Check for nodes under the pointer
        if is_pointer_in_canvas {
            let mut hovered_z_mul = -1;
            self.hovered_canvas_node_key = None;
            for (key, value) in &self.canvas_nodes {
                if value.z_mul > hovered_z_mul && value.is_point_inside_incl(pointer_pos_in_canvas)
                {
                    hovered_z_mul = value.z_mul;
                    self.hovered_canvas_node_key = Some(String::from(key));
                }
            }
            // LMB to (de)select node
            if response.clicked() {
                if let Some(hover_key) = &self.hovered_canvas_node_key {
                    self.is_canvas_node_selected = true;
                    self.selected_canvas_node_key = hover_key.clone();
                } else {
                    self.is_canvas_node_selected = false;
                }
            }
        } else {
            self.hovered_canvas_node_key = None;
        }

        // == Drag n drop new node logic ==
        if self.is_dragndropping_node {
            // We ignore scrolling here, we are checking if pointer is in part of the window
            let pointer_pos = ui
                .input(|i| i.pointer.latest_pos())
                .unwrap_or(egui::Pos2::default());

            // Do not draw the ghost node if in this position releasing LMB won't place it
            if response.rect.contains(pointer_pos) {
                // Draw the "ghost node"
                let label = format!("node_{}", self.canvas_nodes.len());
                let offset = self.gui_canvas_draw_ghost_node(&painter, &label, pointer_pos);

                // Check if LMB released inside the canvas
                if ui.input(|i| i.pointer.button_released(egui::PointerButton::Primary)) {
                    self.is_dragndropping_node = false;
                    // Add new node to canvas (TOML values are zoom level independent so we divide by that)
                    let node_x = ((pointer_pos_in_canvas.x - offset.x) / self.zoom_level) as i64;
                    let node_y = ((pointer_pos_in_canvas.y - offset.y) / self.zoom_level) as i64;
                    self.source += &format!(
                        "\n[node.{}]\ntype = {}\nxy = [{}, {}]\n",
                        label,
                        self.dragndropping_node_type.as_quoted_string(),
                        node_x,
                        node_y
                    );
                    // Don't forget to mark as dirty
                    self.is_source_dirty = true;
                    // Make the new node selected
                    self.is_canvas_node_selected = true;
                    self.selected_canvas_node_key = label;
                }
            } else {
                // Check if LMB released outside the canvas
                if ui.input(|i| i.pointer.button_released(egui::PointerButton::Primary)) {
                    self.is_dragndropping_node = false;
                }
            }
        }

        // .: Secondary canvas toolbar :.
        // .:==========================:.
        if self.do_show_secondary_canvas_toolbar {
            ui.add_enabled_ui(!is_benchmark_running, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(widget::TINYSKIP);

                    // == Add node buttons ==
                    for tup in NODE_TYPES {
                        let response = ui.button(tup.0).on_hover_text(format!(
                            "Drag and drop me onto the canvas to add a '{}' node.",
                            tup.2
                        ));

                        if !self.is_dragndropping_node && response.is_pointer_button_down_on() {
                            self.dragndropping_node_type = tup.1;
                            self.is_dragndropping_node = true;
                        }
                    }

                    // == Zoom level slider ==
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(widget::TINYSKIP);

                        let response = ui.add(
                            egui::Slider::new(
                                &mut self.canvas_font_size,
                                config::FONT_SIZE_CANVAS_MIN..=config::FONT_SIZE_CANVAS_MAX,
                            )
                            .integer()
                            .step_by(config::FONT_SIZE_CANVAS_STEP as f64)
                            .show_value(false)
                            .trailing_fill(true),
                        );

                        if response.changed() {
                            self.update_canvas_zoom();
                        }

                        ui.add(egui::Label::new(
                            egui::RichText::new(format!("Zoom level: {:.2}", self.zoom_level))
                                .color(egui::Color32::from_gray(
                                    if !self.style_is_light_mode
                                        && !self.style_do_force_light_canvas
                                    {
                                        233
                                    } else {
                                        27
                                    },
                                )),
                        ));
                    });
                });
            });
        }

        // --- ---
        response
    }

    pub fn set_canvas_font_size_and_zoom(&mut self, new_font_size: u32) {
        self.canvas_font_size =
            new_font_size.clamp(config::FONT_SIZE_CANVAS_MIN, config::FONT_SIZE_CANVAS_MAX);
        self.update_canvas_zoom();
    }

    /// Updates `zoom_level` to proper value after `canvas_font_size` is changed from code
    pub fn update_canvas_zoom(&mut self) {
        self.zoom_level = self.canvas_font_size as f32 / config::FONT_SIZE_CANVAS_BASE as f32;
    }

    pub fn reset_canvas_scrolling_and_zoom(&mut self) {
        self.scrolling = config::SCROLLING_DEFAULT;
        self.set_canvas_font_size_and_zoom(config::FONT_SIZE_CANVAS_BASE);
    }
}
