use crate::logic::parser::Parser;
use crate::model::node::Node;
use crate::model::pivot::Pivot;
use std::str::FromStr;
use toml_edit::{Table, Value};

impl Parser {
    pub(super) fn parse_node(&mut self, node_table: &Table, curr_node: &mut Node) {
        for (key, item) in node_table {
            match key {
                // == value ==
                "value" => {
                    if let Some(item_str) = item.as_str() {
                        curr_node.value = String::from(item_str);
                        curr_node.is_value_explicitly_set = true;
                    } else {
                        self.report_error("A string must follow after 'value='", &item.span())
                    }
                }
                // == xy ==
                "xy" => {
                    const ERR_MSG_EXPECTED_ARRAY: &str =
                        "An array ([X, Y] or [Parent, Pivot, X, Y]) expected";

                    if let Some(item_arr) = item.as_array() {
                        match item_arr.len() {
                            2 => {
                                curr_node.position.x = self.get_int_from_int_or_var(
                                    item_arr.get(0).expect("item_arr should have len of 2"),
                                );
                                curr_node.position.y = self.get_int_from_int_or_var(
                                    item_arr.get(1).expect("item_arr should have len of 2"),
                                );
                            }
                            4 => {
                                // Parent
                                if let Some(item_str) = item_arr
                                    .get(0)
                                    .expect("item_arr should have len of 4")
                                    .as_str()
                                {
                                    curr_node.position.parent_id = String::from(item_str);
                                    if item_str.is_empty() {
                                        self.report_error(
                                            "Parent reference can't be empty",
                                            &item.span(),
                                        );
                                    }
                                }
                                // Pivot
                                curr_node.position.parent_pivot = self.get_pivot_from_value(
                                    item_arr.get(1).expect("item_arr should have len of 4"),
                                );
                                // X
                                curr_node.position.x = self.get_int_from_int_or_var(
                                    item_arr.get(2).expect("item_arr should have len of 4"),
                                );
                                // Y
                                curr_node.position.y = self.get_int_from_int_or_var(
                                    item_arr.get(3).expect("item_arr should have len of 4"),
                                );
                            }
                            _ => self.report_error(ERR_MSG_EXPECTED_ARRAY, &item.span()),
                        }
                    } else {
                        self.report_error(ERR_MSG_EXPECTED_ARRAY, &item.span());
                    }
                }
                // == pivot ==
                "pivot" => {
                    if let Some(item_value) = item.as_value() {
                        curr_node.pivot = self.get_pivot_from_value(item_value);
                    }
                }
                // == color ==
                "color" => {
                    const ERR_MSG_EXPECTED_ARRAY: &str =
                        "An array of four uchars (0–255) must follow after 'color='";

                    if let Some(item_arr) = item.as_array()
                        && item_arr.len() == 4
                    {
                        let c: Vec<_> = item_arr
                            .iter()
                            .filter_map(|item| item.as_integer())
                            .map(|item| item as u8)
                            .collect();
                        if c.len() == 4 {
                            curr_node.color = (c[0], c[1], c[2], c[3]);
                        } else {
                            self.report_error(ERR_MSG_EXPECTED_ARRAY, &item.span());
                        }
                    } else {
                        self.report_error(ERR_MSG_EXPECTED_ARRAY, &item.span());
                    }
                }
                // == size ==
                "size" => {
                    if let Some(item_arr) = item.as_array()
                        && item_arr.len() == 2
                    {
                        curr_node.width = self.get_int_from_int_or_var(
                            item_arr.get(0).expect("item_arr should have len of 2"),
                        );
                        curr_node.height = self.get_int_from_int_or_var(
                            item_arr.get(1).expect("item_arr should have len of 2"),
                        );
                    } else {
                        self.report_error("An array of two integers/strings of variable names must follow after 'size='", &item.span())
                    }
                }
                // == label_pos ==
                "label_pos" => {
                    if let Some(item_value) = item.as_value() {
                        curr_node.label_position = self.get_pivot_from_value(item_value);
                    }
                }
                // == z ==
                "z" => {
                    //TODO
                }
                // == Unknown key ==
                _ => self.report_error(&format!("Unknown key '{key}'"), &item.span()),
            }
        }
    }

    fn get_int_from_int_or_var(&mut self, value: &Value) -> i64 {
        const DEFAULT_VALUE: i64 = 0;

        if let Some(value_int) = value.as_integer() {
            value_int
        } else if let Some(value_str) = value.as_str() {
            let Some(result) = self.variables.get(value_str) else {
                self.report_error(
                    &format!("Variable '{value_str}' does not exist (expecting [X, Y])"),
                    &value.span(),
                );
                return DEFAULT_VALUE;
            };
            *result
        } else {
            self.report_error(
                "Value must be specified as an integer or a string with a variable name",
                &value.span(),
            );
            DEFAULT_VALUE
        }
    }

    fn get_pivot_from_value(&mut self, value: &Value) -> Pivot {
        const DEFAULT_VALUE: Pivot = Pivot::TopLeft;
        const ERR_MSG_NOT_STRING: &str = "Pivot value must be specified by a string";
        const ERR_MSG_WRONG_STRING: &str = "Allowed pivot values are: 'top-left', 'top', 'top-right', 'right', 'bottom-right', 'bottom', 'bottom-left', 'left', 'center'";

        if let Some(value_str) = value.as_str() {
            if let Ok(result) = Pivot::from_str(value_str) {
                return result;
            } else {
                self.report_error(ERR_MSG_WRONG_STRING, &value.span());
            }
        } else {
            self.report_error(ERR_MSG_NOT_STRING, &value.span());
        }

        DEFAULT_VALUE
    }
}
