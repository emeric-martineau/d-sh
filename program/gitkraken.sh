#!/bin/sh

DOWNLOADED_FILE_NAME='gitkraken.deb'
URL="https://release.gitkraken.com/linux/gitkraken-amd64.deb"
IMAGE_DOCKER="run-gitkraken:latest"
EXTRA_BUILD_ARG=""
DEPENDENCIES="ca-certificates curl fakeroot libasound2 libxss1 libcanberra-gtk-module libcurl4-gnutls-dev libgnome-keyring-common libgnome-keyring-dev git gconf2 gconf-service libgtk2.0-0 libudev1 libgcrypt20 libnotify4 libxtst6 libnss3 libxkbfile1 python gvfs-bin xdg-utils libgnome-keyring0 gir1.2-gnomekeyring-1.0 distro-info-data lsb-release"
COMMAND_LINE="/usr/bin/gitkraken"
