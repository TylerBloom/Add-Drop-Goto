#! /usr/bin/python

from sys import argv

args = " ".join(argv[1:len(argv)])
print args
args = args.replace("--", "-")
print args
args = args.split("-")[1:len(args)]
print args




