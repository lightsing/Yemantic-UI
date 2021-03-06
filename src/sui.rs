#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Colors {
    Red,
    Orange,
    Yellow,
    Olive,
    Green,
    Teal,
    Blue,
    Violet,
    Purple,
    Pink,
    Brown,
    Grey,
    Black,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Sizes {
    Mini,
    Tiny,
    Small,
    Medium,
    Large,
    Big,
    Huge,
    Massive,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Flip {
    Horizontally,
    Vertically
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Float {
    Left,
    Right
}

impl From<Colors> for &'static str {
    fn from(c: Colors) -> Self {
        use Colors::*;
        match c {
            Red => "red",
            Orange => "orange",
            Yellow => "yellow",
            Olive => "olive",
            Green => "green",
            Teal => "teal",
            Blue => "blue",
            Violet => "violet",
            Purple => "purple",
            Pink => "pink",
            Brown => "brown",
            Grey => "grey",
            Black => "black",
        }
    }
}

impl AsRef<str> for Colors {
    fn as_ref(&self) -> &str {
        (*self).into()
    }
}

impl From<Sizes> for &'static str {
    fn from(i: Sizes) -> Self {
        use Sizes::*;

        match i {
            Mini => "mini",
            Tiny => "tiny",
            Small => "small",
            Medium => "medium",
            Large => "large",
            Big => "big",
            Huge => "huge",
            Massive => "massive",
        }
    }
}

impl AsRef<str> for Sizes {
    fn as_ref(&self) -> &str {
        (*self).into()
    }
}

impl From<Flip> for &'static str {
    fn from(f: Flip) -> Self {
        use Flip::*;

        match f {
            Horizontally => "horizontally",
            Vertically => "vertically",
        }
    }
}

impl AsRef<str> for Flip {
    fn as_ref(&self) -> &str {
        (*self).into()
    }
}

impl From<Float> for &'static str {
    fn from(f: Float) -> Self {
        use Float::*;

        match f {
            Left => "left",
            Right => "right",
        }
    }
}

impl AsRef<str> for Float {
    fn as_ref(&self) -> &str {
        (*self).into()
    }
}