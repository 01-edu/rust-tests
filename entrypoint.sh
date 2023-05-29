#!/usr/bin/env bash

set -eo pipefail
IFS='
'

cp -a /app/tests .
cp -a student solutions

if test "$EXAM_MODE"; then
	cd "solutions/$EXERCISE"
	if test "$EXAM_RUN_ONLY"; then
		mv src/lib.rs src/main.rs 2>&1 ||:
	fi
	cargo init
	cd
fi

if ! test -f "tests/${EXERCISE}_test/Cargo.toml"; then
	echo "No test file found for the exercise : $EXERCISE"
	exit 1
fi

if test "$EXAM_RUN_ONLY"; then
	cargo run --manifest-path "solutions/$EXERCISE/Cargo.toml" -- "$@"
else
	cargo test --manifest-path "tests/${EXERCISE}_test/Cargo.toml"
fi
