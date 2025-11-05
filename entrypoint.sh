#!/usr/bin/env bash
set -eo pipefail
IFS=$'\n'

# support both variables CODE_EDITOR_RUN_ONLY and EXAM_RUN_ONLY
CODE_EDITOR_RUN_ONLY="${CODE_EDITOR_RUN_ONLY:-$EXAM_RUN_ONLY}"
# support both variables CODE_EDITOR_MODE and EXAM_MODE
CODE_EDITOR_MODE="${CODE_EDITOR_MODE:-$EXAM_MODE}"

cp -a /app/tests .
cp -a /app/isolate.sh .
cp -a student solutions

if [ "$CODE_EDITOR_MODE" ]; then
    cd "solutions/$EXERCISE"
    # Support both old/new code editor runners
    if ! echo "$EDITOR_FILES" | tr ',' '\n' | grep -q 'src/main.rs'; then
        if [ "$CODE_EDITOR_RUN_ONLY" ]; then
            mv src/lib.rs src/main.rs 2>/dev/null || true
        fi
    fi
    cargo init
    cd -
fi

if [ ! -f "tests/${EXERCISE}_test/Cargo.toml" ]; then
    echo "No test file found for the exercise: $EXERCISE"
    exit 1
fi

# Ensure EXERCISE is inherited by isolate.sh
export EXERCISE
# Ensure current directory is in PATH for isolate.sh
export PATH=".:$PATH"

if [ "$CODE_EDITOR_RUN_ONLY" ]; then
    cargo run --manifest-path "solutions/$EXERCISE/Cargo.toml" -- "$@"
else
    # 1) Compile tests first, without running
    set +e
    cargo test --no-run --manifest-path "tests/${EXERCISE}_test/Cargo.toml"
    rc=$?
    set -e
    if [ "$rc" -ne 0 ]; then
        echo "Solution did not compile."
        exit 1
    fi

    # 2) Run tests with isolate.sh as runner
    cargo --config 'target."cfg(all())".runner="./isolate.sh"' \
        test --manifest-path "tests/${EXERCISE}_test/Cargo.toml"
fi
