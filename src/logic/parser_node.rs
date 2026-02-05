use crate::logic::parser::Parser;
use crate::model::node::Node;
use toml_edit::Table;

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
                    if let Some(item_value) = item.as_value() {
                        self.set_position_point_from_array(item_value, &mut curr_node.position);
                    }
                }
                // == pivot ==
                "pivot" => {
                    if let Some(item_value) = item.as_value() {
                        curr_node.pivot = self.get_pivot_from_value(item_value);
                    }
                }
                // == color ==
                "color" => self.set_color_from_array(item, &mut curr_node.color),
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
}
