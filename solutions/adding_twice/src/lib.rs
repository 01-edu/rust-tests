pub const fn add_curry(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

pub const fn twice(f: impl Fn(i32) -> i32) -> impl Fn(i32) -> i32 {
    move |x| f(f(x))
}
