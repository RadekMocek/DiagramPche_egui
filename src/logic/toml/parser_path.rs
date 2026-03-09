use crate::logic::toml::parser::Parser;
use crate::model::path::Path;
use crate::model::pathpoint::Pathpoint;
use crate::model::pathpoint_type::PathpointType;
use crate::model::pivot::Pivot;
use crate::model::point::Point;
use std::str::FromStr;
use toml_edit::{Array, Table, Value};

impl Parser {
    pub(super) fn parse_path(&mut self, path_table: &Table, curr_path: &mut Path) {
        for (key, item) in path_table {
            match key {
                // == start ==
                "start" => {
                    if let Some(item_value) = item.as_value() {
                        self.parse_path_start_or_end(item_value, &mut curr_path.start)
                    }
                }
                // == end ==
                "end" => {
                    if let Some(item_value) = item.as_value() {
                        let mut curr_point = Point::default();
                        self.parse_path_start_or_end(item_value, &mut curr_point);
                        curr_path.ends.push(curr_point);
                    }
                }
                // == ends ==
                "ends" => {
                    if let Some(item_arr) = item.as_array() {
                        for inner_value in item_arr {
                            let mut curr_point = Point::default();
                            self.parse_path_start_or_end(inner_value, &mut curr_point);
                            curr_path.ends.push(curr_point);
                        }
                    } else {
                        self.report_error(
                            "An array of end Pathpoints must follow after 'ends='",
                            &item.span(),
                        );
                    }
                }
                // == points ==
                "points" => {
                    if let Some(pathpoint_arr_arr) = item.as_array() {
                        for pathpoint_value in pathpoint_arr_arr {
                            if let Some(pathpoint_arr) = pathpoint_value.as_array()
                                && pathpoint_arr.len() == 6
                                && pathpoint_arr.get(0).expect("len 6").is_str()
                                && pathpoint_arr.get(1).expect("len 6").is_str()
                                && (pathpoint_arr.get(2).expect("len 6").is_str()
                                    || pathpoint_arr.get(2).expect("len 6").is_integer())
                                && pathpoint_arr.get(3).expect("len 6").is_str()
                                && pathpoint_arr.get(4).expect("len 6").is_str()
                                && (pathpoint_arr.get(5).expect("len 6").is_str()
                                    || pathpoint_arr.get(5).expect("len 6").is_integer())
                            {
                                let mut curr_pathpoint = Pathpoint::default();
                                self.parse_pathpoint_x_or_y(
                                    pathpoint_arr,
                                    true,
                                    &mut curr_pathpoint,
                                );
                                self.parse_pathpoint_x_or_y(
                                    pathpoint_arr,
                                    false,
                                    &mut curr_pathpoint,
                                );
                                curr_path.pathpoints.push(curr_pathpoint);
                            } else {
                                self.report_error("Pathpoint must be an array of 6 items: [string, string, integer, string, string, integer] \
                                (and 'points' expects an ARRAY of pathpoints)", &pathpoint_value.span());
                            }
                        }
                    }
                }
                // == shift ==
                "shift" => {
                    if let Some(item_arr) = item.as_array()
                        && item_arr.len() == 2
                    {
                        curr_path.shift_start =
                            self.get_int_from_int_or_var(item_arr.get(0).expect("len 2"));
                        curr_path.shift_end =
                            self.get_int_from_int_or_var(item_arr.get(1).expect("len 2"));
                    } else if let Some(item_value) = item.as_value() {
                        let shift_both = self.get_int_from_int_or_var(item_value);
                        curr_path.shift_start = shift_both;
                        curr_path.shift_end = shift_both;
                    }
                }
                // == color ==
                "color" => self.set_color_from_array_or_string(item, &mut curr_path.color),
                // == tips ==
                "tips" => {
                    if let Some(item_str) = item.as_str() {
                        match item_str {
                            "--" => {
                                curr_path.do_start_arrow = false;
                                curr_path.do_end_arrow = false;
                            }
                            "<-" => {
                                curr_path.do_start_arrow = true;
                                curr_path.do_end_arrow = false;
                            }
                            "->" => {
                                curr_path.do_start_arrow = false;
                                curr_path.do_end_arrow = true;
                            }
                            "<>" => {
                                curr_path.do_start_arrow = true;
                                curr_path.do_end_arrow = true;
                            }
                            _ => self.report_error(
                                "Possible values after 'tips=' are '--', '<-', '->', '<>'",
                                &item.span(),
                            ),
                        }
                    } else {
                        self.report_error("A string must follow after 'tips='", &item.span());
                    }
                }
                // == z ==
                "z" => curr_path.z = self.get_z_from_int(item, false),
                // == Unknown key ==
                _ => self.report_error(&format!("Unknown key '{key}'"), &item.span()),
            }
        }
    }

    fn parse_path_start_or_end(&mut self, value: &Value, to_set: &mut Point) {
        self.set_position_point_from_array(value, to_set);
        // Check if the parent id exist, `SetPositionPointFromArray` does not do that because all nodes might not parsed yet when we call it (now they are)
        let parent_id = &to_set.parent_id;
        if !parent_id.is_empty() && !self.result_nodes.contains_key(parent_id) {
            self.report_error(
                &format!("Path's start/end is referencing non existant id: '{parent_id}'"),
                &to_set.parent_id_span,
            );
        }
    }

    fn parse_pathpoint_x_or_y(
        &mut self,
        pathpoint_arr: &Array,
        is_x: bool,
        curr_pathpoint: &mut Pathpoint,
    ) {
        // (This method expects correct data types and is taylor-made for parsing the specific 6 item array)
        // (This method is called twice on the 6 item array, and according to `is_x` parses either first or second half of it)
        let arr_offset = if is_x { 0 } else { 3 };
        let id_str = pathpoint_arr.get(0 + arr_offset).unwrap().as_str().unwrap();
        let mut type_to_set = PathpointType::Reference;
        let mut pivot_to_set = Pivot::Center;
        if id_str.is_empty() {
            if let Ok(type_result) = PathpointType::from_str(
                pathpoint_arr.get(1 + arr_offset).unwrap().as_str().unwrap(),
            ) {
                type_to_set = type_result;
            } else {
                self.report_error("Allowed PathpointType values are: 'start', 'end', 'prev', '' (empty string for absolute coordinates)",
                                  &pathpoint_arr.get(1 + arr_offset).unwrap().span());
            }
        } else {
            if !self.result_nodes.contains_key(id_str) {
                self.report_error(
                    &format!("Pathpoint's x is referencing non existant id: '{id_str}'"),
                    &pathpoint_arr.get(0 + arr_offset).unwrap().span(),
                );
            }
            pivot_to_set = self.get_pivot_from_value(pathpoint_arr.get(1 + arr_offset).unwrap());
        }
        let coor_to_set = self.get_int_from_int_or_var(pathpoint_arr.get(2 + arr_offset).unwrap());
        if is_x {
            curr_pathpoint.x_type = type_to_set;
            curr_pathpoint.x_parent_id = String::from(id_str);
            curr_pathpoint.x_parent_pivot = pivot_to_set;
            curr_pathpoint.x = coor_to_set;
        } else {
            curr_pathpoint.y_type = type_to_set;
            curr_pathpoint.y_parent_id = String::from(id_str);
            curr_pathpoint.y_parent_pivot = pivot_to_set;
            curr_pathpoint.y = coor_to_set;
        }
    }
}
