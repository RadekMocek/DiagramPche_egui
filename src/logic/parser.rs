use crate::model::node::Node;
use std::collections::{HashMap, HashSet};
use std::ops::Range;
use toml_edit::Document;

pub struct Parser {
    pub is_error: bool,
    pub error_message: String,
    pub error_span: Option<Range<usize>>,
    pub result_nodes: HashMap<String, Node>,

    pub(super) variables: HashMap<String, i64>, //TODO pub(super) may not be needed in the future
}

impl Default for Parser {
    fn default() -> Self {
        Parser {
            is_error: false,
            error_message: String::new(),
            error_span: None,
            result_nodes: HashMap::new(),

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
        let mut refs: HashMap<String, String> = HashMap::new();
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
                    let ref_node_batch_number = self.result_nodes[ref_id].draw_batch_number;
                    let dep_node = self
                        .result_nodes
                        .get_mut(dep_id)
                        .expect("It was added to refs so must be in result_nodes too");
                    dep_node.draw_batch_number = ref_node_batch_number + 1;
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
        //TODO
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
