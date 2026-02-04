use crate::model::pivot::Pivot;
use crate::model::point::Point;

pub struct Node {
    // = ID =
    id: String,

    // = Value =
    pub value: String,
    pub is_value_explicitly_set: bool,

    // = XY =
    pub position: Point,

    // = Pivot =
    pub pivot: Pivot,

    // = Color =
    pub color: (u8, u8, u8, u8),

    // = Size =
    pub width: i64,
    pub height: i64,

    // = Label pos =
    pub label_position: Pivot,

    // = Z =
    pub z: i64,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            // = ID =
            id: String::from(""),

            // = Value =
            value: String::from(""),
            is_value_explicitly_set: false,

            // = XY =
            position: Point::default(),

            // = Pivot =
            pivot: Pivot::TopLeft,

            // = Color =
            color: (255, 255, 255, 255),

            // = Size =
            width: 0,
            height: 0,

            // = Label pos =
            label_position: Pivot::Center,

            // = Z =
            z: 0,
        }
    }
}
