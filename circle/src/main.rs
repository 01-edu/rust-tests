use raster::{Color, Image};
use std::f64::consts;
// Create a structure that represents a circle
// using the radius and the center point

fn main() {
	let circle = Circle {
		center: Point { x: 500.0, y: 500.0 },
		radius: 150.0,
	};
	let circle1 = Circle {
		center: Point { x: 80.0, y: 115.0 },
		radius: 30.0,
	};
	println!("circle = {:?} area = {}", circle, circle.area());
	println!("circle = {:?} diameter = {}", circle, circle.diameter());
	println!("circle1 = {:?} diameter = {}", circle1, circle1.diameter());
	println!(
		"circle and circle1 intersect = {}",
		circle.intersect(&circle1)
	);
	let mut image = Image::blank(1000, 1000);
	for row in 0..image.height {
		for col in 0..image.width {
			let position = Point {
				x: col as f64,
				y: row as f64,
			};
			if (position.distance(&circle.center) - circle.radius).abs() < 0.5
				|| (position.distance(&circle1.center) - circle1.radius).abs() < 0.5
			{
				image
					.set_pixel(
						col,
						row,
						Color {
							r: 255,
							g: 255,
							b: 255,
							a: 255,
						},
					)
					.unwrap();
			}
		}
	}
	raster::save(&image, "sample.png").unwrap();
}

#[derive(Debug)]
struct Circle {
	center: Point,
	radius: f64,
}

impl Circle {
	fn area(&self) -> f64 {
		consts::PI * self.radius * self.radius
	}
	fn diameter(&self) -> f64 {
		2.0 * self.radius
	}
	fn intersect(&self, c: &Circle) -> bool {
		self.center.distance(&c.center) < c.radius + self.radius
	}
}

#[derive(Debug)]
struct Point {
	x: f64,
	y: f64,
}

impl Point {
	fn distance(&self, other: &Point) -> f64 {
		((self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0)).sqrt()
	}
}
