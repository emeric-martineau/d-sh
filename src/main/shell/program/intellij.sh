#!/bin/sh

local INTELLIJ_VERSION="2018.1.6"

APPLICATION_DOWNLOADED_FILE_NAME='intellij.tar.gz'
APPLICATION_URL="https://download.jetbrains.com/idea/ideaIC-${INTELLIJ_VERSION}-no-jdk.tar.gz"
APPLICATION_IMAGE_DOCKER="run-intellij:${INTELLIJ_VERSION}"
APPLICATION_DEPENDENCIES="openjdk-8-jdk"

# -f : tell atom to not release bash session
APPLICATION_COMMAND_LINE="/opt/idea-IC-181.5540.7/bin/idea.sh"
