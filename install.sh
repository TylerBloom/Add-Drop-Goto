#! /bin/bash

[ ! -e ~/.config/adg ] && mkdir ~/.config/adg # Creates a directory to store waypoints.txt

[ ! -e ~/.waypoints/waypoints.ini ] && cp waypoints.ini ~/.config/adg/

# TODO: The adg bash script needs to go somewhere... but where? This will depend on the shell...

# TODO: This should be up to the user... different shells need different
# configurations... Or I could support them...
[[ $(cat ~/.bashrc) != *"alias adg='. adg'"* ]] && echo "alias adg='. adg'" >> ~/.bashrc # Addes required aliases
