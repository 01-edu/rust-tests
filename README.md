# ⚙️ rust-tests

[![🐳 On Master - Build and Test Docker Image](https://github.com/01-edu/rust-tests/actions/workflows/ga-image-build-master.yml/badge.svg?branch=master)](https://github.com/01-edu/rust-tests/actions/workflows/ga-image-build-master.yml)

Private repository that holds the files needed to build the Rust tests Docker image.

## Structure of the repo

```bash
root
 ├ solutions
 │ └ [exercise_name]       # This is a Cargo project
 ├ tests
 │ ├ [exercise_name]_test  # This is a Cargo project
 │ └ test_exercises.sh
 ├ tests_utility
 │ └ *                     # Resources needed by some tests to run
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

## Real life test environment for one exercise

It runs the exercise in the docker container used by `runner.go` on the platform.

- Use `test_exercises.sh` with `-real` flag.

> If you want to do it manually here is the process:

- Build the image with `docker build -t rust_tests .`
- Create the directory `tests/student`
- Copy the exercise directory from solution into `tests/student`
- Execute the following command:

```bash
docker run --read-only \
   --network none \
   --memory 500M \
   --cpus 2.0 \
   --user 1000:1000 \
   --env EXERCISE=[exercise_name] \
   --env USERNAME=[user_name] \
   --env HOME=/jail \
   --env TMPDIR=/jail \
   --workdir /jail \
   --tmpfs /jail:size=100M,noatime,exec,nodev,nosuid,uid=1000,gid=1000,nr_inodes=5k,mode=1700 \
   --volume "$(pwd)"/tests/student:/jail/student:ro \
   -it rust_tests
```

## Create new exercises

The script `create_exercise.sh` make it easier to have the necessary boilerplate to start working on a new exercise.

- `-l` flag create the solution as `lib` and adds it as a `dependencies` in the `Cargo.toml` of the test.
- The script can accept more than one exercise name and for each name if will repeat the `cargo new` command according to flags.

## Create refs

Use the script `create_refs.sh [NAMES FILE] [TEMPLATE FILE] [INLINE TEMPLATE FILE] [REF ID]`.

- `[NAMES FILE]` is a file with one exercise name per line.
- `[TEMPLATE FILE]` is the template used for the single file ref.
- `[INLINE TEMPLATE FILE]` is the template for the inline ref (like the line adding an exercise into a quest), it will generate one file `inline_refs.txt`.
- `[REF ID]` is the first available ref number to consume.

### Results

For each line into `[NAMES FILE]` the script will create a file `[REF_ID].json` into `refs/` using the template with `%NAME%` and `%REF_ID%` replaced by the corresponding values.
It will also add the inline template into a file named `inline_refs.json` with `%NAME%` and `%REF_ID%` replaced by the corresponding values.

### Example

`bash create_refs.sh new_exercises.txt ./templates/rust_ref ./inline_templates/rust_inline_ref 10000`.
If the `new_exercises.txt` has 10 exercises in it there will be 10 files created in `refs` directory and a new file `inline_refs.json`.
