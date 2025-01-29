#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Light {
    pub alias: String,
    pub brightness: u8,
}

impl Light {
    pub fn new(alias: &str) -> Self {
        Self {
            alias: alias.to_owned(),
            brightness: Default::default(),
        }
    }
}

pub fn change_brightness(lights: &mut [Light], alias: &str, value: u8) {
    if let Some(v) = lights.iter_mut().find(|l| l.alias == alias) {
        v.brightness = value;
    }
}
