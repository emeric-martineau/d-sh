#!/bin/sh

# This file test build base image
DESCRIPTION="Test build base image"
COMMAND="build"
ARGS="-b"
TEST_FUNCTION="x8fadxnwtg2peng9"

# First argument is return of d.sh
x8fadxnwtg2peng9() {
  if [ $1 -eq 0 ]; then
    # IMAGE_BASE_NAME => from main script
    local NUMBER_IMAGE_EXISTS=$(docker image list ${IMAGE_BASE_NAME} | wc -l)

    if [ "${NUMBER_IMAGE_EXISTS}" -eq 2 ]; then
      return 0
    fi

    error "base image are not build"
    return 1
  else
    error "d.sh command fail"
    return 1
  fi
}
