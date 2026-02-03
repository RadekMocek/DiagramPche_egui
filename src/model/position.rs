pub enum Position {
    Absolute(i64, i64),
    TopLeft(String, i64, i64),
    Top(String, i64, i64),
    TopRight(String, i64, i64),
    Right(String, i64, i64),
    BottomRight(String, i64, i64),
    Bottom(String, i64, i64),
    BottomLeft(String, i64, i64),
    Left(String, i64, i64),
    Center(String, i64, i64),
}
