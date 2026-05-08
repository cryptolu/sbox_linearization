#!/bin/bash

for dir in logs_*; do
	echo "$dir"
	for f in $dir/n*; do
		echo -n "$f | "; tail -1 "$f" | egrep -o 'size [0-9]+ ' || echo;
	done | sort >"summary_$dir.txt" 
done