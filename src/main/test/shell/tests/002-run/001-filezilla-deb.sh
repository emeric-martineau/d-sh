#!/bin/sh

# This file test build base image
DESCRIPTION="Test run deb application"
COMMAND="run"
ARGS="filezilladeb"
TEST_FUNCTION="c5ufm5rp5uf3ntbz"

APPLICATION_IMAGE_DOCKER="run-filezilla:d-sh-test-latest-deb"

# First argument is return of d.sh
c5ufm5rp5uf3ntbz() {
  if [ $1 -eq 0 ]; then
    RUNNING_CONTAINER_ID=$(docker container list --filter="ancestor=${APPLICATION_IMAGE_DOCKER}" -q)

    if [ -n "${RUNNING_CONTAINER_ID}" ]; then
      IS_RUNNING=$(docker container inspect --format='{{.State.Running}}' "${RUNNING_CONTAINER_ID}")

      if [ "${IS_RUNNING}" = "true" ]; then
        return 0
      fi
    fi

    return 1
  fi

  return $1
}
