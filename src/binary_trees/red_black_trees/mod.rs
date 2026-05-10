pub mod red_black_tree;
pub mod cursors;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Color {
    Red,
    Black,
}

impl Color {
    fn opposite(&self) -> Color {
        match self {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
        }
    }
}
