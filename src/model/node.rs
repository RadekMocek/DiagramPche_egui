use crate::helper::draw_layer::*;
use crate::model::color::Color;
use crate::model::node_type::NodeType;
use crate::model::pivot::Pivot;
use crate::model::point::Point;
use std::ops::Range;

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
    pub label_shift_x: i64,
    pub label_shift_y: i64,

    // = Z =
    pub z: i64,

    // = Type =
    pub node_type: NodeType,

    // = Canvas interaction =
    pub node_span: Option<Range<usize>>,
    pub color_span: Option<Range<usize>>,
    pub type_span: Option<Range<usize>>,
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
            label_shift_x: 0,
            label_shift_y: 0,

            // = Z =
            z: DL_USER_CHANNEL_DEFAULT_NODE,

            // = Type =
            node_type: NodeType::Rectangle,

            // = Canvas interaction =
            node_span: None,
            color_span: None,
            type_span: None,
        }
    }
}
