use crate::model::pathpoint_type::PathpointType;
use crate::model::pivot::Pivot;

pub struct Pathpoint {
    pub x_type: PathpointType,
    pub x_parent_id: String,
    pub x_parent_pivot: Pivot,
    pub x: i64,

    pub y_type: PathpointType,
    pub y_parent_id: String,
    pub y_parent_pivot: Pivot,
    pub y: i64,
}

impl Default for Pathpoint {
    fn default() -> Self {
        Pathpoint {
            x_type: PathpointType::Reference,
            x_parent_id: String::from(""),
            x_parent_pivot: Pivot::Center,
            x: 0,

            y_type: PathpointType::Reference,
            y_parent_id: String::from(""),
            y_parent_pivot: Pivot::Center,
            y: 0,
        }
    }
}
