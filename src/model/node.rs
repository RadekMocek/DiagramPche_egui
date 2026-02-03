use crate::model::position::Position;

pub struct Node {
    id: String,
    value: String,
    is_value_explicitly_set: bool,
    pub position: Position,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            id: String::from(""),
            value: String::from(""),
            is_value_explicitly_set: false,
            position: Position::Absolute(0, 0),
        }
    }
}
