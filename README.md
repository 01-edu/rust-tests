# rust-tests

Private repository that holds the files needed to build the Rust tests Docker image.

## Structure of the repo
```bash
root
 ├ solutions
 | └ [exercise_name]       # This is a Cargo project
 ├ tests
 | ├ [exercise_name]_test  # This is a Cargo project
 | └ test_exercises.sh
 ├ tests_utility
 | └ *                     # Resources needed by some tests to run
 ├ Dockerfile
 └ entrypoint.sh
```

> This structure should be preserved to ensure the `Dockerfile` and the `test_exercises.sh` work properly.

## How does it works
- The `Dockerfile` will copy `solutions`, `tests` and `tests_utility` into the image.
- It will then download all the necessary crates to run the `solutions` into the `tests`.
- Finally it removes `solutions` since they're not needed anymore.
- When running, the container will execute `entrypoint.sh` which will test the student solution:
   - This process is done offline.
   - A lot of flags are used with `docker run`, if in doubt check with DevOps.
   - The return of `cargo test` is used to assess success or failure of the test.

## Testing and modifying `solutions` and `tests`
With `bash tests/test_exercises.sh` you can:
- Test all exercises
- Test specific exercises
- Auto-format solutions and tests
- Check for non-idiomatic code
- Have detailed feedback with verbose mode

> Run `bash tests/test_exercises.sh -h` for more info about it.

### Prerequisites
- Install `fmt` with `rustup component add rustfmt`
- Install `clippy` with `rustup component add clippy`

> You will need to have `cargo` and `rustup` installed.