#! /bin/bash

tions=':a:'

while getopts "$tions" currOpt; do
	echo ${currOpt[@]}
	echo ${OPTARG}
done
