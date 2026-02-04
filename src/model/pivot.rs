use std::str::FromStr;

pub enum Pivot {
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    Center,
}

impl FromStr for Pivot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "top-left" => Ok(Self::TopLeft),
            "top" => Ok(Self::Top),
            "top-right" => Ok(Self::TopRight),
            "right" => Ok(Self::Right),
            "bottom-right" => Ok(Self::BottomRight),
            "bottom" => Ok(Self::Bottom),
            "bottom-left" => Ok(Self::BottomLeft),
            "left" => Ok(Self::Left),
            "center" => Ok(Self::Center),
            _ => Err(()),
        }
    }
}
