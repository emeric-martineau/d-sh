command_check() {
  for prog in $(ls "${BASEDIR}/program"); do
    . "${BASEDIR}/program/${prog}"

    local NUMBER_IMAGE_EXISTS=$(docker image list ${APPLICATION_IMAGE_DOCKER} | wc -l)
    local STATUS

    if [ "${NUMBER_IMAGE_EXISTS}" -lt 2 ]; then
      STATUS="Build need"
    else
      STATUS="Build done"
    fi

    printf "%-34s%-34s%-13s\n" "${prog%.*}" "${APPLICATION_IMAGE_DOCKER}" "${STATUS}"
  done
}

COMMAND_DESCRIPTION="List missing container image"
COMMAND_MIN_ARGS=0
COMMAND_MAX_ARGS=0
