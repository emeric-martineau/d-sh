#!/bin/sh

DESCRIPTION="Test build tgz application (with build base if not found)"
COMMAND="build"
ARGS="xeyestgz"
TEST_FUNCTION="mrbvrt6r8pw9f2d2"

mrbvrt6r8pw9f2d2_before() {
  docker image rm -f "${IMAGE_BASE_NAME}"  >> ${LOG_FILE}
}

# First argument is return of d.sh
mrbvrt6r8pw9f2d2() {
  return $1
}
