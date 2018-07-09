#!/bin/sh

REALPATH="$(realpath $0)"
BASEDIR="$(dirname ${REALPATH})"
LOG_FILE="${BASEDIR}/test.log"

HTTP_SERVER_IMAGE_NAME='httpd:2.4'
FOLDER_TO_TEST='d-sh'
IMAGE_BASE_NAME='d-base-image:d-sh-test-1.0'

RESOURCES_FOLDER="${PWD}/../resources"
SOURCES_FOLDER="${PWD}/../../shell/"

echo "Running D-SH tests. Full log in ${LOG_FILE}"

echo "" > ${LOG_FILE}

# Run static server
echo "Run HttpStaticServer..."
HTTP_SERVER_CONTAINER_ID=$(docker run -d --rm -v "${RESOURCES_FOLDER}/download:/usr/local/apache2/htdocs/" -p 23333:80 ${HTTP_SERVER_IMAGE_NAME})

if [ $? -ne 0 ]; then
  echo "Fail. Cannot run HttpStaticServer" >&2
  exit 255
fi

# Copy all file from source to test
echo "Preparing sources..."
rm -rf "${FOLDER_TO_TEST}"
mkdir -p "${FOLDER_TO_TEST}" >> ${LOG_FILE} 2>&1
cp "${SOURCES_FOLDER}/d.sh" "${FOLDER_TO_TEST}/" >> ${LOG_FILE} 2>&1
cp -r "${SOURCES_FOLDER}/scripts" "${FOLDER_TO_TEST}/" >> ${LOG_FILE} 2>&1
cp -r "${RESOURCES_FOLDER}/program" "${FOLDER_TO_TEST}/" >> ${LOG_FILE} 2>&1

# Change base image name in Dockerfile to don't delete existing
sed -i.bak -r "s/FROM .+/FROM ${IMAGE_BASE_NAME}/" ${FOLDER_TO_TEST}/scripts/Dockerfile.from-* >> ${LOG_FILE} 2>&1

# Function to display error in test
error() {
  echo "      Message: $1"
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
  echo "  - ${DESCRIPTION}" >> ${LOG_FILE}
  echo "  - ${DESCRIPTION}"

  BEFORE_FUNCTION="${TEST_FUNCTION}_before"

  # If before function doesn't exists
  "${BEFORE_FUNCTION}" 2>/dev/null || true

  ${FOLDER_TO_TEST}/d.sh ${COMMAND} ${ARGS}  >> ${LOG_FILE} 2>&1
}

echo "Running test.."

TOTAL_TEST=0
FAIL_TEST=0

for currentTestScript in $(find tests/ -name '*.sh' | sort); do
  reset_test

  . ${currentTestScript}

  run_command

  ${TEST_FUNCTION} $?

  if [ $? -eq 0 ]; then
    echo "    > OK" >> ${LOG_FILE}
    echo "    > OK"
  else
    echo "    > Fail" >> ${LOG_FILE}
    echo "    > Fail"
    FAIL_TEST=$(expr ${FAIL_TEST} + 1)
  fi

  TOTAL_TEST=$(expr ${TOTAL_TEST} + 1)
done

echo "" >> ${LOG_FILE}
echo "  ${TOTAL_TEST} tests, ${FAIL_TEST} failures" >> ${LOG_FILE}

echo ""
echo "  ${TOTAL_TEST} tests, ${FAIL_TEST} failures"

# Delete test source
rm -rf ${FOLDER_TO_TEST} >> ${LOG_FILE} 2>&1

# Stop container
docker container kill ${HTTP_SERVER_CONTAINER_ID} >> ${LOG_FILE} 2>&1

# Delete all test image
docker image rm $(docker image list --filter=reference='*:d-sh-test-*' -q) >> ${LOG_FILE} 2>&1
docker system prune --force >> ${LOG_FILE} 2>&1
