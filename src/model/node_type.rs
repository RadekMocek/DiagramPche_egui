use crate::helper::icon::*;
use const_format::concatcp;
use std::str::FromStr;

pub enum NodeType {
    Rectangle,
    Ellipse,
    Diamond,
    Text,
}

impl FromStr for NodeType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" | "rectangle" => Ok(Self::Rectangle),
            "ellipse" => Ok(Self::Ellipse),
            "diamond" => Ok(Self::Diamond),
            "text" => Ok(Self::Text),
            _ => Err(()),
        }
    }
}

impl NodeType {
    pub fn as_usize(&self) -> usize {
        match self {
            NodeType::Rectangle => 0,
            NodeType::Ellipse => 1,
            NodeType::Diamond => 2,
            NodeType::Text => 3,
        }
    }

    pub fn as_quoted_string(&self) -> String {
        get_node_type_quoted_string_from_usize(self.as_usize())
    }
}

pub fn get_node_type_quoted_string_from_usize(num: usize) -> String {
    match num {
        0 => String::from("\"rectangle\""),
        1 => String::from("\"ellipse\""),
        2 => String::from("\"diamond\""),
        3 => String::from("\"text\""),
        _ => String::from("\"\""),
    }
}

// --- --- --- --- --- --- --- --- --- --- --- --- ---

// For toolbar :: combobox to change node type
pub const NODE_TYPE_CHOICES: [&str; 4] = [
    concatcp!(ICON_RECTANGLE_OUTLINE, " Rectangle"),
    concatcp!(ICON_ELLIPSE_OUTLINE, " Ellipse  "),
    concatcp!(ICON_RHOMBUS_OUTLINE, " Diamond  "),
    concatcp!(ICON_FORMAT_TEXT_VARIANT, " Text     "),
];

// For secondary canvas toolbar :: drag n drop buttons
pub const NODE_TYPES: [(&str, NodeType, &str); 4] = [
    (
        concatcp!(ICON_RECTANGLE_OUTLINE),
        NodeType::Rectangle,
        "rectangle",
    ),
    (
        concatcp!(ICON_ELLIPSE_OUTLINE),
        NodeType::Ellipse,
        "elllipse",
    ),
    (
        concatcp!(ICON_RHOMBUS_OUTLINE),
        NodeType::Diamond,
        "diamond",
    ),
    (concatcp!(ICON_FORMAT_TEXT_VARIANT), NodeType::Text, "text"),
];
