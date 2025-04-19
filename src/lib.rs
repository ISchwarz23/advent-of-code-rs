pub mod template;

pub mod rect {
    use crate::vector::Vector2d;

    #[derive(Clone, Debug, Hash, Eq, PartialEq)]
    pub struct Rectangle {
        pub x_from: i64,
        pub x_to: i64,
        pub y_from: i64,
        pub y_to: i64,
    }

    impl Rectangle {
        pub fn contains(&self, vector2d: &Vector2d) -> bool {
            self.x_from <= vector2d.x
                && vector2d.x <= self.x_to
                && self.y_from <= vector2d.y
                && vector2d.y <= self.y_to
        }
    }
}

pub mod vector {
    use std::ops::{Add, Mul, Sub};

    #[derive(Clone, Debug, Hash, Eq, PartialEq)]
    pub struct Vector2d {
        pub x: i64,
        pub y: i64,
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

    impl Add for &Vector2d {
        type Output = Vector2d;

        fn add(self, other: Self) -> Self::Output {
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

    impl Sub for &Vector2d {
        type Output = Vector2d;

        fn sub(self, other: Self) -> Self::Output {
            Vector2d {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    impl Mul<i64> for Vector2d {
        type Output = Vector2d;

        fn mul(self, other: i64) -> Self::Output {
            Vector2d {
                x: self.x * other,
                y: self.y * other,
            }
        }
    }

    impl Mul<i64> for &Vector2d {
        type Output = Vector2d;

        fn mul(self, other: i64) -> Self::Output {
            Vector2d {
                x: self.x * other,
                y: self.y * other,
            }
        }
    }

    pub const DIR_RIGHT: Vector2d = Vector2d { x: 1, y: 0 };
    pub const DIR_DOWN: Vector2d = Vector2d { x: 0, y: 1 };
    pub const DIR_LEFT: Vector2d = Vector2d { x: -1, y: 0 };
    pub const DIR_UP: Vector2d = Vector2d { x: 0, y: -1 };
    pub const DIRS_MAIN: [Vector2d; 4] = [DIR_RIGHT, DIR_UP, DIR_LEFT, DIR_DOWN];

    pub const DIR_RIGHT_DOWN: Vector2d = Vector2d { x: 1, y: 1 };
    pub const DIR_RIGHT_UP: Vector2d = Vector2d { x: 1, y: -1 };
    pub const DIR_LEFT_UP: Vector2d = Vector2d { x: -1, y: -1 };
    pub const DIR_LEFT_DOWN: Vector2d = Vector2d { x: -1, y: 1 };
    pub const DIRS_DIAGONALS: [Vector2d; 4] =
        [DIR_RIGHT_DOWN, DIR_RIGHT_UP, DIR_LEFT_UP, DIR_LEFT_DOWN];

    pub const DIRS_ALL: [Vector2d; 8] = [
        DIR_RIGHT,
        DIR_RIGHT_UP,
        DIR_UP,
        DIR_LEFT_UP,
        DIR_LEFT,
        DIR_LEFT_DOWN,
        DIR_DOWN,
        DIR_RIGHT_DOWN,
    ];
}
