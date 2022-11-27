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

EXERCISES_NAMES="$1"
TEMPLATE_FILE="$2"
INLINE_TEMPLATE_FILE="$3"
REF_ID="$4"

if [ "$#" -ne 4 ]; then
    echo "Create refs for new exercise

    bash ./create_refs.sh [NAMES FILE] [TEMPLATE FILE] [INLINE TEMPLATE FILE] [REF ID]

    [NAMES FILE] a file with one exercise name per line.
    [TEMPLATE FILE] the template used for the single file ref.
    [INLINE TEMPLATE FILE] the template for the inline ref (like the line adding an exercise into a quest).
    [REF ID] the first available ref number to consume."
    exit 1
fi

if [ ! -f "$EXERCISES_NAMES" ]; then
    echo "error: $EXERCISES_NAMES doesn't exists."
    exit 1
fi
if [ ! -f "$TEMPLATE_FILE" ]; then
    echo "error: $TEMPLATE_FILE doesn't exists."
    exit 1
fi
if [ ! -f "$INLINE_TEMPLATE_FILE" ]; then
    echo "error: $INLINE_TEMPLATE_FILE doesn't exists."
    exit 1
fi

TEMPLATE=$(cat "$2")
INLINE_TEMPLATE=$(cat "$3")

mkdir -p refs
echo -n > inline_refs.json

while IFS= read -r exercise; do
    template_result=$(sed "s/%NAME%/$exercise/g" <<< $TEMPLATE)
    template_result=$(sed "s/%REF_ID%/$REF_ID/g" <<< $template_result)
    echo "$template_result" > "refs/${REF_ID}.json"

    inline_result=$(sed "s/%NAME%/$exercise/g" <<< $INLINE_TEMPLATE)
    inline_result=$(sed "s/%REF_ID%/$REF_ID/g" <<< $inline_result)
    echo "$inline_result" >> "inline_refs.json"
    ((REF_ID = REF_ID + 1))
done <"$EXERCISES_NAMES"
