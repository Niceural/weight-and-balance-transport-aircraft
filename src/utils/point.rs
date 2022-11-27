use std::ops;

#[derive(Copy, Clone)]
pub struct Point<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn y(&self) -> &T {
        &self.y
    }

    pub fn z(&self) -> &T {
        &self.z
    }

/*
    pub fn move_option(p: Point<Option<T>>) -> Option<Point<T>> {
        let x: T = match p.x() {
            Some(val) => *val,
            None => return None,
        };
        let y = match p.y() {
            Some(val) => val,
            None => return None,
        };
        let z = match p.z() {
            Some(val) => val,
            None => return None,
        };

        Some(Point::new(x, y, z))
    }
*/
}

impl<T: ops::Add<Output = T>> ops::Add<Point<T>> for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Point<T>) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: ops::Mul<Output = T> + Copy> ops::Mul<T> for Point<T> {
    type Output = Point<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: ops::Div<Output = T> + Copy> ops::Div<T> for Point<T> {
    type Output = Point<T>;

    fn div(self, rhs: T) -> Self::Output {
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

