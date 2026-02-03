use crate::model::node::Node;
use std::collections::HashMap;
use toml::Table;

pub struct Parser {
    pub is_error: bool,
    pub error_message: String,
    pub nodes: Vec<Node>,

    pub(super) variables: HashMap<String, i64>, //TODO pub(super) may not be needed in the future
}

impl Default for Parser {
    fn default() -> Self {
        Parser {
            is_error: false,
            error_message: String::new(),
            nodes: Vec::new(),

            variables: HashMap::new(),
        }
    }
}

impl Parser {
    pub fn parse(&mut self, source: &str) {
        self.is_error = false;

        // Try to parse the TOML input
        let toml_parsed_result = source.parse::<Table>();
        let Ok(toml_parsed) = toml_parsed_result else {
            self.report_error("Error parsing");
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
                        self.variables.insert(key.clone(), value_int);
                    } else {
                        self.report_error("Only integer variables are allowed");
                    }
                }
            }
        }

        // .: Nodes :.
        // .:=======:.
        self.nodes.clear();

        if let Some(node) = toml_parsed.get("node") {
            if let Some(node_table) = node.as_table() {
                for (node_key, node_value) in node_table {
                    if let Some(node_value_table) = node_value.as_table() {
                        // Key exists and is unique (TOML won't parse if duplicate), but it could be empty string (not ideal)
                        if node_key.is_empty() {
                            self.report_error("Node id cannot be empty");
                        }
                        // Currently processed Node
                        let mut curr_node = Node::default();
                        // Parse `node_value_table` data and set `curr_node` members; or set error message
                        self.parse_node(&node_value_table, &mut curr_node);
                        // Add node to the result collection
                        self.nodes.push(curr_node);
                    }
                }
            }
        }
    }

    pub(super) fn report_error(&mut self, error_message: &str) {
        if !self.is_error {
            self.error_message = String::from(error_message);
            self.is_error = true;
        }
    }
}
