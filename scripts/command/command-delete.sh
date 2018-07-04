command_delete_help() {
  cat <<EOF

Usage:	${CURRENT_SCRIPT_NAME} delete PROGRAM

Delete an image for a program

Options:
  -a, --all                Build all image of program

EOF
}

command_delete_one() {
  echo "Deleting ${PROGRAM_NAME}..."

  . "${COMMON_FILE}"

  local NUMBER_IMAGE_EXISTS=$(docker image list ${APPLICATION_IMAGE_DOCKER} | wc -l)

  if [ "${NUMBER_IMAGE_EXISTS}" -gt 1 ]; then
    docker image rm ${APPLICATION_IMAGE_DOCKER}
  fi
}

command_delete_all() {
  for prog in $(ls program); do
    PROGRAM_NAME="${prog%.*}"
    COMMON_FILE="${BASEDIR}/program/${PROGRAM_NAME}.sh"

    command_delete_one
  done
}

command_delete() {
  case ${PROGRAM_NAME} in
    -h | --help    ) command_delete_help;;
    -a | --all     ) command_delete_all;;
    *              )
      local COMMON_FILE=$(get_common_file ${PROGRAM_NAME})

      if [ -n "${COMMON_FILE}" ] && [ -f "${COMMON_FILE}" ]; then
        command_delete_one
      elif [ -n "${COMMON_FILE}" ]; then
        echo "Program ${PROGRAM_NAME} not found. Check 'program' folder." >&2
        RETURN_CODE=3
      else
        RETURN_CODE=128
      fi;;
  esac
}

COMMAND_DESCRIPTION="Delete image"
COMMAND_MIN_ARGS=1
COMMAND_MAX_ARGS=1
