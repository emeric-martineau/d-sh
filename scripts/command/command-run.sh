command_run_help() {
  cat <<EOF

Usage:	${CURRENT_SCRIPT_NAME} run PROGRAM [PROGRAM ARGS]

Run an program

EOF
}

command_run() {
  echo "Running ${PROGRAM_NAME}..."

  . "${COMMON_FILE}"

  # Check if image exists
  NUMBER_IMAGE_EXISTS=$(docker image list ${IMAGE_DOCKER} | wc -l)

  if [ "${NUMBER_IMAGE_EXISTS}" -lt 2 ]; then
    echo "Image for program ${PROGRAM_NAME} not found." >&2
    echo "" >&2
    echo "Build it before with:" >&2
    echo "  ${CURRENT_SCRIPT_NAME} build ${PROGRAM_NAME}" >&2
  else
    echo "Create container ${CONTAINER_NAME}"

    UID=$(id -u ${USER})
    GID=$(id -g ${USER})

    docker run -d -v /tmp/.X11-unix/:/tmp/.X11-unix/ \
                -v /dev/shm:/dev/shm \
                -v ${HOME}:/home/${USER} \
                -e DISPLAY \
                -e USERNAME_TO_RUN=${USER} \
                -e USERNAME_TO_RUN_GID=${GID} \
                -e USERNAME_TO_RUN_UID=${UID} \
                --rm \
                ${IMAGE_DOCKER} ${COMMAND_LINE} $@
  fi
}

case ${PROGRAM_NAME} in
  -h | --help    ) command_run_help;;
  *              ) command_run $@;;
esac
