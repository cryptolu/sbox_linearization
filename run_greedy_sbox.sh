#!/bin/bash

TIMELIMIT="$1"
LOGFOLDER="$2"
SBOX_PATH="$3"
SBOX_NAME=$(basename "$SBOX_PATH")
STAMP=$(date '+%Y%m%d_%H%M%S')
LOGFILE="$LOGFOLDER/$SBOX_NAME.greedyext.$STAMP.txt"

echo "Running sbox file $SBOX_NAME for $TIMELIMIT seconds, log file $LOGFILE"

bzcat "$SBOX_PATH" | timeout "$TIMELIMIT" ./greedy_extension/target/release/greedy_extention >"$LOGFILE"

echo -n "Finished sbox file $SBOX_PATH for $TIMELIMIT seconds (into $LOGFILE) | "
tail -1 "$LOGFILE"