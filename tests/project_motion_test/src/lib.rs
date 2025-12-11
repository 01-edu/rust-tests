#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use project_motion::*;

    #[test]
    fn test_without_initial_velocity() {
        assert_eq!(
            ThrownObject::new(Vec2 { x: 50., y: 50. }, Vec2 { x: 0., y: 0. }).collect::<Vec<_>>(),
            [
                (Vec2 { x: 50., y: 45.1 }, Vec2 { x: 0., y: -9.8 }),
                (Vec2 { x: 50., y: 30.4 }, Vec2 { x: 0., y: -19.6 }),
                (
                    Vec2 {
                        x: 50.,
                        y: 5.899999999999999
                    },
                    Vec2 {
                        x: 0.,
                        y: -29.400000000000002
                    }
                ),
            ]
        );
    }

    #[test]
    fn test_with_velocity() {
        assert_eq!(
            ThrownObject::new(Vec2 { x: 0., y: 50. }, Vec2 { x: 10., y: 10. }).collect::<Vec<_>>(),
            [
                (
                    Vec2 { x: 10., y: 55.1 },
                    Vec2 {
                        x: 10.,
                        y: 0.1999999999999993
                    }
                ),
                (
                    Vec2 { x: 20., y: 50.4 },
                    Vec2 {
                        x: 10.,
                        y: -9.600000000000001
                    }
                ),
                (
                    Vec2 { x: 30., y: 35.9 },
                    Vec2 {
                        x: 10.,
                        y: -19.400000000000002
                    }
                ),
                (
                    Vec2 {
                        x: 40.,
                        y: 11.599999999999994
                    },
                    Vec2 {
                        x: 10.,
                        y: -29.200000000000003
                    }
                )
            ]
        );
    }

    #[test]
    fn test_with_negative_velocity() {
        assert_eq!(
            ThrownObject::new(Vec2 { x: -10., y: 50. }, Vec2 { x: -10., y: -10. })
                .collect::<Vec<_>>(),
            [
                (Vec2 { x: -20., y: 35.1 }, Vec2 { x: -10., y: -19.8 }),
                (
                    Vec2 {
                        x: -30.,
                        y: 10.399999999999999
                    },
                    Vec2 { x: -10., y: -29.6 }
                ),
            ]
        );
    }

    #[test]
    fn test_with_zero() {
        assert_eq!(
            ThrownObject::new(Vec2 { x: 0., y: 0. }, Vec2 { x: 0., y: 0. }).collect::<Vec<_>>(),
            []
        );
    }
}
