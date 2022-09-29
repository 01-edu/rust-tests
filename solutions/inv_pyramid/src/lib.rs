// In general it's better to use `usize` rather than `u32` for variables that represent
// addressable memory. Like the index or length of a vector, slice or string
pub fn inv_pyramid(st: String, max: usize) -> Vec<String> {
    // Vec::with_capacity() creates a buffer with the size that
    // we define. This makes the first operation be more costly because it
    // allocates the memory at that moment but make the push calls more efficient
    // because it avoids extra allocation
    // It's good to use it if you know the final length of
    // the vector
    let mut vec = Vec::with_capacity(2 * max);
    // In truth the length of the vector will always be ( 2*max - 1 ) at the
    // end, but it doesn't make a lot of difference to allocate an
    // extra element and that way I don't have to handle the case
    // where max == 0 which cause a subtraction in overflow

    // Pushes a string with `amount` number of spaces and the string
    // `st` repeated `amount` number of times
    let mut create_line = |amount| vec.push(format!("{:>1$}", st.repeat(amount), 2 * amount));

    for amount in 1..max + 1 {
        create_line(amount)
    }

    // the `rev` method reverses the range
    for amount in (1..max).rev() {
        create_line(amount)
    }
    return vec;
}
