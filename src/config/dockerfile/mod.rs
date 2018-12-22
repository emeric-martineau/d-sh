///
/// Module that contain default dockerfile file.
///
/// Release under MIT License.
///

/// Default docker file for base image
pub const DOCKERFILE_BASE_FILENAME: &str = "Dockerfile.base";
pub const DOCKERFILE_BASE: &str = r#"FROM ubuntu:18.04

ARG DEPENDENCIES_ALL

RUN apt-get update && \
    apt-get install -y \
      $DEPENDENCIES_ALL

COPY scripts/entrypoint.sh /entrypoint.sh

ENTRYPOINT ["/bin/sh", "/entrypoint.sh"]"#;

/// Default docker file for debian file
pub const DOCKERFILE_DEB_FILENAME: &str = "Dockerfile.from-deb-file";
pub const DOCKERFILE_DEB: &str = r#"#
# This Dockerfile is used when install a .deb file
#
FROM d-base-image:v1.0.0

ARG APPLICATION_DOWNLOADED_FILE_NAME
ARG DOWNLOADED_FILE_NAME_DEST

COPY $DOWNLOADED_FILE_NAME_DEST /tmp/

RUN apt-get update && \
    apt-get install -y \
      /tmp/$APPLICATION_DOWNLOADED_FILE_NAME && \
    rm -f /tmp/$APPLICATION_DOWNLOADED_FILE_NAME && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*"#;

/// Default docker file for package file
pub const DOCKERFILE_PACKAGE_FILENAME: &str = "Dockerfile.from-pacakge-file";
pub const DOCKERFILE_PACKAGE: &str = r#"#
# This Dockerfile is used when install a standard package of Linux distribution
#
FROM d-base-image:v1.0.0

ARG APPLICATION_DOWNLOADED_FILE_NAME

RUN apt-get update && \
    apt-get install -y $APPLICATION_DOWNLOADED_FILE_NAME && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*"#;

/// Default docker file for tgz file
pub const DOCKERFILE_TGZ_FILENAME: &str = "Dockerfile.from-tgz-file";
pub const DOCKERFILE_TGZ: &str = r#"#
# This Dockerfile is used when install a .tgz or tar.gz or tar.bz2 file
#
FROM d-base-image:v1.0.0

ARG APPLICATION_DOWNLOADED_FILE_NAME
ARG DOWNLOADED_FILE_NAME_DEST
ARG COMMAND_OPTIONS

COPY $DOWNLOADED_FILE_NAME_DEST /tmp/

RUN tar $COMMAND_OPTIONS /tmp/$APPLICATION_DOWNLOADED_FILE_NAME -C /opt/ && \
    rm -f /tmp/$APPLICATION_DOWNLOADED_FILE_NAME"#;

/// Default entrypoint
pub const ENTRYPOINT_FILENAME: &str = "entrypoint.sh";
pub const ENTRYPOINT: &str = r#"#!/bin/sh

groupadd -g ${USERNAME_TO_RUN_GID} ${USERNAME_TO_RUN}
useradd -u ${USERNAME_TO_RUN_UID} -g ${USERNAME_TO_RUN_GID} ${USERNAME_TO_RUN}

mkdir -p /home/${USERNAME_TO_RUN}
chown -R ${USERNAME_TO_RUN}:${USERNAME_TO_RUN} /home/${USERNAME_TO_RUN}/

exec runuser -u ${USERNAME_TO_RUN} -- "$@""#;
