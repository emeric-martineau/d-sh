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
    *.tar.bz2 ) echo "${DOCKERFILE_TAR_GZ}";;
    # *.bz2)     bunzip_only ;;
    *.tar.gz  ) echo "${DOCKERFILE_TAR_GZ}";;
    *.tgz     ) echo "${DOCKERFILE_TAR_GZ}";;
    # *.gz)      gunzip_only ;;
    # *.zip)     unzip ;;
    # *.7z)      do something ;;
    *.tar.xz  ) echo "${DOCKERFILE_TAR_GZ}";;
    *.deb     ) echo "${DOCKERFILE_DEB}";;
    *         ) echo "${DOCKERFILE_PACKAGE}";;
  esac
}

command_build_get_command_options() {
  case "$1" in
    *.tar.bz2 ) echo "-xjf";;
    # *.bz2)     bunzip_only ;;
    *.tar.gz  ) echo "-xzf";;
    *.tgz     ) echo "-xzf";;
    *.tar.xz  ) echo "-xJf";;
    # *.gz)      gunzip_only ;;
    # *.zip)     unzip ;;
    # *.7z)      do something ;;
  esac
}

# $1 : name of download file
# $2 : if skip redownload
command_build_download() {
  local DOWNLOADED_FILE_NAME_DEST="$1"
  local BUILD_SKIP_REDOWNLOAD="$2"

  # If file exists and we don't want redownload it
  if [ -f "${BASEDIR}/${DOWNLOADED_FILE_NAME_DEST}" ] && [ "${BUILD_SKIP_REDOWNLOAD}" = "true" ]; then
    echo "Skip downloading ${DOWNLOADED_FILE_NAME_DEST}"
    return
  fi

  mkdir -p "${BASEDIR}/download"

  # Get date when file downloaded
  local LAST_DOWNLOAD_CONTENT_FILE="$(date -r "${BASEDIR}/${DOWNLOADED_FILE_NAME_DEST}" -R -u 2>/dev/null)"

  echo "Downloading file to ${BASEDIR}/${DOWNLOADED_FILE_NAME_DEST}"

  if [ -f "${BASEDIR}/${DOWNLOADED_FILE_NAME_DEST}" ] && [ -n "${LAST_DOWNLOAD_CONTENT_FILE}" ]; then
    # Download file only if no updated
    curl -o "${BASEDIR}/${DOWNLOADED_FILE_NAME_DEST}" -z "${BASEDIR}/${DOWNLOADED_FILE_NAME_DEST}" -L "${APPLICATION_URL}"
  else
    curl -o "${BASEDIR}/${DOWNLOADED_FILE_NAME_DEST}" -L "${APPLICATION_URL}"
  fi
}

# $1 : if delete
# $2 : name of image to delete
command_build_remove_image() {
  if [ "$1" = "true" ]; then
    local NUMBER_IMAGE_EXISTS=$(docker image list "$2" | wc -l)

    if [ "${NUMBER_IMAGE_EXISTS}" -gt 1 ]; then
      docker image rm -f "$2"
    fi
  fi
}

command_build_help() {
  cat <<EOF

Usage:	${CURRENT_SCRIPT_NAME} build [OPTIONS] PROGRAM1 PROGRAM2 ...

Build an image for a program

Options:
  -a, --all                Build all image of program
  -b, --base               Build base image
  -f  --force              Remove existing image before build
  -m  --missing            Build only missing image
  -s  --skip-redownload    If binary is present, don't check if new version is available
EOF
}

# $1 : force build
# $2 : if skip redownload
command_build_one() {
  local BUILD_FORCE="$1"
  local BUILD_SKIP_REDOWNLOAD="$2"

  echo "Building ${PROGRAM_NAME}..."

  . "${COMMON_FILE}"

  local DOCKERFILE=$(command_build_get_dockerfile ${APPLICATION_DOWNLOADED_FILE_NAME})
  local COMMAND_OPTIONS=$(command_build_get_command_options ${APPLICATION_DOWNLOADED_FILE_NAME})

  local BASE_IMAGE_DOCKER=$(command_build_get_base_image_version "${DOCKERFILE}")

  # Check if image exists
  local NUMBER_IMAGE_EXISTS=$(docker image list ${BASE_IMAGE_DOCKER} | wc -l)

  if [ "${NUMBER_IMAGE_EXISTS}" -lt 2 ]; then
    command_build_base "${BUILD_FORCE}"
  fi

  command_build_remove_image "${BUILD_FORCE}" "${APPLICATION_IMAGE_DOCKER}"

  if [ -n "${APPLICATION_URL}" ]; then
    #DOWNLOADED_FILE_NAME_DEST="${BASEDIR}/download/${DOWNLOADED_FILE_NAME}"
    local DOWNLOADED_FILE_NAME_DEST="download/${APPLICATION_DOWNLOADED_FILE_NAME}"

    # If in config application we don't want check redownload
    if [ "${APPLICATION_SKIP_CHECK_REDOWNLOAD}" = "true" ]; then
      BUILD_SKIP_REDOWNLOAD="true"
    fi

    command_build_download "${DOWNLOADED_FILE_NAME_DEST}" "${BUILD_SKIP_REDOWNLOAD}"
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
    "${BASEDIR}" -f "${DOCKERFILE}" -t "${APPLICATION_IMAGE_DOCKER}"

  RETURN_CODE=$?
}

