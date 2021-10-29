#! /bin/bash

[ ! -e ~/.config ] && mkdir ~/.config # Creates a config directory if it doesn't exists

[ ! -e ~/.config/adg ] && mkdir ~/.config/adg # Creates a directory to store waypoints.txt

cp waypoints.json ~/.config/adg

# TODO: In the future, there will be a pre-compile release of adg on Github. For now, it needs to be compiled
cargo build --release
cp target/release/adg ~/.config/adg/

# TODO: The adg bash script needs to go somewhere... but where? This will depend on the shell...
echo "Place the shell script wrapper for your shell somewhere in your PATH."
echo 'Alias that script to ". adg" so it is sourced when you run it.'

