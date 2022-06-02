// Create a modules, in another file, called geometrical_shapes that keeps all the
// shapes and define two traits Displayable and Drawable

// `Drawable` contains the methods `draw` and `color`
// `Displayable` contains the methods `display`.

// Define them in correspondence with the way they're called in the
// main function

// You have to define structures for Point, Circle, Line, Rectangle
// and Triangle and make the code in `main.rs` compile and run.

// You're free to implement all the shapes with the internal
// structure that you find more adequate, but you have to provide for
// all the shapes an associated function `new` which will be described
// next:

// Point:
// a new point should be created from two i32 values
// Line:
// a new line should be created from references to two points
// also define an associated function called `random` that receives two
// argument the first is the maximum x value a point can have and the
// second the maximum y value that a point can have
// Triangle:
// a new triangle should be created from references to three points
// Rectangle:
// a new rectangle should be created from two reference to points
// Circle
// a new circle should be created from a point represented the center
// and a i32 value representing the radius
// also define an associated function called `random` that receives two
// argument the first is the maximum x value the center point can have
// and the second the maximum y value that the center point can have

// Don't forget to add the dependencies.
mod geometrical_shapes;

use geometrical_shapes as gs;
use gs::{Displayable, Drawable};
use raster::{Color, Image};

fn main() {
    let mut image = Image::blank(1000, 1000);

    gs::Line::random(image.width, image.height).draw(&mut image);

    gs::Point::random(image.width, image.height).draw(&mut image);

    let rectangle = gs::Rectangle::new(&gs::Point::new(150, 300), &gs::Point::new(50, 60));
    rectangle.draw(&mut image);

    let triangle = gs::Triangle::new(
        &gs::Point::new(500, 500),
        &gs::Point::new(250, 700),
        &gs::Point::new(700, 800),
    );
    triangle.draw(&mut image);

    for _ in 1..50 {
        gs::Circle::random(image.width, image.height).draw(&mut image);
    }

    raster::save(&image, "image.png").unwrap();
}

impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color).unwrap();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display() {
        let mut image = Image::blank(100, 100);
        let point = gs::Point::random(image.width, image.height);
        point.draw(&mut image);
        assert_eq!(
            image.get_pixel(point.x, point.y).unwrap().a,
            point.color().a
        );
        assert_eq!(
            image.get_pixel(point.x, point.y).unwrap().r,
            point.color().r
        );
        assert_eq!(
            image.get_pixel(point.x, point.y).unwrap().g,
            point.color().g
        );
        assert_eq!(
            image.get_pixel(point.x, point.y).unwrap().b,
            point.color().b
        );
        let point = gs::Point { x: 200, y: 200 };
        point.draw(&mut image);
    }
    use std::collections::HashMap;

    struct MockDisplay {
        points: HashMap<gs::Point, bool>,
    }

    impl MockDisplay {
        fn new() -> MockDisplay {
            MockDisplay {
                points: HashMap::new(),
            }
        }
    }
    impl Displayable for MockDisplay {
        fn display(&mut self, x: i32, y: i32, _color: Color) {
            self.points.insert(gs::Point { x, y }, true);
        }
    }

    #[test]
    fn point() {
        let point_a = gs::Point { x: 0, y: 3 };
        let mut display = MockDisplay::new();

        point_a.draw(&mut display);
        assert!(display.points.contains_key(&point_a));
        assert!(!display.points.contains_key(&gs::Point { x: 0, y: 4 }));
    }

    #[test]
    fn horizontal_line() {
        let a = gs::Point { x: 0, y: 3 };
        let b = gs::Point { x: 50, y: 3 };
        let line = gs::Line::new(&a, &b);
        let mut display = MockDisplay::new();

        line.draw(&mut display);
        assert!(display.points.contains_key(&a));
        assert!(display.points.contains_key(&b));
        assert!(display.points.contains_key(&gs::Point { x: 25, y: 3 }));
        assert!(!display.points.contains_key(&gs::Point { x: 25, y: 4 }));
        assert!(!display.points.contains_key(&gs::Point { x: 51, y: 3 }));
    }

    #[test]
    fn vertical_line() {
        let point_a = gs::Point { x: 50, y: 0 };
        let point_b = gs::Point { x: 50, y: 40 };
        let line = gs::Line::new(&point_a, &point_b);
        let mut display = MockDisplay::new();
        line.draw(&mut display);
        assert!(display.points.contains_key(&gs::Point { x: 50, y: 25 }));
        assert!(display.points.contains_key(&point_a));
        assert!(display.points.contains_key(&point_b));
        assert!(!display.points.contains_key(&gs::Point { x: 50, y: 50 }));
    }

    #[test]
    fn slope_between_zero_and_one() {
        let point_a = gs::Point { x: 0, y: 0 };
        let point_b = gs::Point { x: 3, y: 3 };
        let line = gs::Line::new(&point_a, &point_b);
        let mut display = MockDisplay::new();
        line.draw(&mut display);

        assert!(display.points.contains_key(&gs::Point { x: 1, y: 1 }));
        assert!(display.points.contains_key(&point_a));
        assert!(display.points.contains_key(&gs::Point { x: 2, y: 2 }));
        assert!(display.points.contains_key(&point_b));
        assert!(!display.points.contains_key(&gs::Point { x: 1, y: 0 }));
    }

    #[test]
    fn slope_between_zero_and_negative_one() {
        let point_a = gs::Point { x: 0, y: 3 };
        let point_b = gs::Point { x: 6, y: 0 };
        let line = gs::Line::new(&point_a, &point_b);
        let mut display = MockDisplay::new();
        line.draw(&mut display);

        assert!(display.points.contains_key(&point_a));
        assert!(display.points.contains_key(&point_b));
        assert!(display.points.contains_key(&gs::Point { x: 1, y: 2 }));
        assert!(display.points.contains_key(&gs::Point { x: 4, y: 1 }));
        assert!(display.points.contains_key(&gs::Point { x: 2, y: 2 }));
        assert!(!display.points.contains_key(&gs::Point { x: 1, y: 0 }));
    }

    #[test]
    fn positive_slope_greater_than_one() {
        let point_a = gs::Point { x: 7, y: 3 };
        let point_b = gs::Point { x: 9, y: 15 };
        let line = gs::Line::new(&point_a, &point_b);
        let mut display = MockDisplay::new();
        line.draw(&mut display);

        assert!(display.points.contains_key(&point_a));
        assert!(display.points.contains_key(&point_b));
        assert!(display.points.contains_key(&gs::Point { x: 8, y: 7 }));
        assert!(display.points.contains_key(&gs::Point { x: 8, y: 6 }));
        assert!(!display.points.contains_key(&gs::Point { x: 1, y: 0 }));
    }

    #[test]
    fn slope_less_than_negative_one() {
        let point_a = gs::Point { x: 7, y: 15 };
        let point_b = gs::Point { x: 9, y: 3 };
        let line = gs::Line::new(&point_a, &point_b);
        let mut display = MockDisplay::new();
        line.draw(&mut display);

        assert!(display.points.contains_key(&point_a));
        assert!(display.points.contains_key(&point_b));
        assert!(display.points.contains_key(&gs::Point { x: 8, y: 9 }));
        assert!(!display.points.contains_key(&gs::Point { x: 1, y: 0 }));
    }

    #[test]
    fn triangle() {
        let a = gs::Point { x: 7, y: 9 };
        let b = gs::Point { x: 0, y: 0 };
        let c = gs::Point { x: 7, y: 1 };
        let line_a_b = gs::Line::new(&a, &b);
        let line_b_c = gs::Line::new(&b, &c);
        let line_c_a = gs::Line::new(&c, &a);
        let mut display = MockDisplay::new();
        let mut display2 = MockDisplay::new();
        line_a_b.draw(&mut display);
        line_b_c.draw(&mut display);
        line_c_a.draw(&mut display);

        let triangle = gs::Triangle::new(&a, &b, &c);
        triangle.draw(&mut display2);
        assert_eq!(display.points, display2.points);
        assert_eq!(display.points, display2.points);
        assert_eq!(display.points, display2.points);
    }
}
