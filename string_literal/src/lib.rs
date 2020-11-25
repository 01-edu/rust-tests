/*
## string literal

### Instructions

Create the following functions:

- `is_empty`, that returns true if a string is empty
- `contains`, that returns true if the string contains a pattern given
- `split_at`, that divides a string in two returning a tuple


> This exercise will test the **heap allocation** of your function!
> So try your best to allocate the minimum data on the heap! (hit: &str)

### Notions

- https://doc.rust-lang.org/1.22.0/book/first-edition/the-stack-and-the-heap.html
- https://doc.rust-lang.org/rust-by-example/primitives/literals.html

*/

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn is_empty(v: &str) -> bool {
    // v.split("").collect::<Vec<&str>>().len() == 0 // -> this is the bad one for example it will allocate on the heap
    v.is_empty()
}

fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

fn split_at(v: &str, index: usize) -> (&str, &str) {
    v.split_at(index)
}

#[cfg(test)]
mod tests {
    use super::*;
    use jemalloc_ctl::{epoch, stats};

    fn is_empty_sol(v: &str) -> bool {
        v.is_empty()
    }

    fn contains_sol(v: &str, pat: &str) -> bool {
        v.contains(pat)
    }

    fn split_at_sol(v: &str, index: usize) -> (&str, &str) {
        v.split_at(index)
    }

    #[test]
    fn test_memory() {
        // the statistics tracked by jemalloc are cached
        // The epoch controls when they are refreshed
        let e = epoch::mib().unwrap();
        // allocated: number of bytes allocated by the application
        let allocated = stats::allocated::mib().unwrap();
        let test_value = "4of Fo1r pe6ople g3ood th5e the2";

        is_empty_sol("");
        contains_sol("merda", "er");
        split_at_sol("merda", 2);
        // this will advance with the epoch giving the its old value
        // where we read the updated heap allocation using the `allocated.read()`
        e.advance().unwrap();
        let solution = allocated.read().unwrap();

        is_empty("");
        contains("merda", "er");
        split_at("merda", 2);

        e.advance().unwrap();
        let student = allocated.read().unwrap();

        assert_eq!(solution, student);
    }
}