#!/bin/sh

KEEWEB_VERSION_SHORT=1.6.3
KEEWEB_VERSION=v${KEEWEB_VERSION_SHORT}

DOWNLOADED_FILE_NAME='keeweb.deb'
URL="https://github.com/keeweb/keeweb/releases/download/${KEEWEB_VERSION}/KeeWeb-${KEEWEB_VERSION_SHORT}.linux.x64.deb"
IMAGE_DOCKER="run-keeweb:${KEEWEB_VERSION}"
EXTRA_BUILD_ARG=""
DEPENDENCIES="libx11-xcb1 libxtst6 libxss1 libnss3 libasound2 libcanberra-gtk-module libappindicator1 libgconf2-4"
COMMAND_LINE="/usr/bin/KeeWeb"
