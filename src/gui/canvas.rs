use crate::App;
use crate::config::*;
use crate::gui::modal::ActionAfterExport;
use crate::helper::icon::*;
use crate::logic::app_file::open_file;
use crate::model::node_type::NodeType;
use const_format::concatcp;
use crate::gui::widget;

const NODE_TYPES: [(&str, NodeType, &str); 4] = [
    (
        concatcp!(ICON_RECTANGLE_OUTLINE),
        NodeType::Rectangle,
        "rectangle",
    ),
    (
        concatcp!(ICON_ELLIPSE_OUTLINE),
        NodeType::Ellipse,
        "elllipse",
    ),
    (
        concatcp!(ICON_RHOMBUS_OUTLINE),
        NodeType::Diamond,
        "diamond",
    ),
    (concatcp!(ICON_FORMAT_TEXT_VARIANT), NodeType::Text, "text"),
];

impl App {
    pub(super) fn gui_canvas(&mut self, ui: &mut egui::Ui) -> egui::Response {
        const CANVAS_SECONDARY_TOOLBAR_HEIGHT: f32 = 26.0;

        // .: Canvas init :.
        // .:=============:.
        // Painter is our canvas
        let mut canvas_size = ui.available_size();
        if self.do_show_secondary_canvas_toolbar {
            canvas_size.y -= CANVAS_SECONDARY_TOOLBAR_HEIGHT;
        }
        let (response, painter) = ui.allocate_painter(canvas_size, egui::Sense::drag());
        let response_rect = response.rect;

        // .: User interaction :.
        // .:==================:.
        // RMB to move canvas ("scrolling")
        if response.dragged_by(egui::PointerButton::Secondary) {
            self.scrolling += response.drag_delta();
            ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
        }

        // Origin ([0,0]) of the canvas in screen space coordinates, which painter uses
        let mut origin = response_rect.min + self.scrolling.to_vec2();

        let /*mut*/ pointer_pos_in_canvas = if let Some(pointer_pos) = response.hover_pos() {
            pointer_pos - origin
        } else {
            egui::Vec2::default()
        };

        if response.hovered() {
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
                    self.canvas_font_size
                        .saturating_add_signed(scroll.y as i32 * FONT_SIZE_CANVAS_STEP as i32),
                );

                // Zoom anchor under mouse
                if old_zoom != self.zoom_level {
                    let ratio = self.zoom_level / old_zoom;
                    self.scrolling += pointer_pos_in_canvas * (1.0 - ratio);
                    // Scrolling has been changed, we have to update origin and pointer_pos for later use
                    origin = response_rect.min + self.scrolling.to_vec2();
                    /*
                    if let Some(pointer_pos) = response.hover_pos() {
                        pointer_pos_in_canvas = pointer_pos - origin;
                    }
                    */
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
            let grid_step = GRID_STEP_BASE * self.zoom_level;
            let grid_stroke = egui::Stroke::new(1.0, COLOR_GRID_LINE);

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
        // NOT IDEAL
        /*
        if let Some(pointer_pos) = response.interact_pointer_pos() {
            let pointer_pos_in_canvas = pointer_pos - origin;

            // Show tooltip with Node ID on hover
            let mut tooltip = String::new();
            let mut is_first_id = true;
            for (key, value) in &self.canvas_nodes {
                if value.is_point_inside_incl(pointer_pos_in_canvas) {
                    if !is_first_id {
                        tooltip.push_str(", ");
                    }
                    tooltip.push_str(key);
                    is_first_id = false;
                }
            }
            if !tooltip.is_empty() {
                response.show_tooltip_text(tooltip);
            }
        }
        */

        // .: Secondary canvas toolbar :.
        // .:==========================:.
        if self.do_show_secondary_canvas_toolbar {
            ui.horizontal(|ui| {
                ui.add_space(widget::TINYSKIP);

                for tup in NODE_TYPES {
                    let _ = ui.button(tup.0);
                    // TODO try tooltip
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(
                        egui::Slider::new(&mut self.zoom_level, 0.5..=2.0)
                            .suffix("x zoom")
                            .max_decimals(1),
                    );
                });

                ui.add_space(widget::TINYSKIP);
            });
        }

        // --- ---
        response
    }

    pub fn set_canvas_font_size_and_zoom(&mut self, new_font_size: u32) {
        self.canvas_font_size = new_font_size.clamp(FONT_SIZE_CANVAS_MIN, FONT_SIZE_CANVAS_MAX);
        self.zoom_level = self.canvas_font_size as f32 / FONT_SIZE_CANVAS_BASE as f32;
    }

    pub fn reset_canvas_scrolling_and_zoom(&mut self) {
        self.scrolling = SCROLLING_DEFAULT;
        self.set_canvas_font_size_and_zoom(FONT_SIZE_CANVAS_BASE);
    }
}
