#!/bin/sh

REALPATH="$(realpath $0)"
BASEDIR="$(dirname ${REALPATH})"
LOG_FILE="${BASEDIR}/test.log"

HTTP_SERVER_IMAGE_NAME='d-sh-test-static-server:latest'
FOLDER_TO_TEST='d-sh'
IMAGE_BASE_NAME='d-base-image:d-sh-test-1.0'

RESOURCES_FOLDER="${PWD}/src/resources"

echo "Running D-SH tests. Full log in ${LOG_FILE}"

echo "" > ${LOG_FILE}

# Check if image exists
NUMBER_IMAGE_EXISTS=$(docker image list ${HTTP_SERVER_IMAGE_NAME} | wc -l)

if [ "${NUMBER_IMAGE_EXISTS}" -lt 2 ]; then
  # Build image for static server
  echo "Build HttpStaticServer image..."
  docker build . -t ${HTTP_SERVER_IMAGE_NAME} -f httpstaticserver/Dockerfile.httpserver >> ${LOG_FILE} 2>&1

  if [ $? -ne 0 ]; then
    echo "Fail. Cannot build HttpStaticServer" >&2
    exit 255
  fi
fi

# Run static server
echo "Run HttpStaticServer..."
HTTP_SERVER_CONTAINER_ID=$(docker run -d --rm -v ${RESOURCES_FOLDER}/download:/download -p 23333:23333 ${HTTP_SERVER_IMAGE_NAME} static-http -r /download -p 23333 -n 0.0.0.0)

if [ $? -ne 0 ]; then
  echo "Fail. Cannot run HttpStaticServer" >&2
  exit 255
fi

# Copy all file from source to test
echo "Preparing sources..."
mkdir -p ${FOLDER_TO_TEST} >> ${LOG_FILE} 2>&1
cp ../src/d.sh ${FOLDER_TO_TEST}/ >> ${LOG_FILE} 2>&1
cp -r ../src/scripts ${FOLDER_TO_TEST}/ >> ${LOG_FILE} 2>&1
cp -r ${RESOURCES_FOLDER}/program ${FOLDER_TO_TEST}/ >> ${LOG_FILE} 2>&1

# Change base image name in Dockerfile to don't delete existing
sed -i.bak -r "s/FROM .+/FROM ${IMAGE_BASE_NAME}/" ${FOLDER_TO_TEST}/scripts/Dockerfile.from-* >> ${LOG_FILE} 2>&1

# Function to display error in test
error() {
  echo "      Message: $1"
}

echo "Running test.."

TOTAL_TEST=0
FAIL_TEST=0

for currentTestScript in $(find src/main/test -name '*.sh'); do
  . ${currentTestScript}

  echo "  - ${DESCRIPTION}" >> ${LOG_FILE}
  echo "  - ${DESCRIPTION}"

  ${FOLDER_TO_TEST}/d.sh ${COMMAND} ${ARGS}  >> ${LOG_FILE} 2>&1

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

# Delete image for static server
#docker image rm ${HTTP_SERVER_IMAGE_NAME}
