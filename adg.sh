#! /bin/bash

output="$(~/.config/adg/adg $@)"

if [[ "$output" ]]
then
	if [[ "$@" == *"list"* ]]
	then
		echo "${output}"
	else
		echo "${output}"
		cd "${output}"
	fi
fi

