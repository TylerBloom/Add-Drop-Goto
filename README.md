ABOUT:<br />
Adg, standing for add-drop-goto, is a general utility useful for navigating to and between<br />
directories that are commonly accessed, have long path names, and navigating hierachical structures.<br />
It does this by storing given directories under an alias, called a waypoint.<br/><br />
CONFIGURING:<br />
Running ./configADG. This will moving files where they need to go and add the required things to your .bashrc<br />
After running configADG, you'll need to source your ~/.bashrc and /etc/bash_completion for any open terminals. <br />
<br />
HOW TO USE:<br />
There are five option to pass to adg, -a, -d, -g, -v, and -l. <br />
-a: <br />
This option is used to add directories as waypoints. Since there can only be one waypoint of a given name use '-a --f' to change the directory to which a waypoint refers.<br />
Example: adg -a foobar /home/JohnDoe/foo/bar/<br />
This will add a waypoint called "foobar" that refers to the given directory.<br />
The order in which you pass directories and new waypoints doesn't matter, i.e. "adg -a ./ foobar" does the same thing as "adg -a foobar ./".<br /><br />
-d: <br />
This option is used to remove a waypoint. <br />
Example: adg -d foobar <br />
This will remove "foobar" from the avaliable waypoints.<br /><br />
-g:<br />
This option is what moves you between waypoints. <br />
Example: adg -g foobar <br />
This will changing your current directory to the directory to which "foobar" refers.<br /><br />
-l:<br />
This option, simply, lets you look at what waypoints are current available. This can be used in conjuction with other options.<br />
Examples: adg -l, adg -l -a foobar ./ <br />
The first example will just list the current waypoints. The second will list then add the new waypoint.<br /><br />
-v:<br />
This option will give verbose output. Allowing you to see what is happening, when it happens.<br />
Example: adg -a foobar \~/foo/bar/ -v <br />
This will output the following:<br />
Verbose option found. Additional read-out will appear<br />
Current waypoints:<br />
<br />
Adding /home/JohnDoe/foo/bar as 'foobar'<br />
Current waypoints:<br />
'foobar' as /home/JohnDoe/foo/bar <br /><br />
NOTES:<br />
The options -l and -v can be put in anywhere, i.e. before or after any other options. This script it also tab completable.<br />
If you type "adg -g foo" then press tab, it will auto-complete to "adg -g foobar".<br />
During configuration, a directory called "\~/.waypoints" will be created to store the adg script and the storage file waypoints.txt.<br />
These can be moved to anywhere you wish but are required to be in your global PATH. Waypoints will figure out that it has been moved and adjust accordingly.<br />
Lastly, this works across terminals, so you can add a waypoint in one terminal and switch to another and goto that waypoint from the new terminal.
