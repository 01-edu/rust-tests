#!/usr/bin/env bash

set -euo pipefail
IFS='
'

cp -a /app/public .

ln -s student rust-piscine-solutions

if ! test -f "public/rust/tests/${EXERCISE}_test/Cargo.toml"; then
	echo "No test file found for the exercise : $EXERCISE"
	exit 1
fi

cargo test --manifest-path "public/rust/tests/${EXERCISE}_test/Cargo.toml"
