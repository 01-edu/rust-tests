fn main() {
	println!("{} F = {} C", -459.67, celsius_to_fahrenheit(-459.67));
	println!("{} C = {} F", 0.0, fahrenheit_to_celsius(0.0));
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
	f * 1.8 + 32.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
	(c - 32.0) / 1.8
}
