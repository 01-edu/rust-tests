#[cfg(test)]
mod tests {
    use colored::Colorize;
    use events::*;
    use std::time::Duration;

    #[test]
    fn remainder_notification() {
        assert_eq!(
            Event::Remainder("Go to the doctor").notify(),
            Notification {
                size: 50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: "Go to the doctor".to_owned(),
            }
        );
    }

    #[test]
    fn registration_notification() {
        assert_eq!(
            Event::Registration(Duration::from_secs(49094)).notify(),
            Notification {
                size: 30,
                color: (255, 2, 22),
                position: Position::Top,
                content: "You have 13H:38M:14S left before the registration ends".to_owned(),
            }
        );
    }

    #[test]
    fn appointment_notification() {
        assert_eq!(
            Event::Appointment("Go to the doctor").notify(),
            Notification {
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: "Go to the doctor".to_owned(),
            }
        );
    }

    #[test]
    fn holiday_notification() {
        assert_eq!(
            Event::Holiday.notify(),
            Notification {
                size: 25,
                color: (0, 255, 0),
                position: Position::Top,
                content: "Enjoy your holiday".to_owned(),
            }
        );
    }

    #[test]
    fn test_notification_display() {
        let notification = Notification {
            size: 30,
            color: (255, 2, 22),
            position: Position::Top,
            content: "You have 13H:38M:14S left before the registration ends".to_owned(),
        };

        let (r, g, b) = notification.color;
        let formatted_content = notification.content.truecolor(r, g, b);

        let expected = format!(
            "({:?}, {}, {})",
            notification.position, notification.size, formatted_content
        );

        assert_eq!(notification.to_string(), expected);
    }
}
