DOCKERFILE="${BASEDIR}/scripts/Dockerfile"
DOCKERFILE_BASE="${BASEDIR}/scripts/Dockerfile.base"

command_build_get_base_image_version() {
  # Get name and version from application Dockerfile
  cat "${DOCKERFILE}" | grep 'FROM' | sed -r 's/^FROM\s(.+)/\1/'
}

command_build_help() {
  cat <<EOF

Usage:	${CURRENT_SCRIPT_NAME} build PROGRAM | OPTIONS

Build an image for a program

Options:
  -a, --all                Build all image of program
  -b, --base               Build base image (TODO)
EOF
}

command_build() {
  echo "Building ${PROGRAM_NAME}..."

  BASE_IMAGE_DOCKER=$(command_build_get_base_image_version)

  # Check if image exists
  NUMBER_IMAGE_EXISTS=$(docker image list ${BASE_IMAGE_DOCKER} | wc -l)

  if [ "${NUMBER_IMAGE_EXISTS}" -lt 2 ]; then
    command_build_base
  fi

  . "${COMMON_FILE}"

  #DOWNLOADED_FILE_NAME_DEST="${BASEDIR}/download/${DOWNLOADED_FILE_NAME}"
  DOWNLOADED_FILE_NAME_DEST="./download/${APPLICATION_DOWNLOADED_FILE_NAME}"

  mkdir -p download

  # Get date when file downloaded
  LAST_DOWNLOAD_CONTENT_FILE="$(date -r "${DOWNLOADED_FILE_NAME_DEST}" -R -u 2>/dev/null)"

  if [ -f "${DOWNLOADED_FILE_NAME_DEST}" ] && [ -n "${LAST_DOWNLOAD_CONTENT_FILE}" ]; then
    # Download file only if no updated
    curl -o "${DOWNLOADED_FILE_NAME_DEST}" -z "${DOWNLOADED_FILE_NAME_DEST}" -L "${APPLICATION_URL}"
  else
    curl -o "${DOWNLOADED_FILE_NAME_DEST}" -L "${APPLICATION_URL}"
  fi

  docker build \
    --build-arg "APPLICATION_DOWNLOADED_FILE_NAME=${APPLICATION_DOWNLOADED_FILE_NAME}" \
    --build-arg "DOWNLOADED_FILE_NAME_DEST=${DOWNLOADED_FILE_NAME_DEST}" \
    . -f "${DOCKERFILE}" -t ${APPLICATION_IMAGE_DOCKER}

  RETURN_CODE=$?
}

command_build_all() {
  for prog in $(ls program); do
    PROGRAM_NAME="${prog%.*}"
    COMMON_FILE="${BASEDIR}/program/${PROGRAM_NAME}.sh"

    command_build
  done
}

command_build_base() {
  echo "Building base image..."

  DEPENDENCIES_ALL=""

  for prog in $(ls program); do
    programmName="${prog%.*}"
    commonFile="${BASEDIR}/program/${programmName}.sh"

    . "${commonFile}"

    DEPENDENCIES_ALL="${DEPENDENCIES_ALL} ${APPLICATION_DEPENDENCIES}"
  done

  BASE_IMAGE_DOCKER=$(command_build_get_base_image_version)

  docker build \
    --build-arg "DEPENDENCIES_ALL=${DEPENDENCIES_ALL}" \
    . -f "${DOCKERFILE_BASE}" -t ${BASE_IMAGE_DOCKER}

  RETURN_CODE=$?
}

case ${PROGRAM_NAME} in
  -h | --help    ) command_build_help;;
  -a | --all     ) command_build_all;;
  -b | --base    ) command_build_base;;
  *              )
    if [ -f "${COMMON_FILE}" ]; then
      command_build
    else
      echo "Program ${PROGRAM_NAME} not found. Check 'program' folder." >&2
      RETURN_CODE=3
    fi;;
esac
