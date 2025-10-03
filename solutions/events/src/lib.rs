use colored::*;
use std::{fmt, time::Duration};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Position {
    Top,
    Bottom,
    Center,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Notification {
    pub size: u32,
    pub color: (u8, u8, u8),
    pub position: Position,
    pub content: String,
}

#[derive(Clone, Copy)]
pub enum Event<'a> {
    Remainder(&'a str),
    Registration(Duration),
    Appointment(&'a str),
    Holiday,
}

#[derive(Clone, Copy)]
struct ReadableDuration {
    hours: u64,
    minutes: u64,
    seconds: u64,
}

impl From<Duration> for ReadableDuration {
    fn from(duration: Duration) -> Self {
        let total_secs = duration.as_secs();
        let hours = total_secs / 3600;
        let minutes = (total_secs % 3600) / 60;
        let seconds = total_secs % 60;
        Self {
            hours,
            minutes,
            seconds,
        }
    }
}

impl fmt::Display for ReadableDuration {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}H:{}M:{}S", self.hours, self.minutes, self.seconds)
    }
}

impl fmt::Display for Notification {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({:?}, {}, {})",
            self.position,
            self.size,
            self.content
                .truecolor(self.color.0, self.color.1, self.color.2)
        )
    }
}

impl Event<'_> {
    pub fn notify(self) -> Notification {
        match self {
            Event::Remainder(text) => Notification {
                size: 50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: text.to_owned(),
            },
            Event::Registration(time_left) => Notification {
                size: 30,
                color: (255, 2, 22),
                position: Position::Top,
                content: format!(
                    "You have {} left before the registration ends",
                    ReadableDuration::from(time_left)
                ),
            },
            Event::Appointment(text) => Notification {
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: text.to_owned(),
            },
            Event::Holiday => Notification {
                size: 25,
                color: (0, 255, 0),
                position: Position::Top,
                content: "Enjoy your holiday".to_owned(),
            },
        }
    }
}
