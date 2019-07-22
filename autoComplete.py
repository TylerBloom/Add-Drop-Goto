#! /usr/bin/env python

import os 
import glob
import configparser
from sys import argv

waypoints = configparser.ConfigParser()
waypoints.read( os.environ["HOME"] + "/.waypoints/waypoints.ini" )


retStr = ""

if len(argv) == 2 and not "-" in argv[1]: # adg w/o arguments should require a flag, a locale or site, or a nearby directory
  print (" ".join(waypoints["locales"]) + " " + " ".join(waypoints["sites"]) + " " + " ".join( [ thing if not "." in thing and os.path.isdir(thing) else "" for thing in os.listdir("./") ] )).strip()
elif "-l" == argv[2]:
  print ""
elif "-a" == argv[2]:
  retList = []
  for thing in glob.glob( "*" if argv[-1] == "-a" else argv[-1] + "*/" ):
    if os.path.isdir(thing):
      retList.append( thing )
  print " ".join(retList).strip() 
elif "-d" == argv[2]:
  print (" ".join(waypoints["locales"]) + " " + " ".join(waypoints["sites"])).strip()
else:
  currEntry = argv[-1].split("/")
  if len(currEntry) == 1:
    retStr += "/ ".join(waypoints["locales"]) + "/ " 
    retStr += "/ ".join(waypoints["sites"])  + "/ " 
    retStr += "/ ".join( [ thing if not "." in thing and os.path.isdir(thing) else "" for thing in os.listdir("./") ] )
    print retStr.strip()
  else:
    listStr = ""
    for thing in currEntry:
      if thing in waypoints["locales"]:
        listStr += waypoints["locales"][thing] + "/"
      elif thing in waypoints["sites"]:
        listStr += waypoints["sites"][thing] + "/"
      elif thing != currEntry[-1]:
        listStr += thing + "/"
    prntStr = ""
    for thing in os.listdir(listStr):
      if not "." in thing and os.path.isdir(listStr + thing):
        prntStr += "/".join(currEntry[:-1]) + "/" + thing + "/ "
    print prntStr.strip()
      


