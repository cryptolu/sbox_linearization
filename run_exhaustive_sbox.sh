#!/bin/bash

RE_EXE=${RE_EXE:="./exhaustive_reduction/ReductionExhaustive8"}

LOGFOLDER="$1"
SBOX_PATH="$2"
MAXVAL="$3"
LB=$(($MAXVAL+1))
SBOX_NAME=$(basename "$SBOX_PATH")
STAMP=$(date '+%Y%m%d_%H%M%S')
LOGFILE="$LOGFOLDER/$SBOX_NAME.exhaustred.LB$LB.$STAMP.txt"

echo "Running sbox file $SBOX_NAME, log file $LOGFILE"

bzcat "$SBOX_PATH" | /usr/bin/time -v "$RE_EXE" "$LB" &>"$LOGFILE"

echo -n "Finished sbox file $SBOX_PATH for $TIMELIMIT seconds (into $LOGFILE) | "
tail -1 "$LOGFILE"