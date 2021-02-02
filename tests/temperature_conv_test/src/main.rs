use temperature_conv::*;
use math::round;

use std::f64::EPSILON;

fn eql(a: f64, b: f64) -> bool {
	(b - a).abs() < EPSILON
}

#[test]
fn test_f_to_c() {
	println!("{}", fahrenheit_to_celsius(83.0));

	let rounded = round::ceil(fahrenheit_to_celsius(20.0),2)
	assert!(eql(rounded, -6.65));

	let rounded = round::ceil(fahrenheit_to_celsius(83.0),2)
	assert!(eql(fahrenheit_to_celsius(83.0), 28.34));
}

#[test]
fn test_c_to_f() {
	assert!(eql(celsius_to_fahrenheit(27.0), 80.6));
	assert!(eql(celsius_to_fahrenheit(0.0), 32.0))
}
