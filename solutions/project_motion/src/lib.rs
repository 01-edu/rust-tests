#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ThrownObject {
    initial_position: Vec2,
    initial_velocity: Vec2,
    elapsed: f64,
}

const ACCELERATION: f64 = -9.8;

impl ThrownObject {
    pub const fn new(position: Vec2, velocity: Vec2) -> Self {
        Self {
            initial_position: position,
            initial_velocity: velocity,
            elapsed: 0.,
        }
    }
}

impl Iterator for ThrownObject {
    type Item = (Vec2, Vec2);

    fn next(&mut self) -> Option<(Vec2, Vec2)> {
        self.elapsed += 1.;

        let t = self.elapsed;

        let pos = Vec2 {
            x: self.initial_position.x + self.initial_velocity.x * t,
            y: self.initial_position.y + self.initial_velocity.y * t + 0.5 * ACCELERATION * t * t,
        };

        let vel = Vec2 {
            x: self.initial_velocity.x,
            y: self.initial_velocity.y + ACCELERATION * t,
        };

        if pos.y <= 0.0 {
            return None;
        }

        Some((pos, vel))
    }
}
