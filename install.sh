#! /bin/bash

[ ! -e ~/.waypoints ] && mkdir ~/.config/adg # Creates a directory to store waypoints.txt

[ ! -e ~/.waypoints/waypoints.ini ] && echo "[locales]" > ~/.waypoints/waypoints.ini && echo "[sites]" >> ~/.waypoints/waypoints.ini

cp adg ~/.waypoints
cp adg.py ~/.waypoints
cp autoComplete.py ~/.waypoints

echo "Adding completion file to /etc/bash_completion.d/"

sudo cp ADG /etc/bash_completion.d/adg


[[ $(cat ~/.bashrc) != *'PATH=''"''${PATH}:~/.waypoints''"'* ]] && echo 'PATH=''"''${PATH}:~/.waypoints''"' >> ~/.bashrc # Addes the new directory to the PATH

[[ $(cat ~/.bashrc) != *"alias adg='. adg'"* ]] && echo "alias adg='. adg'" >> ~/.bashrc # Addes required aliases

echo "Configure complete. Please source ~/.bashrc and /etc/bash_completion for every terminal open or close it and open it again."
echo "This only needs to happen once. Then you're good to go!"
