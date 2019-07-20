#! /usr/bin/env python

import os 
import configparser
from sys import argv

# Adds a site or locale to waypoints
def addPlace ():
  if os.path.isdir( argv[3] ):
    os.chdir( argv[3] )
    waypoints["locales"][argv[2]] = os.environ["PWD"]
  else:
    waypoints["sites"][argv[2]] = argv[3]


# Removes places from waypoints
def dropPlaces ():
  for i in argv[1:]:
    try:
      del( waypoints["locales"][i] )
    except KeyError:
      del( waypoints["sites"][i] )


# Determines where to go
def location ():
  prtVal = ""
  places = argv[1].split("/")
  
  prtVal += waypoints["locales"][places[0]] if places[0] in waypoints["locales"] else places[0]
  
  for i in range(1, len(places)):
    prtVal += "/"
    prtVal += waypoints["sites"][places[i]] if places[i] in waypoints["site"] else places[i]
  
  print prtVal

# Prints a formatted list of the locales and sites
def listOut ():
  print "List of locales:"

  for i in waypoints["locales"]:
    print "  " + i + " as " + waypoints["locales"][i]
  
  print "\nList of sites:"

  for i in waypoints["sites"]:
    print "  " + i + " as " + waypoints["sites"][i]


waypoints = configparser.ConfigParser()
waypoints.read( os.environ["HOME"] + "/.waypoints/waypoints.ini" )


if argv[1] == "-a":
  addPlace()
elif argv[1] == "-d":
  dropPlaces()
elif argv[1] == "-l":
  listOut()
else:
  location()


storeFile = open( os.environ["HOME"] + "/.waypoints/waypoints.ini", "w" )
waypoints.write( storeFile )
storeFile.close()

bashFile = open( os.environ["HOME"] + "/.waypoints/waypoints.ini", "w" )
bashFile.write( "locales=( " )
for i in waypoints["locales"]:
  bashFile.write( i + " " )
bashFile.write( ")\nsites=( " )
for i in waypoints["sites"]:
  bashFile.write( i + " " )
bashFile.write( ")" )
bashFile.close()


