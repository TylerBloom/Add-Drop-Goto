#! /usr/bin/env python

import os 
import ConfigParser
from sys import argv

def isIn( key, place ):
  for thing in waypoints[key]:
    if waypoints[key][thing] == place:
      return True
  return False


# Adds a site or locale to waypoints
def addPlace ():
  if os.path.isdir( argv[2] ):
    os.chdir( argv[2] )
    waypoints["locales"][argv[3]] = os.getcwd()
  else:
    waypoints["sites"][argv[3]] = argv[2]


# Removes places from waypoints
def dropPlaces ():
  for i in argv[2:]:
    try:
      del( waypoints["locales"][i] )
    except KeyError:
      del( waypoints["sites"][i] )


# Determines where to go
def location ():
  prtVal = ""
  places = argv[1].split("/")

  if places[-1] == '':
    del(places[-1])
  
  if len(places) == 1 and places[0] in waypoints["sites"]:
    while not isIn( "locales", os.getcwd() ):
      os.chdir("../")
      if os.getcwd() == "/":
        break
    if os.path.isdir( os.getcwd() + "/" + waypoints["sites"][places[0]] ):
      prtVal = os.getcwd() + "/" + waypoints["sites"][places[0]]
  else:
    if places[0] in waypoints["locales"]:
      prtVal += waypoints["locales"][places[0]]
    elif places[0] in waypoints["sites"]:
      prtVal += waypoints["sites"][places[0]]
    else:
      prtVal += places[0]
    
    for i in range(1, len(places)):
      prtVal += "/"
      prtVal += waypoints["sites"][places[i]] if places[i] in waypoints["sites"] else places[i]
  
  print prtVal

# Prints a formatted list of the locales and sites
def listOut ():
  print "List of locales:"

  for i in waypoints["locales"]:
    print "  " + i + " as " + waypoints["locales"][i]
  print "\nList of sites:"

  for i in waypoints["sites"]:
    print "  " + i + " as " + waypoints["sites"][i]
  print ""

def printHelp ():
    print "adg is a script used for bookmarking locations in a file structure and for moving moving between those bookmarks."
    print "Each bookmark falls into one of two catagories, a 'locale' or a 'site'. These can be thought of as major and minor bookmarks."
    print "Locale are existing directories in a file structure while sites are common locations found around locales.\n"
    print "As an example, let '/foo' and '/bar' be directories found just below root and let these directories have a subdirectory called 'foobar.'"
    print "One could have a locale for '/foo', '/bar', '/foo/foobar', and '/bar/foobar'. Alternately, one could have a locale for '/foo' and '/bar' and a site for 'foobar'.\n\n" 
    print "Options: -a -d -l"
    print "\t -a : Adds a locale or site."
    print "\t      adg -a path_to_dir pseudonym_for_dir"
    print "\t      adg -a site_dir    pseudonym_for_dir"
    print "\t Note that a site can't be added if that dir exists in the in current working directory.\n"
    print "\t -d : Drops locales and site."
    print "\t      adg -a list_of_locales_and_sites_to_be_dropped"
    print "\t Note that multiple locales and sites can be dropped at a time.\n"
    print "\t -l : Lists existing locales and site."
    print "\t      adg -l "
    print "\t Note both the pseudonym and absolute path of locales and the pseudonym and names of sites are printed.\n"

# Opening a config parser and having it parse the waypoints.ini file that
# should be in a hidden directory in the home directory
waypoints = ConfigParser.ConfigParser()
waypoints.read( os.environ["HOME"] + "/.waypoints/waypoints.ini" )

# Reading the arguments then calling the appropriate function
if len(argv) == 1:
  printHelp()
elif argv[1] == "-h":
  printHelp()
elif argv[1] == "-a":
  addPlace()
elif argv[1] == "-d":
  dropPlaces()
elif argv[1] == "-l":
  listOut()
else:
  location()


# Writing the config tree back to the waypoints.ini file
storeFile = open( os.environ["HOME"] + "/.waypoints/waypoints.ini", "w" )
waypoints.write( storeFile )
storeFile.close()



