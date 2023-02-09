#[repr(u8)]
#[derive(Clone, Copy)]
#[allow(unused)]
pub enum Color {
    Black = 30,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1B[{}m", *self as u8)
    }
}
