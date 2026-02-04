use crate::model::pivot::Pivot;

pub struct Point {
    pub parent_id: String,
    pub parent_pivot: Pivot,
    pub x: i64,
    pub y: i64,
    //TODO parent_id_source_region for better error highlighting (?)
}

impl Default for Point {
    fn default() -> Self {
        Point {
            parent_id: String::from(""),
            parent_pivot: Pivot::Center,
            x: 0,
            y: 0,
        }
    }
}
