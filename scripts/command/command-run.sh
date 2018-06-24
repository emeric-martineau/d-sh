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
  local NUMBER_IMAGE_EXISTS=$(docker image list ${APPLICATION_IMAGE_DOCKER} | wc -l)

  if [ "${NUMBER_IMAGE_EXISTS}" -lt 2 ]; then
    echo "Image for program ${PROGRAM_NAME} not found." >&2
    echo "" >&2
    echo "Build it before with:" >&2
    echo "  ${CURRENT_SCRIPT_NAME} build ${PROGRAM_NAME}" >&2
    RETURN_CODE=3
  else
    echo "Create container ${CONTAINER_NAME}"

    local UID=$(id -u ${USER})
    local GID=$(id -g ${USER})

    local EXTRA_RUN_ARG=""

    if [ "${APPLICATION_IPC_HOST}" = "true" ]; then
      EXTRA_RUN_ARG="${EXTRA_RUN_ARG} --ipc=host"
    fi

    docker run -d -v /tmp/.X11-unix/:/tmp/.X11-unix/ \
                -v /dev/shm:/dev/shm \
                -v ${HOME}:/home/${USER} \
                -e DISPLAY \
                -e USERNAME_TO_RUN=${USER} \
                -e USERNAME_TO_RUN_GID=${GID} \
                -e USERNAME_TO_RUN_UID=${UID} \
                ${EXTRA_RUN_ARG} \
                --rm \
                ${APPLICATION_IMAGE_DOCKER} ${APPLICATION_COMMAND_LINE} $@

    RETURN_CODE=$?
  fi
}

case ${PROGRAM_NAME} in
  -h | --help    ) command_run_help;;
  *              ) command_run $@;;
esac
