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
IS_LIB=false

create_exercise () {
	exercise=$1
	exercise_test=${exercise}"_test"

    if [[ $IS_LIB == true ]]
    then
        solution_args="--lib"
    fi
    cd solutions
    cargo new $solution_args $exercise

    cd ../tests
    cargo new $exercise_test
    if [[ $IS_LIB == true ]]
    then
        echo "$exercise = { path = \"../../solutions/${exercise}\" }" >> ${exercise_test}/Cargo.toml
    fi
    cd ..
}

if [ -n $ARG ] && ([[ $ARG == '-h' ]] || [[ $ARG == '--help' ]])
then
	echo "Create new exercises boilerplates for Rust

	-h, --help          show this usage screen
    -l                  add `--lib` as argument to cargo new
	[exercise_name]     create one or more exercises (separated by spaces)"
else
	# Arguments parsing
	while [[ $# -gt 0 ]]
	do
		case $1 in
			-l)
			IS_LIB=true
			shift
			;;
			*)
			break
		esac
	done

    while [[ $# -gt 0 ]]
    do
        exercise="${1}"
        create_exercise $exercise
        shift # past argument
    done
fi
