#!/usr/bin/env bash

IFS='
'
cd -P "$(dirname "$0")"

ARG=$1
IS_VERBOSE=false

run_test () {
	exercise_dir=$1
	is_verbose=$2
	printf "  Test: %s\n" ${exercise_dir%_test/}
	if [[ $is_verbose == true ]]
	then
		cargo test --manifest-path "$exercise_dir"Cargo.toml
	else
		cargo test -q --manifest-path "$exercise_dir"Cargo.toml >/dev/null
	fi
}

if [ -n $ARG ] && ([[ $ARG == '-h' ]] || [[ $ARG == '--help' ]])
then
	echo "Run cargo test for all the exercises

	-h, --help          show this usage screen
	-t                  show a table with the time it takes to run each exercise
	-v                  show more details for each test
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
	if [[ $# -gt 0 ]] && [[ $1 = '-v' ]]
	then
		IS_VERBOSE=true
		shift
	fi

	if [[ $# -gt 0 ]]
	then
		while [[ $# -gt 0 ]]
		do
			exercise="${1}_test/"
			run_test $exercise $IS_VERBOSE
			shift # past argument
		done
	else
		for dir in */; do
			run_test $dir $IS_VERBOSE
		done
	fi
fi
