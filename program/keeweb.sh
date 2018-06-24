#!/bin/sh

KEEWEB_VERSION_SHORT=1.6.3
KEEWEB_VERSION=v${KEEWEB_VERSION_SHORT}

APPLICATION_DOWNLOADED_FILE_NAME='keeweb.deb'
APPLICATION_URL="https://github.com/keeweb/keeweb/releases/download/${KEEWEB_VERSION}/KeeWeb-${KEEWEB_VERSION_SHORT}.linux.x64.deb"
APPLICATION_IMAGE_DOCKER="run-keeweb:${KEEWEB_VERSION}"
APPLICATION_DEPENDENCIES="libx11-xcb1 libxtst6 libxss1 libnss3 libasound2 libcanberra-gtk-module libappindicator1 libgconf2-4"
APPLICATION_COMMAND_LINE="/usr/bin/KeeWeb"
