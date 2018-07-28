command_run_help() {
  cat <<EOF

Usage:	${CURRENT_SCRIPT_NAME} run [-i | --interactive] PROGRAM [PROGRAM ARGS]

Run an program

EOF
}

command_run_one() {
  echo "Running ${PROGRAM_NAME}..."

  local RUN_INTERACTIVE="$1"
  shift

  local COMMON_FILE=$(get_common_file ${PROGRAM_NAME})

  if [ -z "${COMMON_FILE}" ]; then
    RETURN_CODE=128
    return
  fi

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

    if [ "${APPLICATION_INTERACTIVE}" = "true" ] || [ "${RUN_INTERACTIVE}" = "true" ]; then
      EXTRA_RUN_ARG="${EXTRA_RUN_ARG} -it"
    else
      EXTRA_RUN_ARG="${EXTRA_RUN_ARG} -d"
    fi

    docker run -v /tmp/.X11-unix/:/tmp/.X11-unix/ \
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

command_run() {
  local RUN_INTERACTIVE="false"

  case ${PROGRAM_NAME} in
    -h | --help        ) command_run_help; return;;
    -i | --interactive )
                         RUN_INTERACTIVE="true"
                         PROGRAM_NAME="$1"
                         shift;;
  esac

  command_run_one "${RUN_INTERACTIVE}" $@
}

COMMAND_DESCRIPTION="Run container"
COMMAND_MIN_ARGS=1
COMMAND_MAX_ARGS=-1
