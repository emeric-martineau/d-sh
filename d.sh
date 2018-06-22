#!/bin/sh

CURRENT_SCRIPT_NAME="d.sh"
CURRENT_SCRIPT_VERSION="1.0.0"
REALPATH="$(realpath $0)"
BASEDIR="$(dirname ${REALPATH})"
# Return code of script
RETURN_CODE=0

help() {
  cat <<EOF

Usage: ${CURRENT_SCRIPT_NAME} COMMAND

A tool to container for all your life

Options:
  -c, --check              List missing container image
  -h, --help               Print this current help
  -l, --list               List all program avaible
  -v, --version            Print version information and quit

Commands:
  build    Build container image
  delete   Delete image
  run      Run container

EOF
}

error_command() {
  echo "${CURRENT_SCRIPT_NAME}: '$1' is not a d.sh command." >&2
  echo "See '${CURRENT_SCRIPT_NAME} --help'" >&2
}

error_command_missing_param() {
  echo "\"${CURRENT_SCRIPT_NAME} $1\" requires argument." >&2
  echo "See '${CURRENT_SCRIPT_NAME} $1 --help'." >&2
}

version() {
  echo "${CURRENT_SCRIPT_NAME} version ${CURRENT_SCRIPT_VERSION}"
  echo "Copyleft Emeric MARTINEAU (c) 2018"
}

check_docker() {
  IS_DOCKER_INSTALLED=$(docker --version 2>/dev/null)

  if [ -z "${IS_DOCKER_INSTALLED}" ]; then
    echo "Docker is not installed or your are not allowed to run it." >&2
    exit 255
  fi
}

exec_command() {
  check_docker

  case "${COMMAND}" in
    build          ) . ${BASEDIR}/scripts/command/command-build.sh;;
    delete         ) . ${BASEDIR}/scripts/command/command-delete.sh;;
    run            ) . ${BASEDIR}/scripts/command/command-run.sh $@;;
    -c | --check   ) . ${BASEDIR}/scripts/command/command-check.sh;;
    -l | --list    ) . ${BASEDIR}/scripts/command/command-list.sh;;
    *              ) error_command "${COMMAND}";
                     RETURN_CODE=2;;
  esac
}

if [ $# -gt 0 ]; then
  COMMAND="$1"
  PROGRAM_NAME="$2"

  # if "$1" not start by "-", we need check parameter if set
  FIRST_COMMAND_CHAR=$(echo "${COMMAND}" | cut -c 1)

  if [ ! "${FIRST_COMMAND_CHAR}" = "-" ] && [ $# -lt 2 ]; then
    error_command_missing_param ${COMMAND}
    RETURN_CODE=1
  else
    shift;

    if [ $# -gt 0 ]; then
      shift
    fi

    COMMON_FILE="${BASEDIR}/program/${PROGRAM_NAME}.sh"

    case "${COMMAND}" in
      -h | --help    ) help;;
      -v | --version ) version;;
      *              ) exec_command "$@";;
    esac
  fi
else
  help
fi

exit ${RETURN_CODE}
