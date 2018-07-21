#!/bin/sh

DESCRIPTION="Test build missing application"
COMMAND="build"
ARGS="--missing"
TEST_FUNCTION="dds5t8zcwr3as2sk"

dds5t8zcwr3as2sk_remove_image() {
  . "../resources/program/$1.sh"

  RUNNING_CONTAINER_ID=$(docker container list --filter="ancestor=${APPLICATION_IMAGE_DOCKER}" -q)

  if [ -n "${RUNNING_CONTAINER_ID}" ]; then
    docker container kill "${RUNNING_CONTAINER_ID}" >> ${LOG_FILE}
  fi

  docker image rm "${APPLICATION_IMAGE_DOCKER}" >> ${LOG_FILE}
}

dds5t8zcwr3as2sk_test_if_image_exists() {
  . "../resources/program/$1.sh"

  DOCKER_IMAGE_CREATE=$(docker images --format "{{.CreatedAt}}" "${APPLICATION_IMAGE_DOCKER}")

  if [ -z "${DOCKER_IMAGE_CREATE}" ] ; then
    return 1
  fi

  return 0
}

dds5t8zcwr3as2sk_before() {
  dds5t8zcwr3as2sk_remove_image "xeyestgz"
  dds5t8zcwr3as2sk_remove_image "xeyestargz"
}

# First argument is return of d.sh
dds5t8zcwr3as2sk() {
  if [ $1 -eq 0 ]; then
    dds5t8zcwr3as2sk_test_if_image_exists "xeyestgz" && dds5t8zcwr3as2sk_test_if_image_exists "xeyestargz" && return 0

    return 1
  fi

  return $1
}
