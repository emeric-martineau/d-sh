#!/bin/sh

DESCRIPTION="Test build tgz application with force"
COMMAND="build"
ARGS="--force xeyestgz"
TEST_FUNCTION="uzumrv2teyv3jnby"

uzumrv2teyv3jnby_before() {
  . "../resources/program/xeyestgz.sh"

  DOCKER_IMAGE_CREATE_AT_BEFORE=$(docker images --format "{{.CreatedAt}}" "${APPLICATION_IMAGE_DOCKER}")

  echo "${DOCKER_IMAGE_CREATE_AT_BEFORE}" >> ${LOG_FILE}
}

# First argument is return of d.sh
uzumrv2teyv3jnby() {
  if [ $1 -eq 0 ]; then
    . "../resources/program/xeyestgz.sh"

    DOCKER_IMAGE_CREATE_AT_AFTER=$(docker images --format "{{.CreatedAt}}" "${APPLICATION_IMAGE_DOCKER}")

    echo "${DOCKER_IMAGE_CREATE_AT_AFTER}" >> ${LOG_FILE}

    if [ "${DOCKER_IMAGE_CREATE_AT_BEFORE}" = "${DOCKER_IMAGE_CREATE_AT_AFTER}" ]; then
      return 1
    fi
  fi

  return $1
}
