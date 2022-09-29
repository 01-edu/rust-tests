#!/usr/bin/env bash

IFS='
'
cd -P "$(dirname "$0")"

NC="\033[0m"
WHT="\033[0;37m"
BLK="\033[0;30m"
RED="\033[0;31m"
YEL="\033[0;33m"
BLU="\033[0;34m"
GRN="\033[0;32m"

ARG=$1
IS_VERBOSE=false
CARGO_FORMAT=false
CARGO_CLIPPY=false
TEST_EXERCISES=true

run_test () {
	exercise_dir=$1
	exercise_name=${exercise_dir%_test/}
	if [[ $CARGO_FORMAT == true ]]
	then
		printf "  ${GRN}[FORMAT]${NC} %s\n" $exercise_name
		cargo fmt --manifest-path "$exercise_dir"Cargo.toml
		cargo fmt --manifest-path ../solutions/"${exercise_dir%_test/}"/Cargo.toml
	fi
	if [[ $CARGO_CLIPPY == true ]]
	then
		printf "  ${YEL}[CLIPPY]${NC} %s\n" $exercise_name
		cargo clippy -q --manifest-path "$exercise_dir"Cargo.toml
		cargo clippy -q --manifest-path ../solutions/"${exercise_dir%_test/}"/Cargo.toml
	fi
	if [[ $TEST_EXERCISES == true ]]
	then
		printf "  ${BLU}[TEST  ]${NC} %s\n" $exercise_name
		if [[ $IS_VERBOSE == true ]]
		then
			cargo test --manifest-path "$exercise_dir"Cargo.toml
		else
			cargo test -q --manifest-path "$exercise_dir"Cargo.toml >/dev/null
		fi
	fi
}

if [ -n $ARG ] && ([[ $ARG == '-h' ]] || [[ $ARG == '--help' ]])
then
	echo "Run cargo test for all the exercises

	-h, --help          show this usage screen
	-t                  show a table with the time it takes to run each exercise
	-v                  show more details for each test
	-f                  apply \"cargo fmt\" to the exercises
	-c                  run \"cargo clippy\" to the exercises
	-n                  do NOT run \"cargo test\" on the exercises
	[exercise_name]+    test one or more selected exercises
	[NO ARGUMENTS]      test all exercises in test directory"
elif [[ $ARG == '-t' ]]
then
	printf "NOTICE: This could take some minutes before to show any output\n"
	printf "| %-26s| %-14s|\n" Exercise Time
	# Print a table with the time that took to test each exercise
	for dir in */; do
		exercise_name=${dir%_test/};
		# Don't clean the folder that don't exist
		# This are the only exercises that don't follow the
		# pattern (1 solution -> 1 crate)
		if [ $exercise_name != 'matrix_mult' -a $exercise_name != 'matrix_ops' -a $exercise_name != 'roman_numbers_iter' ]
		then
			cargo clean --manifest-path ../solutions/"$exercise_name"/Cargo.toml;
		fi;
		cargo clean --manifest-path "$dir"Cargo.toml;

		time=$(/usr/bin/time -f '%e secs.' cargo test -q --manifest-path "$dir"Cargo.toml 2>&1 1 > /dev/null | grep secs);
		printf "| %-26s| %-14s|\n" $exercise_name "$time"
	done |
		sort -rn -k4
else
	# Arguments parsing
	while [[ $# -gt 0 ]]
	do
		case $1 in
			-v)
			IS_VERBOSE=true
			shift
			;;
			-f)
			CARGO_FORMAT=true
			shift
			;;
			-c)
			CARGO_CLIPPY=true
			shift
			;;
			-n)
			TEST_EXERCISES=false
			shift
			;;
			*)
			break
		esac
	done

	if [[ $# -gt 0 ]]
	then
		while [[ $# -gt 0 ]]
		do
			exercise="${1}_test/"
			run_test $exercise
			shift # past argument
		done
	else
		for dir in */; do
			run_test $dir
		done
	fi
fi
