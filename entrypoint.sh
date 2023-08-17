#!/usr/bin/env bash

set -eo pipefail
IFS='
'

# support both variables CODE_EDITOR_RUN_ONLY and EXAM_RUN_ONLY
CODE_EDITOR_RUN_ONLY="${CODE_EDITOR_RUN_ONLY:-$EXAM_RUN_ONLY}"
# support both variables CODE_EDITOR_MODE and EXAM_MODE
CODE_EDITOR_MODE="${CODE_EDITOR_MODE:-$EXAM_MODE}"

cp -a /app/tests .
cp -a student solutions

if test "$CODE_EDITOR_MODE"; then
	cd "solutions/$EXERCISE"
	if test "$CODE_EDITOR_RUN_ONLY"; then
		mv src/lib.rs src/main.rs 2>&1 ||:
	fi
	cargo init
	cd
fi

if ! test -f "tests/${EXERCISE}_test/Cargo.toml"; then
	echo "No test file found for the exercise : $EXERCISE"
	exit 1
fi

if test "$CODE_EDITOR_RUN_ONLY"; then
	cargo run --manifest-path "solutions/$EXERCISE/Cargo.toml" -- "$@"
else
	cargo test --manifest-path "tests/${EXERCISE}_test/Cargo.toml"
fi
