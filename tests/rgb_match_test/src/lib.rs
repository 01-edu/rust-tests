use rgb_match::*;

#[test]
fn test_one() {
    let c = Color {
        r: 255,
        g: 200,
        b: 10,
        a: 30,
    };
    // swap r
    assert_eq!(
        c.swap(c.r, c.b),
        Color {
            r: 10,
            g: 200,
            b: 255,
            a: 30
        }
    );
    assert_eq!(
        c.swap(c.r, c.g),
        Color {
            r: 200,
            g: 255,
            b: 10,
            a: 30
        }
    );
    assert_eq!(
        c.swap(c.r, c.a),
        Color {
            r: 30,
            g: 200,
            b: 10,
            a: 255
        }
    );

    // swap g
    assert_eq!(
        c.swap(c.g, c.r),
        Color {
            r: 200,
            g: 255,
            b: 10,
            a: 30
        }
    );
    assert_eq!(
        c.swap(c.g, c.b),
        Color {
            r: 255,
            g: 10,
            b: 200,
            a: 30
        }
    );
    assert_eq!(
        c.swap(c.g, c.a),
        Color {
            r: 255,
            g: 30,
            b: 10,
            a: 200
        }
    );

    // swap b
    assert_eq!(
        c.swap(c.b, c.r),
        Color {
            r: 10,
            g: 200,
            b: 255,
            a: 30
        }
    );
    assert_eq!(
        c.swap(c.b, c.g),
        Color {
            r: 255,
            g: 10,
            b: 200,
            a: 30
        }
    );
    assert_eq!(
        c.swap(c.b, c.a),
        Color {
            r: 255,
            g: 200,
            b: 30,
            a: 10
        }
    );

    // swap a
    assert_eq!(
        c.swap(c.a, c.r),
        Color {
            r: 30,
            g: 200,
            b: 10,
            a: 255
        }
    );
    assert_eq!(
        c.swap(c.a, c.b),
        Color {
            r: 255,
            g: 200,
            b: 30,
            a: 10
        }
    );
    assert_eq!(
        c.swap(c.a, c.g),
        Color {
            r: 255,
            g: 30,
            b: 10,
            a: 200
        }
    );
}
