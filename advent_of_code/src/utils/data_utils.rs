#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct IndexPoint {
    pub x: usize,
    pub y: usize,
}
