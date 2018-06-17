#!/bin/sh

ATOM_VERSION=v1.27.2

DOWNLOADED_FILE_NAME='atom.deb'
URL="https://github.com/atom/atom/releases/download/${ATOM_VERSION}/atom-amd64.deb"
IMAGE_DOCKER="run-atom:${ATOM_VERSION}"
EXTRA_BUILD_ARG=""
DEPENDENCIES="ca-certificates curl fakeroot gconf2 gconf-service git gvfs-bin libasound2 libcap2 libgconf-2-4 libgtk2.0-0 libnotify4 libnss3 libxkbfile1 libxss1 libxtst6 libgl1-mesa-glx libgl1-mesa-dri python xdg-utils libcanberra-gtk3-module libgtk2.0-0 libudev1 libx11-xcb1 libsecret-1-0 gir1.2-gnomekeyring-1.0"

# -f : tell atom to not release bash session
COMMAND_LINE="/usr/bin/atom -f"
