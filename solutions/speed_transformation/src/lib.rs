const S_IN_H: f64 = 60. * 60.;
const M_IN_KM: f64 = 1000.;

pub fn km_per_hour_to_meters_per_second(km_h: f64) -> f64 {
    km_h * (M_IN_KM / S_IN_H)
}
