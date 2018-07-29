#!/bin/sh

DESCRIPTION="Test build tgz application (force no redownload by commandline option)"
COMMAND="build"
ARGS="-s xeyestgz"
TEST_FUNCTION="t3h86fxn8f7czg26"

t3h86fxn8f7czg26_before() {
  # Get time stamp of file use by static server
  TIMESTAMP_OF_RESSOURCE_FILE=$(stat -c %Y "${RESOURCES_FOLDER}/download/xeyes.tgz")
  # Caculate a new time stamp
  TIMESTAMP_OF_DOWNLOAD_FILE=$(expr ${TIMESTAMP_OF_RESSOURCE_FILE} - 50000)
  NEW_DATE_OF_DOWNLOAD_FILE=$(date -ud "@1532382665" "+%Y%m%d%H%M.%S")

  # Change timestamp to force redownload
  touch -t "${NEW_DATE_OF_DOWNLOAD_FILE}" "${FOLDER_TO_TEST}/download/xeyes.tgz"

  LAST_MODIFY_1=$(stat -c %y "${FOLDER_TO_TEST}/download/xeyes.tgz")
}

# First argument is return of d.sh
t3h86fxn8f7czg26() {
  if [ $1 -eq 0 ]; then
    LAST_MODIFY_2=$(stat -c %y "${FOLDER_TO_TEST}/download/xeyes.tgz")

    [ "${LAST_MODIFY_1}" = "${LAST_MODIFY_2}" ]

    return $?
  fi

  return $1
}
