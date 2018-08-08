use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Neg;
use std::ops::AddAssign;

#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Point {
    pub x : f64,
    pub y : f64
}

impl Point {
    pub fn norm(self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn rotate(self, angle: f64) -> Point {
        Point {
            x: self.x * angle.cos() - self.y * angle.sin(),
            y: self.x * angle.sin() + self.y * angle.cos()
        }
    }
    
    pub fn cross(self, other: Point) -> f64 {
        self.x * other.y - self.y * other.x
    }

    pub fn from_angle(angle: f64) -> Point {
        Point{ x: angle.cos(), y: angle.sin() }
    }

    pub fn normalized(self) -> Point {
        self / self.norm()
    }

    pub fn new(x: f64, y: f64) -> Point {
        Point {
            x: x,
            y: y
        }
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl Mul<f64> for Point {
    type Output = Point;
    fn mul(self, other: f64) -> Point {
        Point {
            x: self.x * other,
            y: self.y * other
        }
    }
}

impl Mul<Point> for f64 {
    type Output = Point;
    fn mul(self, other: Point) -> Point {
        Point {
            x: self * other.x,
            y: self * other.y
        }
    }
}

impl Mul<Point> for Point {
    type Output = f64;
    fn mul(self, other: Point) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

impl Div<f64> for Point {
    type Output = Point;
    fn div(self, other: f64) -> Point {
        Point {
            x: self.x / other,
            y: self.y / other
        }
    }
}

impl Neg for Point {
    type Output = Point;
    fn neg(self) -> Point {
        Point {
            x: -self.x,
            y: -self.y
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        self.x += other.x;
        self.y += other.y;
    }
}

