use crate::model::color::Color;
use crate::model::node::Node;
use crate::model::path::Path;
use crate::model::pivot::Pivot;
use crate::model::point::Point;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::ops::Range;
use std::str::FromStr;
use toml_edit::{Document, Item, Value};

pub struct Parser {
    pub is_error: bool,
    pub error_message: String,
    pub error_span: Option<Range<usize>>,
    pub result_nodes: HashMap<String, Node>,
    pub result_paths: Vec<Path>,

    variables: HashMap<String, i64>,
}

impl Default for Parser {
    fn default() -> Self {
        Parser {
            is_error: false,
            error_message: String::new(),
            error_span: None,
            result_nodes: HashMap::new(),
            result_paths: Vec::new(),

            variables: HashMap::new(),
        }
    }
}

impl Parser {
    pub fn parse(&mut self, source: &str) {
        self.is_error = false;

        // Try to parse the TOML input
        let toml_parsed_result = source.parse::<Document<String>>();
        let Ok(toml_parsed) = toml_parsed_result else {
            let err = toml_parsed_result
                .err()
                .expect("toml_parsed_result is not Ok");
            self.report_error(err.message(), &err.span());
            // By returning here, last valid TOML will be drawn (result collections weren't cleared yet)
            return;
        };

        // .: Variables :.
        // .:===========:.
        self.variables.clear();

        if let Some(vars) = toml_parsed.get("variables") {
            if let Some(vars_table) = vars.as_table() {
                for (key, value) in vars_table {
                    if let Some(value_int) = value.as_integer() {
                        self.variables.insert(String::from(key), value_int);
                    } else {
                        self.report_error("Only integer variables are allowed", &value.span());
                    }
                }
            }
        }

        // .: Nodes :.
        // .:=======:.
        self.result_nodes.clear();
        let mut refs: BTreeMap<String, String> = BTreeMap::new();
        let mut stable_nodes: HashSet<String> = HashSet::new();

        if let Some(node) = toml_parsed.get("node") {
            if let Some(node_table) = node.as_table() {
                for (node_key, node_value) in node_table {
                    if let Some(node_value_table) = node_value.as_table() {
                        // Key exists and is unique (TOML won't parse if duplicate), but it could be empty string (not ideal)
                        if node_key.is_empty() {
                            self.report_error("Node id cannot be empty", &node_value_table.span());
                        }

                        // Currently processed Node
                        let mut curr_node = Node::default();
                        curr_node.id = String::from(node_key);

                        // Parse `node_value_table` data and set `curr_node` members; or set error message
                        self.parse_node(&node_value_table, &mut curr_node);

                        // Check if the node is not referencing itself
                        if curr_node.id == curr_node.position.parent_id {
                            self.report_error(
                                &format!("Node with id '{}' is referencing itself", curr_node.id),
                                &curr_node.position.parent_id_span,
                            );
                        }

                        // If user doesn't set any text value explicitly, we use node's ID (can be rejected by setting `value=""`)
                        if !curr_node.is_value_explicitly_set {
                            curr_node.value = curr_node.id.clone();
                        }

                        // Empty parent means stable node; otherwise dependant node
                        if !curr_node.position.parent_id.is_empty() {
                            refs.insert(curr_node.id.clone(), curr_node.position.parent_id.clone());
                        } else {
                            stable_nodes.insert(curr_node.id.clone());
                        }

                        // Add node to the result collection
                        self.result_nodes.insert(curr_node.id.clone(), curr_node);
                    }
                }
            }
        }

        // Check the refs
        let mut did_anything_change = !refs.is_empty();
        while did_anything_change {
            did_anything_change = false;
            for (dep_id, ref_id) in &refs {
                // Check if the referred ID does exist
                if !self.is_error && !self.result_nodes.contains_key(ref_id) {
                    self.report_error(
                        &format!("Node '{dep_id}' is referencing non existant id: '{ref_id}'"),
                        &self.result_nodes[dep_id].position.parent_id_span.clone(),
                    )
                }
                if !stable_nodes.contains(dep_id) && stable_nodes.contains(ref_id) {
                    let ref_node_batch_number = self.result_nodes[ref_id].preparation_batch_number;
                    let dep_node = self
                        .result_nodes
                        .get_mut(dep_id)
                        .expect("It was added to refs so must be in result_nodes too");
                    dep_node.preparation_batch_number = ref_node_batch_number + 1;
                    stable_nodes.insert(dep_id.clone());
                    did_anything_change = true;
                }
            }
        }

        // Check circular reference
        if !self.is_error && stable_nodes.len() < self.result_nodes.len() {
            let mut error_message = String::from("Circular reference somewhere among:");
            for (key, _) in &self.result_nodes {
                if !stable_nodes.contains(key) {
                    error_message.push_str(&format!(" '{key}'"));
                }
            }
            self.report_error(&error_message, &None);
        }

        // .: Paths :.
        // .:=======:.
        self.result_paths.clear();

        if let Some(paths) = toml_parsed.get("path") {
            if let Some(paths_table_array) = paths.as_array_of_tables() {
                for path_table in paths_table_array {
                    let mut curr_path = Path::default();
                    self.parse_path(&path_table, &mut curr_path);
                    self.result_paths.push(curr_path);
                }
            }
        }
    }

