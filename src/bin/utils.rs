use std::ops::{Add, Sub};

#[derive(Clone, Debug)]
pub struct Vector2d {
    pub x: i32,
    pub y: i32,
}

impl Add for Vector2d {
    type Output = Vector2d;

    fn add(self, other: Self) -> Vector2d {
        Vector2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector2d {
    type Output = Vector2d;

    fn sub(self, other: Self) -> Self::Output {
        Vector2d {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for &Vector2d {
    type Output = Vector2d;

    fn add(self, other: Self) -> Self::Output {
        Vector2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for &Vector2d {
    type Output = Vector2d;

    fn sub(self, other: Self) -> Self::Output {
        Vector2d {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
