use std::f64::consts;

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Self {
            center: Point(x, y),
            radius,
        }
    }

    pub fn area(self) -> f64 {
        consts::PI * self.radius.powi(2)
    }

    pub fn diameter(self) -> f64 {
        2.0 * self.radius
    }

    pub fn intersect(self, c: Self) -> bool {
        self.center.distance(c.center) < c.radius + self.radius
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(self, to: Self) -> f64 {
        ((self.0 - to.0).powi(2) + (self.1 - to.1).powi(2)).sqrt()
    }
}