    pub(super) fn get_pivot_from_value(&mut self, value: &Value) -> Pivot {
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

    pub(super) fn get_int_from_int_or_var(&mut self, value: &Value) -> i64 {
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

    pub(super) fn set_position_point_from_array(&mut self, value: &Value, to_set: &mut Point) {
        const ERR_MSG_EXPECTED_ARRAY: &str = "An array ([X, Y] or [Parent, Pivot, X, Y]) expected";

        if let Some(value_arr) = value.as_array() {
            match value_arr.len() {
                2 => {
                    to_set.x = self.get_int_from_int_or_var(
                        value_arr.get(0).expect("value_arr should have len of 2"),
                    );
                    to_set.y = self.get_int_from_int_or_var(
                        value_arr.get(1).expect("value_arr should have len of 2"),
                    );
                }
                4 => {
                    let parent_id_span = value_arr
                        .get(0)
                        .expect("value_arr should have len of 4")
                        .span();

                    // Parent
                    if let Some(value_str) = value_arr
                        .get(0)
                        .expect("value_arr should have len of 4")
                        .as_str()
                    {
                        to_set.parent_id = String::from(value_str);
                        if value_str.is_empty() {
                            self.report_error("Parent reference can't be empty", &parent_id_span);
                        }
                        // Better error reporting (self reference or non existing reference) for better diagram developer experience :)
                        to_set.parent_id_span = parent_id_span;
                    }
                    // Pivot
                    to_set.parent_pivot = self.get_pivot_from_value(
                        value_arr.get(1).expect("value_arr should have len of 4"),
                    );
                    // X
                    to_set.x = self.get_int_from_int_or_var(
                        value_arr.get(2).expect("value_arr should have len of 4"),
                    );
                    // Y
                    to_set.y = self.get_int_from_int_or_var(
                        value_arr.get(3).expect("value_arr should have len of 4"),
                    );
                }
                _ => self.report_error(ERR_MSG_EXPECTED_ARRAY, &value.span()),
            }
        } else {
            self.report_error(ERR_MSG_EXPECTED_ARRAY, &value.span());
        }
    }

    pub(super) fn set_color_from_array(&mut self, item: &Item, to_set: &mut Color) {
        const ERR_MSG_EXPECTED_ARRAY: &str =
            "An array of four uchars (0–255) or RGBA hex string must follow after 'color='";

        if let Some(item_arr) = item.as_array()
            && item_arr.len() == 4
        {
            let c: Vec<_> = item_arr
                .iter()
                .filter_map(|item| item.as_integer())
                .map(|item| item as u8)
                .collect();
            if c.len() == 4 {
                *to_set = Color::new(c[0], c[1], c[2], c[3]);
            } else {
                self.report_error(ERR_MSG_EXPECTED_ARRAY, &item.span());
            }
        } else if let Some(item_str) = item.as_str() {
            *to_set = Color::from_str(item_str);
        } else {
            self.report_error(ERR_MSG_EXPECTED_ARRAY, &item.span());
        }
    }

    pub(super) fn get_z_from_int(&mut self, item: &Item, is_node: bool) -> i64 {
        const MIN: i64 = 0;
        const MAX: i64 = crate::config::N_DRAW_LIST_CHANNELS - 1;
        let err_msg_range = format!("An integer between {MIN} and {MAX} must follow after 'z='");
        let default_result = if is_node {
            crate::config::DRAW_LIST_CHANNEL_DEFAULT_NODE
        } else {
            crate::config::DRAW_LIST_CHANNEL_DEFAULT_PATH
        };

        if let Some(result) = item.as_integer() {
            if result < MIN {
                self.report_error(&err_msg_range, &item.span());
                return MIN;
            }
            if result > MAX {
                self.report_error(&err_msg_range, &item.span());
                return MAX;
            }
            return result;
        }

        self.report_error(&err_msg_range, &item.span());
        default_result
    }

    pub(super) fn report_error(&mut self, error_message: &str, error_span: &Option<Range<usize>>) {
        if !self.is_error {
            self.error_message = String::from(error_message);
            //self.error_message = format!("{error_message} {error_span:?}");
            self.error_span = error_span.clone();
            self.is_error = true;
        }
    }
}
