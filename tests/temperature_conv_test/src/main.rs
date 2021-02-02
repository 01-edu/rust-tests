use temperature_conv::*;

use std::f64::EPSILON;

fn eql(a: f64, b: f64) -> bool {
	(b - a).abs() < EPSILON
}

fn ceil(value: f64, scale: u8) -> f64 {
	let multiplier = 10i64.pow(scale as u32) as f64;
	(value * multiplier).ceil() / multiplier
}


#[test]
fn test_f_to_c() {
	println!("{}", fahrenheit_to_celsius(83.0));

	let rounded = ceil(fahrenheit_to_celsius(20.0),2);
	assert!(eql(rounded, -6.65));

	let rounded2 = ceil(fahrenheit_to_celsius(83.0),2);
	assert!(eql(rounded2, 28.34));
}

#[test]
fn test_c_to_f() {
	assert!(eql(celsius_to_fahrenheit(27.0), 80.6));
	assert!(eql(celsius_to_fahrenheit(0.0), 32.0))
}
