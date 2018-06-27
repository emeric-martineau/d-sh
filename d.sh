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
  -h, --help               Print this current help
  -v, --version            Print version information and quit

Commands:
EOF
  for commandFile in $(ls "${BASEDIR}/scripts/command/" | sort); do
    local cmdName=$(echo $commandFile | cut -f 1 -d . | cut -f 2 -d -)

    . "${BASEDIR}/scripts/command/command-${cmdName}.sh"

    printf "  %-9s%s\n" "${cmdName}" "${COMMAND_DESCRIPTION}"
  done
}

error_command() {
  echo "${CURRENT_SCRIPT_NAME}: '$1' is not a d.sh command." >&2
  echo "See '${CURRENT_SCRIPT_NAME} --help'" >&2
}

error_command_missing_param() {
  echo "\"${CURRENT_SCRIPT_NAME} $1\" bad arguments number." >&2
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

check_valid_command_name() {
  local IS_VALID_COMMAND=$(echo "$1" | grep -E '^[a-z]+$')

  if [ -z "${IS_VALID_COMMAND}" ]; then
    echo "'$1' is not valid command name." >&2
    exit 128
  fi
}

check_valid_program_name() {
  local IS_VALID_PROGRAM=$(echo "$1" | grep -E '^[a-z]+$')

  if [ -z "${IS_VALID_PROGRAM}" ]; then
    echo "'$1' is not valid application name." >&2
    exit 128
  fi
}

exec_command() {
  check_docker

  check_valid_command_name "${COMMAND}"

  local NB_ARGS=$#

  if [ -f "${BASEDIR}/scripts/command/command-${COMMAND}.sh" ]; then
    . "${BASEDIR}/scripts/command/command-${COMMAND}.sh"

    if ([ "${NB_ARGS}" -eq "${COMMAND_MIN_ARGS}" ] || [ "${NB_ARGS}" -gt "${COMMAND_MIN_ARGS}" ]) &&
       ([ "${NB_ARGS}" -eq "${COMMAND_MAX_ARGS}" ] || [ "${NB_ARGS}" -lt "${COMMAND_MAX_ARGS}" ] || [ -1 -eq "${COMMAND_MAX_ARGS}" ]); then
      # If no arg need
      if [ "${NB_ARGS}" -gt 0 ]; then
        local PROGRAM_NAME="$1"

        check_valid_program_name "${PROGRAM_NAME}"

        local COMMON_FILE="${BASEDIR}/program/${PROGRAM_NAME}.sh"

        shift
      fi

      "command_${COMMAND}" $@
    else
      error_command_missing_param "${COMMAND}"
      RETURN_CODE=1
    fi
  else
    error_command "${COMMAND}"
    RETURN_CODE=2
  fi
}

if [ $# -gt 0 ]; then
  COMMAND="$1"

  shift;

  case "${COMMAND}" in
    -h | --help    ) help;;
    -v | --version ) version;;
    *              ) exec_command "$@";;
  esac

else
  help
fi

exit ${RETURN_CODE}
