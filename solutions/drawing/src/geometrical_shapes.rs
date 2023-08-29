use raster::Color;
use std::f64::{self, consts};

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

pub trait Drawable {
    fn draw<I: Displayable>(&self, image: &mut I);
    fn color(&self) -> Color {
        Color::white()
    }
}

#[derive(Debug)]
pub struct Rectangle {
    pub upper_left_corner: Point,
    pub lower_right_corner: Point,
}

impl Rectangle {
    #[allow(dead_code)]
    pub fn new(point1: &Point, point2: &Point) -> Rectangle {
        Rectangle {
            upper_left_corner: point1.clone(),
            lower_right_corner: point2.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Triangle {
    pub vertices: (Point, Point, Point),
}

#[allow(dead_code)]
impl Triangle {
    pub fn new(a: &Point, b: &Point, c: &Point) -> Triangle {
        Triangle {
            vertices: (a.clone(), b.clone(), c.clone()),
        }
    }
}

#[derive(Debug)]
pub struct Line {
    pub a: Point,
    pub b: Point,
}

impl Line {
    pub fn new(beg: &Point, end: &Point) -> Line {
        Line {
            a: beg.clone(),
            b: end.clone(),
        }
    }

    pub fn random(limit_x: i32, limit_y: i32) -> Line {
        Line::new(
            &Point::random(limit_x, limit_y),
            &Point::random(limit_x, limit_y),
        )
    }
}

#[derive(Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: i32,
    pub color: Color,
}

#[allow(dead_code)]
impl Circle {
    pub fn new(x: i32, y: i32, radius: i32) -> Circle {
        Circle {
            center: Point { x, y },
            radius,
            color: Color::white(),
        }
    }
    #[allow(dead_code)]
    pub fn area(&self) -> f64 {
        consts::PI * self.radius as f64 * self.radius as f64
    }
    #[allow(dead_code)]
    pub fn diameter(&self) -> i32 {
        2 * self.radius
    }
    #[allow(dead_code)]
    pub fn intersect(&self, c: &Circle) -> bool {
        self.center.distance(&c.center) < c.radius as f64 + self.radius as f64
    }
    pub fn random(limit_x: i32, limit_y: i32) -> Circle {
        let red = rand::thread_rng().gen_range(0..255);
        let blue = rand::thread_rng().gen_range(0..255);
        let green = rand::thread_rng().gen_range(0..255);
        Circle {
            center: Point::random(limit_x, limit_y),
            radius: rand::thread_rng().gen_range(9..500),
            color: raster::Color::rgb(red, green, blue),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
use rand::Rng;
impl Point {
    #[allow(dead_code)]
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
    #[allow(dead_code)]
    pub fn distance(&self, other: &Point) -> f64 {
        ((self.x as f64 - other.x as f64).powf(2.0) + (self.y as f64 - other.y as f64).powf(2.0))
            .sqrt()
    }
    pub fn random(limit_x: i32, limit_y: i32) -> Point {
        Point {
            x: rand::thread_rng().gen_range(0..limit_x),
            y: rand::thread_rng().gen_range(0..limit_y),
        }
    }
}
impl Drawable for Point {
    fn draw<I: Displayable>(&self, image: &mut I) {
        image.display(self.x, self.y, self.color())
    }
}
impl Drawable for Rectangle {
    fn draw<I: Displayable>(&self, image: &mut I) {
        let upper_right_corner = Point {
            x: self.lower_right_corner.x,
            y: self.upper_left_corner.y,
        };
        let lower_left_corner = Point {
            x: self.upper_left_corner.x,
            y: self.lower_right_corner.y,
        };
        Line::new(&self.upper_left_corner, &upper_right_corner).draw(image);
        Line::new(&upper_right_corner, &self.lower_right_corner).draw(image);
        Line::new(&self.lower_right_corner, &lower_left_corner).draw(image);
        Line::new(&lower_left_corner, &self.upper_left_corner).draw(image);
    }
}

impl Drawable for Triangle {
    fn draw<I: Displayable>(&self, image: &mut I) {
        Line::new(&self.vertices.0, &self.vertices.1).draw(image);
        Line::new(&self.vertices.1, &self.vertices.2).draw(image);
        Line::new(&self.vertices.2, &self.vertices.0).draw(image);
    }
}

use std::cmp;
impl Drawable for Line {
    fn draw<I: Displayable>(&self, image: &mut I) {
        let beg = self.a.clone();
        let end = self.b.clone();
        let dx = (end.x - beg.x).abs();
        let dy = (end.y - beg.y).abs();
        let is_a_vertical_line = dx == 0;

        if is_a_vertical_line {
            let start = cmp::min(beg.y, end.y);
            let fin = cmp::max(beg.y, end.y);
            for y in start..fin + 1 {
                image.display(self.a.x, y, self.color());
            }
            return;
        }

        let slope = (beg.y - end.y) as f64 / (beg.x - end.x) as f64;

        let inc = if slope < 0.0 { -1 } else { 1 };
        let slope_between_zero_and_one = |image: &mut I| {
            let a = 2 * dy;
            let b = a - 2 * dx;
            let mut p = a - dx;
            let start = cmp::min(beg.x, end.x);
            let fin = cmp::max(beg.x, end.x) + 1;
            let mut y = if start == beg.x { beg.y } else { end.y };
            for x in start..fin {
                image.display(x, y, self.color());
                if p < 0 {
                    p += a;
                } else {
                    y += inc;
                    p += b;
                }
            }
        };
        let slope_bigger_than_one = |image: &mut I| {
            let a = 2 * dx;
            let b = a - 2 * dy;
            let mut p = a - dy;
            let start = cmp::min(beg.y, end.y);
            let fin = cmp::max(beg.y, end.y) + 1;
            let mut x = if start == beg.y { beg.x } else { end.x };
            for y in start..fin {
                image.display(x, y, self.color());
                if p < 0 {
                    p += a;
                } else {
                    x += inc;
                    p += b;
                }
            }
        };
        if slope.abs() > 1.0 {
            slope_bigger_than_one(image);
        } else {
            slope_between_zero_and_one(image);
        }
    }
}

impl Drawable for Circle {
    fn draw<I: Displayable>(&self, image: &mut I) {
        let mut x = 0;
        let mut y = self.radius;
        let mut d: i64 = 3 - 2 * self.radius as i64;
        while x < y {
            image.display(y + self.center.x, x + self.center.y, self.color());
            image.display(x + self.center.x, y + self.center.y, self.color());
            image.display(-x + self.center.x, y + self.center.y, self.color());
            image.display(-y + self.center.x, x + self.center.y, self.color());
            image.display(-y + self.center.x, -x + self.center.y, self.color());
            image.display(-x + self.center.x, -y + self.center.y, self.color());
            image.display(x + self.center.x, -y + self.center.y, self.color());
            image.display(y + self.center.x, -x + self.center.y, self.color());
            if d < 0 {
                d = d + 4 * x as i64 + 6;
                x = x + 1;
            } else {
                d = d + 4 * (x - y) as i64 + 10;
                x = x + 1;
                y = y - 1;
            }
        }
    }
    fn color(&self) -> Color {
        self.color.clone()
    }
}
