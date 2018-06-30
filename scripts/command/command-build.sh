local DOCKERFILE_TAR_GZ="${BASEDIR}/scripts/Dockerfile.from-tgz-file"
local DOCKERFILE_DEB="${BASEDIR}/scripts/Dockerfile.from-deb-file"
local DOCKERFILE_PACKAGE="${BASEDIR}/scripts/Dockerfile.from-package"
local DOCKERFILE_BASE="${BASEDIR}/scripts/Dockerfile.base"
local COMMAND_OPTIONS=""

command_build_get_base_image_version() {
  # Get name and version from application Dockerfile
  cat "$1" | grep 'FROM' | sed -r 's/^FROM\s(.+)/\1/'
}

command_build_get_dockerfile() {
  case "$1" in
    *.tar.bz2) echo "${DOCKERFILE_TAR_GZ}";;
    # *.bz2)     bunzip_only ;;
    *.tar.gz)  echo "${DOCKERFILE_TAR_GZ}";;
    *.tgz)     echo "${DOCKERFILE_TAR_GZ}";;
    # *.gz)      gunzip_only ;;
    # *.zip)     unzip ;;
    # *.7z)      do something ;;
    *.deb)     echo "${DOCKERFILE_DEB}";;
    *)         echo "${DOCKERFILE_PACKAGE}";;
  esac
}

command_build_get_command_options() {
  case "$1" in
    *.tar.bz2) echo "-xjf";;
    # *.bz2)     bunzip_only ;;
    *.tar.gz)  echo "-xzf";;
    *.tgz)     echo "-xzf";;
    # *.gz)      gunzip_only ;;
    # *.zip)     unzip ;;
    # *.7z)      do something ;;
  esac
}

command_build_download() {
  local DOWNLOADED_FILE_NAME_DEST="$1"

  mkdir -p download

  # Get date when file downloaded
  local LAST_DOWNLOAD_CONTENT_FILE="$(date -r "$1" -R -u 2>/dev/null)"

  if [ -f "${DOWNLOADED_FILE_NAME_DEST}" ] && [ -n "${LAST_DOWNLOAD_CONTENT_FILE}" ]; then
    # Download file only if no updated
    curl -o "${DOWNLOADED_FILE_NAME_DEST}" -z "${DOWNLOADED_FILE_NAME_DEST}" -L "${APPLICATION_URL}"
  else
    curl -o "${DOWNLOADED_FILE_NAME_DEST}" -L "${APPLICATION_URL}"
  fi
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

command_build_one() {
  echo "Building ${PROGRAM_NAME}..."

  . "${COMMON_FILE}"

  local DOCKERFILE=$(command_build_get_dockerfile ${APPLICATION_DOWNLOADED_FILE_NAME})
  local COMMAND_OPTIONS=$(command_build_get_command_options ${APPLICATION_DOWNLOADED_FILE_NAME})

  local BASE_IMAGE_DOCKER=$(command_build_get_base_image_version "${DOCKERFILE}")

  # Check if image exists
  local NUMBER_IMAGE_EXISTS=$(docker image list ${BASE_IMAGE_DOCKER} | wc -l)

  if [ "${NUMBER_IMAGE_EXISTS}" -lt 2 ]; then
    command_build_base
  fi

  if [ -n "${APPLICATION_URL}" ]; then
    #DOWNLOADED_FILE_NAME_DEST="${BASEDIR}/download/${DOWNLOADED_FILE_NAME}"
    local DOWNLOADED_FILE_NAME_DEST="./download/${APPLICATION_DOWNLOADED_FILE_NAME}"

    command_build_download "${DOWNLOADED_FILE_NAME_DEST}"
  fi

  local BUILD_OPTS=""

  if [ -n "${COMMAND_OPTIONS}" ]; then
    BUILD_OPTS=" --build-arg COMMAND_OPTIONS=${COMMAND_OPTIONS}"
  fi

  if [ -n "${DOWNLOADED_FILE_NAME_DEST}" ]; then
    BUILD_OPTS="${BUILD_OPTS} --build-arg DOWNLOADED_FILE_NAME_DEST=${DOWNLOADED_FILE_NAME_DEST}"
  fi

  docker build \
    --build-arg "APPLICATION_DOWNLOADED_FILE_NAME=${APPLICATION_DOWNLOADED_FILE_NAME}" \
    ${BUILD_OPTS} \
    . -f "${DOCKERFILE}" -t ${APPLICATION_IMAGE_DOCKER}

  RETURN_CODE=$?
}

command_build_all() {
  for prog in $(ls program); do
    local PROGRAM_NAME="${prog%.*}"
    local COMMON_FILE="${BASEDIR}/program/${PROGRAM_NAME}.sh"

    command_build_one
  done
}

command_build_base() {
  echo "Building base image..."

  local DEPENDENCIES_ALL=""

  for prog in $(ls program); do
    programmName="${prog%.*}"
    commonFile="${BASEDIR}/program/${programmName}.sh"

    . "${commonFile}"

    local DEPENDENCIES_ALL="${DEPENDENCIES_ALL} ${APPLICATION_DEPENDENCIES}"
  done

  local BASE_IMAGE_DOCKER=$(command_build_get_base_image_version)

  docker build \
    --build-arg "DEPENDENCIES_ALL=${DEPENDENCIES_ALL}" \
    . -f "${DOCKERFILE_BASE}" -t ${BASE_IMAGE_DOCKER}

  RETURN_CODE=$?
}

# Main function
command_build() {
  case ${PROGRAM_NAME} in
    -h | --help    ) command_build_help;;
    -a | --all     ) command_build_all;;
    -b | --base    ) command_build_base;;
    *              )
      if [ -f "${COMMON_FILE}" ]; then
        command_build_one
      else
        echo "Program ${PROGRAM_NAME} not found. Check 'program' folder." >&2
        RETURN_CODE=3
      fi;;
  esac
}

COMMAND_DESCRIPTION="Build container image"
COMMAND_MIN_ARGS=1
COMMAND_MAX_ARGS=1
