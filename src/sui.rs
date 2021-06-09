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
pub enum Flip {
    Horizontally,
    Vertically
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