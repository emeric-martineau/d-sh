#!/bin/sh

# This file test build base image
DESCRIPTION="Test list all applications"
COMMAND="list"
ARGS=""
TEST_FUNCTION="s4fe7gvngr6skn97"

# First argument is return of d.sh
# Second argument is output of d.sh
s4fe7gvngr6skn97() {
  OUTPUT_EXPECTED="bashapplicationinteractive
bash
filezilladeb
xeyespackage
xeyestarbz
xeyestargz
xeyestarxz
xeyestgznodl
xeyestgz"

  if [ $1 -eq 0 ]; then
    if [ "$2" = "${OUTPUT_EXPECTED}" ]; then
      return 0
    else
      return 1
    fi
  fi

  return $1
}
