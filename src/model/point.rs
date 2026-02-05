use crate::model::pivot::Pivot;
use std::ops::Range;

pub struct Point {
    pub parent_id: String,
    pub parent_pivot: Pivot,
    pub x: i64,
    pub y: i64,
    pub parent_id_span: Option<Range<usize>>,
}

impl Default for Point {
    fn default() -> Self {
        Self {
            parent_id: String::from(""),
            parent_pivot: Pivot::Center,
            x: 0,
            y: 0,
            parent_id_span: None,
        }
    }
}
