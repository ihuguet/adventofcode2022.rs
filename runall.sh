#!/bin/sh

run() {
	for bin in target/release/day[0-9][0-9]; do
		echo running $bin
		time $bin
		echo
	done

	echo
	echo TOTAL:
}

cargo build --release || exit 1
time run
