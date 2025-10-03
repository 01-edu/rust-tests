pub struct Car<'a> {
    pub plate_nbr: &'a str,
    pub model: &'a str,
    pub horse_power: u32,
    pub year: u32,
}

pub struct Truck<'a> {
    pub plate_nbr: &'a str,
    pub model: &'a str,
    pub horse_power: u32,
    pub year: u32,
    pub load_tons: u32,
}

pub trait Vehicle {
    fn model(&self) -> &str;
    fn year(&self) -> u32;
}

impl Vehicle for Truck<'_> {
    #[inline]
    fn model(&self) -> &str {
        self.model
    }

    #[inline]
    fn year(&self) -> u32 {
        self.year
    }
}

impl Vehicle for Car<'_> {
    #[inline]
    fn model(&self) -> &str {
        self.model
    }

    #[inline]
    fn year(&self) -> u32 {
        self.year
    }
}

#[inline]
pub fn all_models<const N: usize>(list: [&dyn Vehicle; N]) -> [&str; N] {
    list.map(Vehicle::model)
}
