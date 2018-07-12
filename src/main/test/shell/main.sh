#!/bin/sh

REALPATH="$(realpath $0)"
BASEDIR="$(dirname ${REALPATH})"
LOG_FILE="${BASEDIR}/test.log"

HTTP_SERVER_IMAGE_NAME='httpd:2.4'
FOLDER_TO_TEST='d-sh'
IMAGE_BASE_NAME='d-base-image:d-sh-test-1.0'

RESOURCES_FOLDER="${PWD}/../resources"
SOURCES_FOLDER="${PWD}/../../shell/"

# Display log in screen (stdout) and file
log() {
  echo "$1" | tee ${LOG_FILE}
}

# Display log in screen (stderr) and file
log_error() {
  echo "$1" >&2 | tee ${LOG_FILE}
}

# Function to display error in test
error() {
  log "      Message: $1"
}

# Reset variable of test
reset_test() {
  DESCRIPTION=""
  COMMAND=""
  ARGS=""
  TEST_FUNCTION=""
}

# Run before function and run command
run_command() {
  log "  - ${DESCRIPTION}"

  BEFORE_FUNCTION="${TEST_FUNCTION}_before"

  # If before function doesn't exists
  "${BEFORE_FUNCTION}" 2>/dev/null || true

  D_SH_OUTPUT=$(${FOLDER_TO_TEST}/d.sh ${COMMAND} ${ARGS} 2>&1 | tee ${LOG_FILE})
}

log "Running D-SH tests. Full log in ${LOG_FILE}"
log ""

# Run static server
log "Run HttpStaticServer..."

HTTP_SERVER_CONTAINER_ID=$(docker run -d --rm -v "${RESOURCES_FOLDER}/download:/usr/local/apache2/htdocs/" -p 23333:80 ${HTTP_SERVER_IMAGE_NAME})

if [ $? -ne 0 ]; then
  log_error "Fail. Cannot run HttpStaticServer"
  exit 255
fi

# Copy all file from source to test
log "Preparing sources..."
rm -rf "${FOLDER_TO_TEST}"
mkdir -p "${FOLDER_TO_TEST}" >> ${LOG_FILE} 2>&1
cp "${SOURCES_FOLDER}/d.sh" "${FOLDER_TO_TEST}/" >> ${LOG_FILE} 2>&1
cp -r "${SOURCES_FOLDER}/scripts" "${FOLDER_TO_TEST}/" >> ${LOG_FILE} 2>&1
cp -r "${RESOURCES_FOLDER}/program" "${FOLDER_TO_TEST}/" >> ${LOG_FILE} 2>&1

# Change base image name in Dockerfile to don't delete existing
sed -i.bak -r "s/FROM .+/FROM ${IMAGE_BASE_NAME}/" ${FOLDER_TO_TEST}/scripts/Dockerfile.from-* >> ${LOG_FILE} 2>&1

log "Running test.."

TOTAL_TEST=0
FAIL_TEST=0

for currentTestScript in $(find tests/ -name '*.sh' | sort); do
  reset_test

  . ${currentTestScript}

  run_command

  ${TEST_FUNCTION} $? "${D_SH_OUTPUT}"

  if [ $? -eq 0 ]; then
    log "    > OK"
  else
    log "    > Fail"
    FAIL_TEST=$(expr ${FAIL_TEST} + 1)
  fi

  TOTAL_TEST=$(expr ${TOTAL_TEST} + 1)
done

log ""
log "  ${TOTAL_TEST} tests, ${FAIL_TEST} failures"

# Delete test source
rm -rf ${FOLDER_TO_TEST} >> ${LOG_FILE} 2>&1

# Stop container
docker container kill ${HTTP_SERVER_CONTAINER_ID} >> ${LOG_FILE} 2>&1

log "Kill all test container"
docker container kill $(docker container list | grep ':d-sh-test-' | cut -d ' ' -f 1) >> ${LOG_FILE} 2>&1

log "Delete all test image"
docker image rm --force $(docker image list --filter=reference='*:d-sh-test-*' -q) >> ${LOG_FILE} 2>&1
docker system prune --force >> ${LOG_FILE} 2>&1
