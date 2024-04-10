use crate::units::{LengthUnit, Point};

pub struct Grid {
    columns: u32,
    rows: u32,
    column_gutter: f32,
    row_gutter: f32,
}

pub struct Block<T: LengthUnit> {
    height: T,
    width: T,
    lines: usize,
    leading: Point,
}
