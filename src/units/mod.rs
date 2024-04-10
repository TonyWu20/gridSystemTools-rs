pub struct Point(f32);

impl Point {
    pub fn to_mm(&self) -> Millimeter {
        Millimeter::from_pt(self)
    }
    pub fn to_inch(&self) -> Inch {
        Inch::from_pt(self)
    }
    pub fn to_pixel(&self) -> Pixel {
        Pixel::from_pt(self)
    }
}
pub struct Millimeter(f32);
pub struct Inch(f32);
pub struct Pixel(f32);

pub trait LengthUnit {
    const BASE_TO_POINT: f32;
    fn new(value: f32) -> Self;
    fn from_pt(points: &Point) -> Self;
    fn to_pt(&self) -> Point;
}

impl LengthUnit for Point {
    const BASE_TO_POINT: f32 = 1.0;

    fn new(value: f32) -> Self {
        Self(value)
    }
    fn from_pt(points: &Point) -> Self {
        Self(points.0)
    }

    fn to_pt(&self) -> Point {
        Point::new(self.0 * Self::BASE_TO_POINT)
    }
}

impl LengthUnit for Millimeter {
    const BASE_TO_POINT: f32 = 2.8345;

    fn new(value: f32) -> Self {
        Self(value)
    }
    fn from_pt(points: &Point) -> Self {
        let value = points.0 / Self::BASE_TO_POINT;
        Self((value * 10000.0).round() / 10000.0)
    }
    fn to_pt(&self) -> Point {
        Point::new(self.0 * Self::BASE_TO_POINT)
    }
}

impl LengthUnit for Inch {
    const BASE_TO_POINT: f32 = 72.0;

    fn new(value: f32) -> Self {
        Self(value)
    }
    fn from_pt(points: &Point) -> Self {
        let value = points.0 / Self::BASE_TO_POINT;
        Self((value * 10000.0).round() / 10000.0)
    }
    fn to_pt(&self) -> Point {
        Point::new(self.0 * Self::BASE_TO_POINT)
    }
}

impl LengthUnit for Pixel {
    const BASE_TO_POINT: f32 = 0.75;

    fn new(value: f32) -> Self {
        Self(value)
    }
    fn from_pt(points: &Point) -> Self {
        let value = points.0 / Self::BASE_TO_POINT;
        Self((value * 10000.0).round() / 10000.0)
    }
    fn to_pt(&self) -> Point {
        Point::new(self.0 * Self::BASE_TO_POINT)
    }
}
