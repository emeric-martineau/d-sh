#!/bin/sh

# This file test build base image
DESCRIPTION="Test check all applications"
COMMAND="check"
ARGS=""
TEST_FUNCTION="rrc8er9de3gt3y62"

rrc8er9de3gt3y62_before() {
  . "../resources/program/xeyestgz.sh"

  RUNNING_CONTAINER_ID=$(docker container list --filter="ancestor=${APPLICATION_IMAGE_DOCKER}" -q)

  if [ -n "${RUNNING_CONTAINER_ID}" ]; then
    docker container kill "${RUNNING_CONTAINER_ID}" >> ${LOG_FILE}
  fi

  docker image rm "${APPLICATION_IMAGE_DOCKER}" >> ${LOG_FILE}
}

# First argument is return of d.sh
rrc8er9de3gt3y62() {
  local OUTPUT_EXPECTED_1="filezilladeb                      run-filezilla:d-sh-test-latest-debBuild done   "
  local OUTPUT_EXPECTED_2="xeyespackage                      run-xeyes:d-sh-test-latest-packageBuild done   "
  local OUTPUT_EXPECTED_3="xeyestargz                        run-xeyes:d-sh-test-latest-tar-gz Build done   "
  local OUTPUT_EXPECTED_4="xeyestgz                          run-xeyes:d-sh-test-latest-tgz    Build need   "
  local COUNTER=1

  local OUTPUT_ACTUAL="$(echo "$2" | tr '\n' '|')"

  if [ $1 -eq 0 ]; then
    # Ok, Dash sucks
    local OLD_IFS="${IFS}"
    IFS='|'

    for line in ${OUTPUT_ACTUAL}; do
      local var="OUTPUT_EXPECTED_${COUNTER}"

      local expected=$(eval echo \$${var})

      if [ ! "${line}" = "${expected}" ]; then
        return 0
      fi

      local COUNTER=$(expr ${COUNTER} + 1)
    done

    IFS="${OLD_IFS}"

    return 0
  fi

  return $1
}
