#! /bin/bash
 
if [ -e waypoint.txt ] # Creates waypoints if it does not exist
then
	rm waypoints.txt
fi

touch waypoints.txt
base=$(pwd)
cd ~
home=$(pwd)
cd $base
echo "wayPointsLocation=""'"$home"/.waypoints'" >> waypoints.txt

mkdir ~/.waypoints # Creates a directory to store waypoints.txt
cp waypoints.txt adg ~/.waypoints
echo "Adding completion file to /etc/bash_completion.d/"
sudo cp ADG /etc/bash_completion.d/adg
echo 'source ~/.waypoints/waypoints.txt' >> ~/.bashrc # Addes waypoints upon startup
echo 'PATH=''"''${PATH}:~/.waypoints''"' >> ~/.bashrc # Addes the new directory to the PATH

# Addes required aliases
echo "alias adg='. adg'" >> ~/.bashrc

echo "Configure complete. Please source ~/.bashrc and /etc/bash_completion for every terminal open or close it and open it again."
echo "This only needs to happen once. Then you're good to go!"
