#! /bin/fish

set output (~/.config/adg/adg $argv)

if test -n "$output"
	switch $argv
	case "*list*"
		string join \n $output
	case "*"
		cd $output
	end
end
