use std::fmt::Debug;

pub fn print_2d_slice<T: Debug>(vec: &[T]) {
    for line in vec {
        println!("{:?}", line);
    }
}

#[derive(Debug, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Default for Position {
    fn default() -> Self {
        Position { x: 0, y: 0 }
    }
}