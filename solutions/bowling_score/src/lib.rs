#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: Vec<u32>,
    pins_left: u32,
    rolls_in_frame: u32,
    fill_balls: u32,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            rolls: vec![],
            pins_left: 10,
            rolls_in_frame: 2,
            fill_balls: 0,
        }
    }

    pub fn roll(&mut self, pins: u32) -> Result<(), Error> {
        if pins > self.pins_left {
            return Err(Error::NotEnoughPinsLeft);
        } else if self.rolls.len() >= 20 && self.fill_balls == 0 {
            return Err(Error::GameComplete);
        }
        self.pins_left -= pins;
        self.rolls.push(pins);

        if self.fill_balls != 0 {
            self.fill_balls -= 1;
        }

        if self.pins_left == 0 {
            self.strike_or_spare(pins);
        }

        if self.rolls_in_frame == 1 {
            self.pins_left = 10;
            self.rolls_in_frame = 2;
        } else {
            self.rolls_in_frame -= 1;
        }

        Ok(())
    }

    fn strike_or_spare(&mut self, pins: u32) {
        if self.rolls_in_frame == 2 {
            self.rolls_in_frame -= 1;
            self.rolls.push(0);
        }
        if self.rolls.len() == 20 {
            match pins {
                10 => self.fill_balls = 2,
                _ => self.fill_balls = 1,
            }
        }
    }

    pub fn score(&self) -> Option<u32> {
        if self.rolls.len() < 20 || self.fill_balls != 0 {
            return None;
        }
        let cur_frame = self.rolls.chunks(2);
        let next_1 = cur_frame.clone().skip(1).cycle();
        let next_2 = next_1.clone().skip(1).cycle();

        cur_frame
            .take(10)
            .zip(next_1)
            .zip(next_2)
            .map(
                |((cur, next_1), next_2)| match (cur[0], cur[1], next_1[0]) {
                    (10, _, 10) => 20 + next_2[0],
                    (10, _, _) => 10 + next_1[0] + next_1[1],
                    (a, b, _) if a + b == 10 => 10 + next_1[0],
                    (a, b, _) => a + b,
                },
            )
            .reduce(|acc, score| acc + score)
    }
}
