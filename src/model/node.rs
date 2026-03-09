use crate::helper::draw_layer::*;
use crate::model::color::Color;
use crate::model::pivot::Pivot;
use crate::model::point::Point;

pub struct Node {
    // = ID =
    pub id: String,

    // = Value =
    pub value: String,
    pub is_value_explicitly_set: bool,

    // = XY =
    pub position: Point,

    // = Pivot =
    pub pivot: Pivot,

    // = Color =
    pub color: Color,
    pub color_border: Color,

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
            color: Color::new(255, 255, 255, 255),
            color_border: Color::new(0, 0, 0, 255),

            // = Size =
            width: 0,
            height: 0,

            // = Label pos =
            label_position: Pivot::Center,

            // = Z =
            z: DL_USER_CHANNEL_DEFAULT_NODE,
        }
    }
}
