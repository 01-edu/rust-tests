const BASE: f64 = 32.;
const RATIO: f64 = 9. / 5.;

pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - BASE) / RATIO
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * RATIO + BASE
}
