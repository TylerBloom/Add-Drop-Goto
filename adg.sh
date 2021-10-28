#! /bin/bash

output="$(~/.config/adg/adg $@)"

if [[ "$output" ]]
then
	if [[ "$@" == *"-l"* ]]
	then
		echo "${output}"
	else
		echo "${output}"
		cd "${output}"
	fi
fi

