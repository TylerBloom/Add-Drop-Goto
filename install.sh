#! /bin/bash

if [ ! -e ~/.waypoints ]
then
	mkdir ~/.waypoints # Creates a directory to store waypoints.txt
fi

if [ ! -e ~/.waypoints/waypoint.txt ] # Creates waypoints if it does not exist
then
	touch waypoints.txt
	cp waypoints.txt adg ~/.waypoints
fi

base=$(pwd)
cd ~
home=$(pwd)
cd $base
echo "wayPointsLocation=""'"$home"/.waypoints'" >> waypoints.txt
echo "Adding completion file to /etc/bash_completion.d/"
sudo cp ADG /etc/bash_completion.d/adg

if [[ $(cat ~/.bashrc) != *'source ~/.waypoints/waypoints.txt'* ]]
then
	echo 'source ~/.waypoints/waypoints.txt' >> ~/.bashrc # Addes waypoints upon startup
fi

if [[ $(cat ~/.bashrc) != *'PATH=''"''${PATH}:~/.waypoints''"'* ]]
then
	echo 'PATH=''"''${PATH}:~/.waypoints''"' >> ~/.bashrc # Addes the new directory to the PATH
fi

if [[ $(cat ~/.bashrc) != *"alias adg='. adg'"* ]]
then
	echo "alias adg='. adg'" >> ~/.bashrc # Addes required aliases
fi

echo "Configure complete. Please source ~/.bashrc and /etc/bash_completion for every terminal open or close it and open it again."
echo "This only needs to happen once. Then you're good to go!"
