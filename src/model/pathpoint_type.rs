use std::str::FromStr;

pub enum PathpointType {
    Reference,
    Absolute,
    Start,
    End,
    Previous,
}

impl FromStr for PathpointType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Absolute),
            "start" => Ok(Self::Start),
            "end" => Ok(Self::End),
            "prev" => Ok(Self::Previous),
            _ => Err(()),
        }
    }
}
