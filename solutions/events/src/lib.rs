// You're have to design a notification system for a platform
// This events are Remainders, Registrations, Appointments, Holidays
// Create an event handler that depending of the type of event creates
// different notification: different color, different size and
// different position

// The possible positions are Top, Bottom and Center: Create and Enum
// `Position` with those values

// Create a struct called `Notification` with the fields
// size: u32,
// color: (u8, u8, u8),
// position: Position,
// content: String,

// The event that you have to handle are
// enum Event {
// 	Remainder(&str),
// 	Registration(Duration),
// 	Appointment(&str),
// 	Holiday,
// }

// Create a method called `notify`
// fn notify(&self) -> Notification
// That returns a notification with the following caracteristics for
// each
// Remainder:
// size= 50,
// color= (50, 50, 50),
// position= Bottom,
// content= the slice associated to the enum value

// Registration(chrono::Duration),
// size = 30,
// color = (255, 2, 22),
// position = Top,
// content = "You have `duration` left before the registration ends",
// `durations` must be displayed in the form of
// {hours}:{minutes}:{seconds} left for the beginning of the event
// for example if there is two hours 32 minutes and 3 seconds left
// before the registration then the content will be `You have 2:32:2 left before the registration ends`

//		Appointment(text)
// 	size: 100
// 	color: (200, 200, 3)
// 	position: Center
// 	content: text associated to the value

// Holiday
// size: 25
// color: (0, 255, 0)
// position: Top
// content: "Enjoy your holiday"

use chrono::Duration;
use colored::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Position {
    Top,
    Bottom,
    Center,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Notification {
    pub size: u32,
    pub color: (u8, u8, u8),
    pub position: Position,
    pub content: String,
}

#[derive(Debug)]
pub enum Event<'a> {
    Remainder(&'a str),
    Registration(Duration),
    Appointment(&'a str),
    Holiday,
}

use std::fmt;

#[derive(Debug)]
struct DurationInHours {
    hours: i64,
    minutes: i64,
    seconds: i64,
}

impl From<&Duration> for DurationInHours {
    fn from(duration: &Duration) -> DurationInHours {
        let mut left = duration.num_seconds();
        let hours = left / 3600;
        left %= 3600;
        let minutes = left / 60;
        left %= 60;
        let seconds = left;
        DurationInHours {
            hours,
            minutes,
            seconds,
        }
    }
}

impl fmt::Display for DurationInHours {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}H:{}M:{}S", self.hours, self.minutes, self.seconds)
    }
}

impl fmt::Display for Notification {
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

use Event::*;

impl<'a> Event<'a> {
    pub fn notify(&self) -> Notification {
        match self {
            Remainder(text) => Notification {
                size: 50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: text.to_string(),
            },
            Registration(time_left) => Notification {
                size: 30,
                color: (255, 2, 22),
                position: Position::Top,
                content: format!(
                    "You have {} left before the registration ends",
                    DurationInHours::from(time_left)
                ),
            },
            Appointment(text) => Notification {
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: text.to_string(),
            },
            Holiday => Notification {
                size: 25,
                color: (0, 255, 0),
                position: Position::Top,
                content: String::from("Enjoy your holiday"),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remainder_notification() {
        let remainder = Remainder("Go to the doctor");
        let notification = remainder.notify();
        println!("{}", &notification);
        assert_eq!(
            notification,
            Notification {
                size: 50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: "Go to the doctor".to_string(),
            }
        );
    }

    #[test]
    fn registration_notification() {
        let registration = Registration(Duration::seconds(49094));
        let notification = registration.notify();
        println!("{}", registration.notify());
        assert_eq!(
            notification,
            Notification {
                size: 30,
                color: (255, 2, 22),
                position: Position::Top,
                content: "You have 13H:38M:14S left before the registration ends".to_string(),
            }
        );
    }

    #[test]
    fn appointment_notification() {
        let appointment = Appointment("Go to the doctor");
        let notification = appointment.notify();
        println!("{}", &notification);
        assert_eq!(
            notification,
            Notification {
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: "Go to the doctor".to_string(),
            }
        );
    }

    #[test]
    fn holiday_notification() {
        let holiday = Holiday;
        let notification = Holiday.notify();
        println!("{}", holiday.notify());
        assert_eq!(
            notification,
            Notification {
                size: 25,
                color: (0, 255, 0),
                position: Position::Top,
                content: String::from("Enjoy your holiday"),
            }
        );
    }
}
