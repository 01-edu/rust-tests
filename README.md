# rust-tests

Private repository that holds the files needed to build the Rust tests Docker image

## Testing

In order to test a solution for an exercise you will need to place the testing
solution inside the `solutions` folder in the `src/` folder of the exercise in
case.

After that, in the `tests` folder of the exercise, run `cargo test`.

### Example

In order to test the exercise `middle_day`, after being inside `rust_tests/`:

```sh
# depending on the exercise you will need to change different files
> $ code solutions/middle_day/src/lib.rs
# place the tested code in the file
> $ cargo test --manifest-path tests/middle_day_test/Cargo.toml
```