# $1 : force build
# $2 : if skip redownload
command_build_all() {
  local BUILD_FORCE="$1"
  local BUILD_SKIP_REDOWNLOAD="$2"

  for prog in $(ls program); do
    local PROGRAM_NAME="${prog%.*}"
    local COMMON_FILE="${BASEDIR}/program/${PROGRAM_NAME}.sh"

    command_build_one "${BUILD_FORCE}" "${BUILD_SKIP_REDOWNLOAD}"
  done
}

# $1 : force build
command_build_base() {
  echo "Building base image..."

  local DEPENDENCIES_ALL=""

  for prog in $(ls "${BASEDIR}/program"); do
    programmName="${prog%.*}"
    commonFile="${BASEDIR}/program/${programmName}.sh"

    . "${commonFile}"

    local DEPENDENCIES_ALL="${DEPENDENCIES_ALL} ${APPLICATION_DEPENDENCIES}"
  done

  local BASE_IMAGE_DOCKER=$(command_build_get_base_image_version ${DOCKERFILE_DEB})

  command_build_remove_image "$1" "${BASE_IMAGE_DOCKER}"

  docker build \
    --build-arg "DEPENDENCIES_ALL=${DEPENDENCIES_ALL}" \
    ${BASEDIR} -f "${DOCKERFILE_BASE}" -t ${BASE_IMAGE_DOCKER}

  RETURN_CODE=$?
}

# $1 : if skip redownload
command_build_missing() {
  local BUILD_SKIP_REDOWNLOAD="$1"

  for prog in $(ls "${BASEDIR}/program"); do
    local PROGRAM_NAME="${prog%.*}"

    local COMMON_FILE=$(get_common_file ${PROGRAM_NAME})

    . "${COMMON_FILE}"

    local NUMBER_IMAGE_EXISTS=$(docker image list ${APPLICATION_IMAGE_DOCKER} | wc -l)

    if [ "${NUMBER_IMAGE_EXISTS}" -lt 2 ]; then
      command_build_one "false" "${BUILD_SKIP_REDOWNLOAD}"
    fi
  done
}

# Main function
command_build() {
  local BUILD_BASE="false"
  local BUILD_ALL="false"
  local BUILD_FORCE="false"
  local BUILD_MISSING="false"
  local BUILD_SKIP_REDOWNLOAD="false"
  local LIST_BUILD_PROGRAM=""

  for cmd in "${PROGRAM_NAME}" $@; do
    case ${cmd} in
      -h | --help            ) command_build_help; return;;
      -a | --all             ) BUILD_ALL="true";;
      -b | --base            ) BUILD_BASE="true";;
      -f | --force           ) BUILD_FORCE="true";;
      -m | --missing         ) BUILD_MISSING="true";;
      -s | --skip-redownload ) BUILD_SKIP_REDOWNLOAD="true";;
      *                      )
        LIST_BUILD_PROGRAM="${LIST_BUILD_PROGRAM} ${cmd}"
    esac
  done

  if [ "${BUILD_BASE}" = "true" ]; then
    command_build_base "${BUILD_FORCE}"
  fi

  if [ "${BUILD_ALL}" = "true" ]; then
    command_build_all "${BUILD_FORCE}" "${BUILD_SKIP_REDOWNLOAD}"
  elif [ "${BUILD_MISSING}" = "true" ]; then
    command_build_missing "${BUILD_SKIP_REDOWNLOAD}"
  fi

  for PROGRAM_NAME in ${LIST_BUILD_PROGRAM}; do
    local COMMON_FILE=$(get_common_file ${PROGRAM_NAME})

    if [ -n "${COMMON_FILE}" ] && [ -f "${COMMON_FILE}" ]; then
      command_build_one "${BUILD_FORCE}" "${BUILD_SKIP_REDOWNLOAD}"
    elif [ -n "${COMMON_FILE}" ]; then
      echo "Program ${PROGRAM_NAME} not found. Check 'program' folder." >&2

      RETURN_CODE=3

      return
    else
      RETURN_CODE=128

      return
    fi
  done
}

COMMAND_DESCRIPTION="Build container image"
COMMAND_MIN_ARGS=1
COMMAND_MAX_ARGS=-1
