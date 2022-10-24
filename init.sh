#!/bin/bash

# This script is designed to aid in getting the correct packages for development.

# To run, you need to provide the OS information.

if test -z "$1"
then
  opersys="debian"
else
  opersys="$1"
fi

case "${opersys}" in
  "windows")
      echo "Running script for windows..."

      #Note: this command MUST be run in an elevated powershell instance.

      # TODO
      ;;
  "debian")
      sudo apt-get update #This line makes sure that debian packages are up to date
      echo "Running script for debian linux..."

      sudo apt-get install -y libsdl2-dev


      ;;
  "fedora")
      echo "Running script for fedora linux..."

      sudo dnf install SDL2-devel
      ;;
  "arch")
      echo "Running script for arch linux..."
      sudo pacman -S sdl2

      ;;
  "gentoo")
      echo "Running script for gentoo linux..."
      sudo emerge libsdl2
      ;;
  *)
      echo "Unknown operating system: ${opersys}"
      echo "Possible operating systems are: fedora debian windows"
      ;;
esac