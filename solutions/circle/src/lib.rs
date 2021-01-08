//use raster::{Color, Image};
use std::f64::consts;

// Create a structure that represents a circle
// using the radius and the center point

// fn main() {
// 	let circle = Circle::new(500.0, 500.0, 150.0);
// 	let circle1 = Circle {
// 		center: Point { x: 80.0, y: 115.0 },
// 		radius: 30.0,
// 	};
// 	let point_a = Point { x: 1.0, y: 1.0 };
// 	let point_b = Point { x: 0.0, y: 0.0 };
// 	println!("circle = {:?} area = {}", circle, circle.area());
// 	println!("circle = {:?} diameter = {}", circle, circle.diameter());
// 	println!("circle1 = {:?} diameter = {}", circle1, circle1.diameter());
// 	println!(
// 		"circle and circle1 intersect = {}",
// 		circle.intersect(&circle1)
// 	);

// 	println!(
// 		"distance between {:?} and {:?} is {}",
// 		point_a,
// 		point_b,
// 		point_a.distance(&point_b)
// 	);

// 	// let mut image = Image::blank(1000, 1000);
// 	// for row in 0..image.height {
// 	// 	for col in 0..image.width {
// 	// 		let position = Point {
// 	// 			x: col as f64,
// 	// 			y: row as f64,
// 	// 		};
// 	// 		if (position.distance(&circle.center) - circle.radius).abs() < 0.5
// 	// 			|| (position.distance(&circle1.center) - circle1.radius).abs() < 0.5
// 	// 		{
// 	// 			image
// 	// 				.set_pixel(
// 	// 					col,
// 	// 					row,
// 	// 					Color {
// 	// 						r: 255,
// 	// 						g: 255,
// 	// 						b: 255,
// 	// 						a: 255,
// 	// 					},
// 	// 				)
// 	// 				.unwrap();
// 	// 		}
// 	// 	}
// 	// }
// 	// raster::save(&image, "sample.png").unwrap();
// }

#[derive(Debug)]
pub struct Circle {
	pub center: Point,
	pub radius: f64,
}

impl Circle {
	pub fn new(x: f64, y: f64, radius: f64) -> Circle {
		Circle {
			center: Point { x, y },
			radius,
		}
	}
	pub fn area(&self) -> f64 {
		consts::PI * self.radius * self.radius
	}
	pub fn diameter(&self) -> f64 {
		2.0 * self.radius
	}
	pub fn intersect(&self, c: &Circle) -> bool {
		self.center.distance(&c.center) < c.radius + self.radius
	}
}

#[derive(Debug)]
pub struct Point {
	pub x: f64,
	pub y: f64,
}

impl Point {
	pub fn distance(&self, other: &Point) -> f64 {
		((self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0)).sqrt()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::f64;

	fn approx_eq(a: f64, b: f64) -> bool {
		(a - b).abs() < f64::EPSILON
	}

	#[test]
	fn test_new_circle() {
		let circle = Circle::new(500.0, 400.0, 150.0);
		assert!(approx_eq(circle.radius, 150.0));
		assert!(approx_eq(circle.center.x, 500.0));
		assert!(approx_eq(circle.center.y, 400.0));
	}

	#[test]
	fn test_distance() {
		let a = Point { x: 0.0, y: 1.0 };
		let b = Point { x: 0.0, y: 0.0 };
		assert!(approx_eq(a.distance(&b), 1.0));
		let a = Point { x: 1.0, y: 0.0 };
		let b = Point { x: 0.0, y: 0.0 };
		assert!(approx_eq(a.distance(&b), 1.0));
		let a = Point { x: 1.0, y: 1.0 };
		let b = Point { x: 0.0, y: 0.0 };
		assert!(approx_eq(a.distance(&b), f64::sqrt(2.0)));
	}

	#[test]
	fn test_area() {
		let circle = Circle::new(500.0, 400.0, 150.0);
		assert!(approx_eq(circle.area(), 70685.83470577035));
	}

	#[test]
	fn test_intersection() {
		let circle = Circle::new(500.0, 500.0, 150.0);
		let circle1 = Circle::new(80.0, 115.0, 30.0);
		assert!(!circle.intersect(&circle1));
		let circle = Circle::new(100.0, 300.0, 150.0);
		let circle1 = Circle::new(80.0, 115.0, 100.0);
		assert!(circle.intersect(&circle1));
	}
}
