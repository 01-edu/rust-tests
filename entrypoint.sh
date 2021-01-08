#!/usr/bin/env bash

set -euo pipefail
IFS='
'

cp -a /app/tests .

ln -s student solutions

if ! test -f "tests/${EXERCISE}_test/Cargo.toml"; then
	echo "No test file found for the exercise : $EXERCISE"
	exit 1
fi

cargo test --manifest-path "tests/${EXERCISE}_test/Cargo.toml"
