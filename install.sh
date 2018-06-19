#! /bin/bash

[ ! -e ~/.waypoints ] && mkdir ~/.waypoints # Creates a directory to store waypoints.txt

if [ ! -e ~/.waypoints/waypoint.txt ] # Creates waypoints if it does not exist
then
	home=$(cd ~; pwd)
	echo "wayPointsLocation=""'"$home"/.waypoints'" > waypoints.txt
	cp waypoints.txt ~/.waypoints
fi

cp adg ~/.waypoints
echo "Adding completion file to /etc/bash_completion.d/"
sudo cp ADG /etc/bash_completion.d/adg

[[ $(cat ~/.bashrc) != *'source ~/.waypoints/waypoints.txt'* ]] && echo 'source ~/.waypoints/waypoints.txt' >> ~/.bashrc # Addes waypoints upon startup

[[ $(cat ~/.bashrc) != *'PATH=''"''${PATH}:~/.waypoints''"'* ]] && echo 'PATH=''"''${PATH}:~/.waypoints''"' >> ~/.bashrc # Addes the new directory to the PATH

[[ $(cat ~/.bashrc) != *"alias adg='. adg'"* ]] && echo "alias adg='. adg'" >> ~/.bashrc # Addes required aliases

echo "Configure complete. Please source ~/.bashrc and /etc/bash_completion for every terminal open or close it and open it again."
echo "This only needs to happen once. Then you're good to go!"
