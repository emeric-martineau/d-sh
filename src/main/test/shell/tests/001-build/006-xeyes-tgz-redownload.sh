#!/bin/sh

DESCRIPTION="Test build tgz application (no redownload)"
COMMAND="build"
ARGS="xeyestgz"
TEST_FUNCTION="p6yj3bmahmnhgcm7"

p6yj3bmahmnhgcm7_before() {
  LAST_MODIFY_1=$(stat -c %y ${FOLDER_TO_TEST}/download/xeyes.tgz | grep 'Modify:')
}

# First argument is return of d.sh
p6yj3bmahmnhgcm7() {
  if [ $1 -eq 0 ]; then
    LAST_MODIFY_2=$(stat -c %y ${FOLDER_TO_TEST}/download/xeyes.tgz | grep 'Modify:')

    [ "${LAST_MODIFY_1}" = "${LAST_MODIFY_2}" ]

    return $?
  fi

  return $1
}
