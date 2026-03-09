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
