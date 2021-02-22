#!/usr/bin/env bash

set -euo pipefail
IFS='
'

cp -a /app/tests .

ln -s student solutions

if test "$EXAM_MODE"; then
	cd "student/$EXERCISE"
	cargo init --lib
	cd
fi

if ! test -f "tests/${EXERCISE}_test/Cargo.toml"; then
	echo "No test file found for the exercise : $EXERCISE"
	exit 1
fi

if find student -type f -name '*.rs' -exec grep -q 'std::process' {} +; then
	echo "Your Rust source code cannot use std::process"
	exit 1
fi

cargo test --manifest-path "tests/${EXERCISE}_test/Cargo.toml"
