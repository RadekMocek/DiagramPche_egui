use crate::logic::parser::Parser;
use crate::model::node::Node;
use crate::model::position::Position;
use toml::{Table, Value};

impl Parser {
    pub(super) fn parse_node(&mut self, node_table: &Table, curr_node: &mut Node) {
        const ERR_MSG_EXPECTED_ARRAY: &str = "An array ([X, Y] or [Parent, Pivot, X, Y]) expected";

        for (key, value) in node_table {
            match key as &str {
                "xy" => {
                    if let Some(value_arr) = value.as_array() {
                        match value_arr.len() {
                            2 => {
                                let x = self.get_int_from_int_or_var(
                                    value_arr.get(0).expect("value_arr should have len of 2"),
                                );
                                let y = self.get_int_from_int_or_var(
                                    value_arr.get(1).expect("value_arr should have len of 2"),
                                );
                                curr_node.position = Position::Absolute(x, y);
                            }
                            4 => {
                                //
                            }
                            _ => self.report_error(ERR_MSG_EXPECTED_ARRAY),
                        }
                    } else {
                        self.report_error(ERR_MSG_EXPECTED_ARRAY);
                    }
                }
                _ => self.report_error(&format!("Unknown key '{key}'")),
            }
        }
    }

    fn get_int_from_int_or_var(&mut self, value: &Value) -> i64 {
        const DEFAULT_VALUE: i64 = 0;

        if let Some(value_int) = value.as_integer() {
            value_int
        } else if let Some(value_str) = value.as_str() {
            let Some(result) = self.variables.get(value_str) else {
                self.report_error(&format!(
                    "Variable '{value_str}' does not exist (expecting [X, Y])"
                ));
                return DEFAULT_VALUE;
            };
            *result
        } else {
            self.report_error(
                "Value must be specified as an integer or a string with a variable name",
            );
            DEFAULT_VALUE
        }
    }
}
