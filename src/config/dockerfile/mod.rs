///
/// Module that contain default dockerfile file.
///
/// Release under MIT License.
///

/// Default base from
pub const DOCKERFILE_DEFAULT_FROM: &'static str = "ubuntu:18.04";
/// Default tag base name
pub const DOCKERFILE_DEFAULT_TAG: &'static str = "d-base-image:v1.0.0";

/// Default docker file for base image
pub const DOCKERFILE_BASE_FILENAME: &str = "Dockerfile.hbs";
pub const DOCKERFILE_BASE: &str = r#"FROM {{dockerfile_from}}

{{#if dockerfile_base}}
    RUN apt-get update && \
        apt-get install -y \
          {{dependencies}}

    COPY entrypoint.sh /entrypoint.sh

    ENTRYPOINT ["/bin/sh", "/entrypoint.sh"]
{{else}}

{{#if (ends_width application_filename  ".deb")}}
    COPY {{application_filename}} /tmp/

    RUN apt-get update && \
        apt-get install -y \
          /tmp/{{application_filename}} && \
        rm -f /tmp/{{application_filename}} && \
        apt-get clean && \
        rm -rf /var/lib/apt/lists/*
{{else}}

{{#if (ends_width application_filename  ".tar.bz2")}}
    COPY {{application_filename}} /tmp/

    RUN tar -xjf /tmp/{{application_filename}} -C /opt/ && \
        rm -f /tmp/{{application_filename}}
{{else}}

{{#if (ends_width application_filename  ".tar.xz")}}
    COPY {{application_filename}} /tmp/

    RUN tar -xJf /tmp/{{application_filename}} -C /opt/ && \
        rm -f /tmp/{{application_filename}}
{{else}}

{{#if (or (ends_width application_filename  ".tar.gz") (ends_width application_filename  ".tgz"))}}
    COPY {{application_filename}} /tmp/

    RUN tar -xzf /tmp/{{application_filename}} -C /opt/ && \
        rm -f /tmp/{{application_filename}}
{{else}}

RUN apt-get update && \
    apt-get install -y {{application_filename}} && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

{{/if}}

{{/if}}

{{/if}}

{{/if}}

{{/if}}
"#;

/// Default entrypoint
pub const ENTRYPOINT_FILENAME: &str = "entrypoint.sh";
pub const ENTRYPOINT: &str = r#"#!/bin/sh
groupadd -g ${USERNAME_TO_RUN_GID} ${USERNAME_TO_RUN}
useradd -u ${USERNAME_TO_RUN_UID} -g ${USERNAME_TO_RUN_GID} ${USERNAME_TO_RUN}
mkdir -p /home/${USERNAME_TO_RUN}
chown -R ${USERNAME_TO_RUN}:${USERNAME_TO_RUN} /home/${USERNAME_TO_RUN}/
exec runuser -u ${USERNAME_TO_RUN} -- "$@""#;
