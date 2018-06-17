#!/bin/sh

groupadd -g ${USERNAME_TO_RUN_GID} ${USERNAME_TO_RUN}
useradd -u ${USERNAME_TO_RUN_UID} -g ${USERNAME_TO_RUN_GID} ${USERNAME_TO_RUN}

mkdir -p /home/${USERNAME_TO_RUN}
chown -R ${USERNAME_TO_RUN}:${USERNAME_TO_RUN} /home/${USERNAME_TO_RUN}/

exec runuser -u ${USERNAME_TO_RUN} -- "$@"
